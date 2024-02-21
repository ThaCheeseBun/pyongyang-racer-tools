use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{common, conversion::obj};

pub fn obj_to_obj<R: Read + Seek>(mut f: R, input: &Path, output: &Path, root: Option<String>) {
    //
    // ReadHeader
    //
    let magic = f.read_i32::<LittleEndian>().unwrap();
    let version = f.read_i32::<LittleEndian>().unwrap();

    if magic != 1245859584 || version != 16777216 {
        eprintln!("Invalid magic or version, exiting...");
        return;
    }

    let geom_off = f.read_i32::<LittleEndian>().unwrap();
    let vtx_num = f.read_i32::<LittleEndian>().unwrap();
    let uv_num = f.read_i32::<LittleEndian>().unwrap();
    let tri_num = f.read_i32::<LittleEndian>().unwrap();

    // unused for conversion atm
    let _max = (
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
    );
    let _min = (
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
    );

    let mat_num = f.read_i32::<LittleEndian>().unwrap();
    let mat_off = f.read_i32::<LittleEndian>().unwrap();
    let mut mat_off_cur = mat_off as u64;

    //
    // ReadGeomData
    //
    f.seek(SeekFrom::Start(geom_off as u64)).unwrap();

    let mut vtx_buf = vec![];
    for _ in 0..vtx_num {
        vtx_buf.push((
            f.read_f32::<LittleEndian>().unwrap(),
            f.read_f32::<LittleEndian>().unwrap(),
            f.read_f32::<LittleEndian>().unwrap(),
        ));
    }

    let mut tri_buf = vec![];
    for _ in 0..tri_num {
        tri_buf.push((
            f.read_u16::<LittleEndian>().unwrap(),
            f.read_u16::<LittleEndian>().unwrap(),
            f.read_u16::<LittleEndian>().unwrap(),
        ));
    }

    let mut uv_buf = vec![];
    for _ in 0..uv_num {
        uv_buf.push((
            f.read_f32::<LittleEndian>().unwrap(),
            1.0 - f.read_f32::<LittleEndian>().unwrap(),
        ));
    }

    //
    // ReadMaterials
    //
    for i in 0..mat_num {
        f.seek(SeekFrom::Start(mat_off_cur)).unwrap();

        let poly_off = f.read_i32::<LittleEndian>().unwrap();
        let poly_num = f.read_i32::<LittleEndian>().unwrap();
        f.read_i32::<LittleEndian>().unwrap(); // unknown and unused

        // due to this string being static size i need to trim excess NULLs at the end
        let tex_name = common::read_string(&mut f, 100)
            .unwrap()
            .trim_end_matches('\0')
            .to_owned();

        // i guess we just throw these away?
        f.read_u16::<LittleEndian>().unwrap();
        f.read_u16::<LittleEndian>().unwrap();

        mat_off_cur = f.seek(SeekFrom::Current(0)).unwrap();

        //
        // ReadPolyData
        //
        f.seek(SeekFrom::Start(poly_off as u64)).unwrap();

        let mut thing_buf = vec![];
        for _ in 0..poly_num {
            let pos = f.read_u16::<LittleEndian>().unwrap() as usize;
            thing_buf.push(tri_buf[pos]);
        }

        // logging stuff
        let pfx = if mat_num > 1 {
            format!("[{}/{}] ", i + 1, mat_num)
        } else {
            String::new()
        };

        // path stuff time!!!!
        let in_base = input.file_name().unwrap().to_str().unwrap();

        let obj_f_name = {
            if mat_num > 1 {
                format!("{}.{}.obj", in_base, i)
            } else {
                format!("{}.obj", in_base)
            }
        };
        let mtl_f_name = {
            if mat_num > 1 {
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

        // write object
        println!("{}Writing object (.obj) to {:?}...", pfx, obj_f_path);
        let mut obj_f = fs::File::create(obj_f_path).unwrap();
        obj::write_obj_alt(&mut obj_f, &mtl_f_name, &vtx_buf, &uv_buf, &thing_buf).unwrap();

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
