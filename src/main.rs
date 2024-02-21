mod common;
mod conversion;
mod formats;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, Subcommand};
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use std::{
    fs,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::formats::*;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    // Commands for working with the archive format
    Pack {
        /// Output file, default is <folder>.dat
        #[arg(short, long)]
        output: Option<String>,

        /// Folder to pack, e.g. common
        folder: String,
    },
    Unpack {
        /// Output folder, default is <file> without ".dat"
        #[arg(short, long)]
        output: Option<String>,

        /// File to unpack, e.g. common.dat
        file: String,
    },

    // Commands for converting extracted files
    Convert {
        /// Output folder, default is current dir
        #[arg(short, long)]
        output: Option<String>,

        /// Extracted root directory, set to copy assets automatically if it's somewhere else
        #[arg(short, long)]
        root: Option<String>,

        /// File to conert, e.g. man.box
        file: String,
    },
}

// MDL files are not mapped yet since the name/extension is unknown
fn rsrc_type_mapper(peth: &Path) -> Option<u8> {
    // first try to match whole name
    match peth.file_name().unwrap().to_str().unwrap() {
        // these dont use special extensions and they only appear once,
        // so i guess this is the way to do it?
        "path.dat" => Some(4),
        "animate.dat" => Some(5),
        "carproperty.dat" => Some(6),
        // then try to match extension
        _ => match peth
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "box" => Some(0),
            "obj" => Some(1),
            "map" => Some(2),
            "hmp" => Some(3),
            // there might be other texture formats but these are known
            "png" | "jpg" => Some(10),
            // unsupported input file
            _ => None,
        },
    }
}

fn pack(folder: &String, output: Option<String>) {
    // check if we can use input directory
    let folder_path = Path::new(folder);
    if !folder_path.is_dir() {
        eprintln!(
            "Input directory {:?} does not exist, exiting...",
            folder_path
        );
        return;
    }

    // figure out output file
    let out_file = match output {
        Some(v) => PathBuf::from(v),
        None => {
            let file_stem = folder_path.file_stem().unwrap();
            PathBuf::from(file_stem)
        }
    };
    // check if we can use it
    if out_file.try_exists().unwrap() {
        eprintln!("Output file {:?} already exists, exiting...", out_file);
        return;
    }

    // get all files in folder
    let mut files = vec![];
    for x in WalkDir::new(folder_path) {
        let x = x.unwrap();
        // ignore anything other than files
        if !x.file_type().is_file() {
            continue;
        }

        let peth = x.path();
        // skip files without any compatible resourcetype
        let mapped = rsrc_type_mapper(peth);
        if mapped.is_none() {
            eprintln!("Skipping unsupported file {:?}", peth);
            continue;
        }

        // push file info
        files.push((
            peth.strip_prefix(folder_path)
                .unwrap()
                .to_str()
                .unwrap()
                .replace("\\", "/"),
            peth.to_owned(),
            mapped.unwrap(),
        ));
    }

    // exit if no files
    if files.len() < 1 {
        eprintln!("No files found to pack, exiting...");
        return;
    }

    // sort files
    // story time: the game is kinda badly made and always assumes the textures are already loaded.
    // this means textures needs to be defined BEFORE any objects, maps or boxes.
    // so this is a bandaid fix to that problem since i dont have control over which order
    // the walkdir function gets files.
    // thankfully the id works great for this :).
    files.sort_by(|a, b| b.2.cmp(&a.2));

    // store header in memory buffer
    let mut head = Cursor::new(vec![]);

    // write all file names and types
    head.write_i32::<LittleEndian>(files.len() as i32).unwrap();
    for x in &files {
        // filename
        head.write_u8(x.0.len() as u8).unwrap();
        head.write_all(x.0.as_bytes()).unwrap();
        // type
        head.write_u8(x.2).unwrap();
    }
    head.flush().unwrap();

    // then compress and write result
    let f = fs::File::create(out_file).unwrap();
    let mut comp = DeflateEncoder::new(f, Compression::best());
    comp.write_all(&head.into_inner()).unwrap();

    let mut in_buf = vec![];
    for x in &files {
        // read file into memory
        let mut in_f = fs::File::open(&x.1).unwrap();
        let readed = in_f.read_to_end(&mut in_buf).unwrap();
        // write length and data
        comp.write_i32::<LittleEndian>(readed as i32).unwrap();
        comp.write_all(&in_buf[0..readed]).unwrap();
        in_buf.clear();
    }
    comp.flush().unwrap();
}

