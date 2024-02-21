use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{common, conversion::obj};

pub fn box_to_obj<R: Read + Seek>(mut f: R, input: &Path, output: &Path, root: Option<String>) {
    //
    // ReadHeader
    //
    let magic = f.read_i32::<LittleEndian>().unwrap();
    let version = f.read_i32::<LittleEndian>().unwrap();

    // not present in original code
    if magic != 1112496128 || version != common::FORMAT_VERSION {
        eprintln!("Invalid magic or version, exiting...");
        return;
    }

    let lump_num = f.read_i32::<LittleEndian>().unwrap();
    f.read_i32::<LittleEndian>().unwrap(); // unused, lump_off
    let frame_num = f.read_i32::<LittleEndian>().unwrap();
    f.read_i32::<LittleEndian>().unwrap(); // unused, box_off

    //
    // Readlumps
    //
    for i in 0..lump_num {
        let pos = f.read_i32::<LittleEndian>().unwrap();
        let vtx_num = f.read_i32::<LittleEndian>().unwrap();
        let tri_num = f.read_i32::<LittleEndian>().unwrap();
        let uvs_num = f.read_i32::<LittleEndian>().unwrap();

        // due to this string being static size i need to trim excess NULLs at the end
        let tex_name = common::read_string(&mut f, 100)
            .unwrap()
            .trim_end_matches('\0')
            .to_owned();

        //
        // ReadpolyMesh
        //
        f.seek(SeekFrom::Start(pos as u64)).unwrap();

        let mut uvs_buf = vec![];
        for _ in 0..uvs_num {
            uvs_buf.push((
                f.read_f32::<LittleEndian>().unwrap(),
                1.0 - f.read_f32::<LittleEndian>().unwrap(),
            ));
        }

        let mut tri_buf = vec![];
        for _ in 0..tri_num {
            tri_buf.push(common::Tri {
                a: f.read_i32::<LittleEndian>().unwrap(),
                ta: f.read_i32::<LittleEndian>().unwrap(),
                b: f.read_i32::<LittleEndian>().unwrap(),
                tb: f.read_i32::<LittleEndian>().unwrap(),
                c: f.read_i32::<LittleEndian>().unwrap(),
                tc: f.read_i32::<LittleEndian>().unwrap(),
            });
        }

        let mut frame_buf = vec![];
        for _ in 0..frame_num {
            let mut vtx_buf = vec![];
            for _ in 0..vtx_num {
                vtx_buf.push((
                    f.read_f32::<LittleEndian>().unwrap(),
                    f.read_f32::<LittleEndian>().unwrap(),
                    f.read_f32::<LittleEndian>().unwrap(),
                ));
            }
            frame_buf.push(vtx_buf);
        }

        // logging stuff
        let pfx = if lump_num > 1 {
            format!("[{}/{}] ", i + 1, lump_num)
        } else {
            String::new()
        };

        // figure out file path stuff
        let in_base = input.file_name().unwrap().to_str().unwrap();

        let obj_f_name = {
            if lump_num > 1 {
                format!("{}.{}.obj", in_base, i)
            } else {
                format!("{}.obj", in_base)
            }
        };
        let mtl_f_name = {
            if lump_num > 1 {
                format!("{}.{}.mtl", in_base, i)
            } else {
                format!("{}.mtl", in_base)
            }
        };

        // check if outputs already exist
        let obj_f_path = output.join(obj_f_name);
        if obj_f_path.try_exists().unwrap() {
            eprintln!("Output file {:?} already exists, exiting...", obj_f_path);
            return;
        }
        let mtl_f_path = output.join(&mtl_f_name);
        if mtl_f_path.try_exists().unwrap() {
            eprintln!("Output file {:?} already exists, exiting...", mtl_f_path);
            return;
        }

        // first write object
        println!("{}Writing object (.obj) to {:?}...", pfx, obj_f_path);
        let mut obj_f = fs::File::create(obj_f_path).unwrap();
        obj::write_obj(&mut obj_f, &mtl_f_name, &frame_buf[0], &uvs_buf, &tri_buf).unwrap();

        // then write material
        println!("{}Writing material (.mtl) to {:?}...", pfx, mtl_f_path);
        let mut mtl_f = fs::File::create(mtl_f_path).unwrap();
        obj::write_mtl(&mut mtl_f, &tex_name).unwrap();

        // check if we can auto copy texture
        let tex_base = Path::new(&tex_name).file_name().unwrap().to_str().unwrap();
        let tex_path = match root {
            Some(ref v) => Path::new(v).join(&tex_name),
            None => PathBuf::from(&tex_name),
        };
        if tex_path.is_file() {
            // check if we already did
            let dest = output.join(&tex_base);
            if dest.try_exists().unwrap() && dest.is_file() {
                println!("{}Skipping copying texture, already exists", pfx);
            } else {
                println!("{}Copying texture to {:?}...", pfx, dest);
                fs::copy(tex_path, dest).unwrap();
            }
        } else {
            // otherwise texture is manual because we don't know where it is
            println!(
                "{}Couldn't find texture, copy texture from \"{}\" to view with textures",
                pfx, tex_name
            );
        }
    }
}