fn unpack(file: &String, output: Option<String>) {
    // check if we can use input file
    let file_path = Path::new(file);
    if !file_path.is_file() {
        eprintln!("Input file {:?} does not exist, exiting...", file_path);
        return;
    }

    let f = fs::File::open(file).unwrap();
    let mut rdr = DeflateDecoder::new(f);

    // figure out output directory
    let out_dir = match output {
        Some(v) => PathBuf::from(v),
        None => {
            let file_stem = Path::new(file).file_stem().unwrap();
            PathBuf::from(file_stem)
        }
    };
    // check if we can use it
    if out_dir.try_exists().unwrap() {
        eprintln!("{:?} already exists, exiting...", out_dir);
        return;
    }

    // get all objects
    let mut objects = vec![];
    let objects_n = rdr.read_i32::<LittleEndian>().unwrap();
    for _ in 0..objects_n {
        let len = rdr.read_u8().unwrap();
        let filename = common::read_string(&mut rdr, len).unwrap();
        let type_ = rdr.read_u8().unwrap(); // type
        objects.push((filename, type_));
    }

    // output em
    for o in objects {
        let len = rdr.read_i32::<LittleEndian>().unwrap();
        println!("\"{}\", length: {}, type: {}", o.0, len, o.1);

        let mut buffer = vec![0u8; len as usize];
        rdr.read_exact(&mut buffer).unwrap();

        let out_path = out_dir.join(&o.0);
        fs::create_dir_all(out_path.parent().unwrap()).unwrap();

        let mut out = fs::File::create(&out_path).unwrap();
        out.write_all(&buffer).unwrap();
    }
}

fn convert(input: String, root: Option<String>, output: Option<String>) {
    // check if we can use input file
    let input_path = Path::new(&input);
    if !input_path.is_file() {
        eprintln!("Input file {:?} does not exist, exiting...", input_path);
        return;
    }

    // check if specified root exists
    if root.is_some() {
        let h = root.as_ref().unwrap();
        let root_path = Path::new(&h);
        if !root_path.is_dir() {
            eprintln!("Root directory {:?} does not exist, exiting...", root_path);
            return;
        }
    }

    // figure out output dir
    let out_dir = match output {
        Some(ref v) => Path::new(v),
        None => {
            let a = Path::new("_Converted");
            if !a.try_exists().unwrap() {
                fs::create_dir(a).unwrap();
            }
            a
        },
    };
    // technically we do it double but eh whatever
    if !out_dir.is_dir() {
        eprintln!("Output directory {:?} does not exist, exiting...", out_dir);
        return;
    }

    // open file
    let input_file = fs::File::open(input_path).unwrap();

    // magik
    match input_path.extension().unwrap().to_str().unwrap() {
        "box" => r#box::box_to_obj(input_file, input_path, out_dir, root),
        "obj" => obj::obj_to_obj(input_file, input_path, out_dir, root),
        "map" => map::map_to_obj(input_file, input_path, out_dir, root),
        _ => ()
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CliCommands::Pack { output, folder } => {
            pack(&folder, output);
        }
        CliCommands::Unpack { output, file } => {
            unpack(&file, output);
        }
        CliCommands::Convert { output, root, file } => {
            convert(file, root, output);
        }
    }
}
