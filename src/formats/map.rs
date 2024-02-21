use std::{
    fs,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{common, conversion::obj};

/*struct ObjPos {
    vec: (f32, f32, f32),
    angle: f32,
    type_: i32,
}

struct Cell {
    numcells: i32,
    numtri: i32,
    numuv: i32,
    triofs: i32,
    uvofs: i32,
    cellofs: i32,
}

struct CellInfo {
    tri_idx_buf: Vec<i32>,
    uv_idx_buf: Vec<i32>,
    pvscell: Vec<i32>,
    numtri: i32,
    numuv: i32,
    numcell: i32,
}

struct UvAnimation {
    flow_u: f32,
    flow_v: f32,
    uv_index_array: Vec<i32>,
}

struct TexAnimation {
    type_: i32,
    uv_index_array: Vec<u16>,
}*/

pub fn map_to_obj<R: Read + Seek>(
    mut f: R,
    input: &Path,
    output: &Path,
    root: Option<String>,
) {
    //
    // ReadHeader
    //
    let magic = f.read_i32::<LittleEndian>().unwrap();
    let version = f.read_i32::<LittleEndian>().unwrap();

    if magic != 1245859584 || version != 16777216 {
        eprintln!("Invalid magic or version, skipping...");
        return;
    }

    let geom_off = f.read_i32::<LittleEndian>().unwrap();
    f.read_i32::<LittleEndian>().unwrap(); // unused, split
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

    f.read_i32::<LittleEndian>().unwrap(); // unused, height_off

    f.read_i32::<LittleEndian>().unwrap(); // unused, obj_pos_num
    f.read_i32::<LittleEndian>().unwrap(); // unused, obj_pos_off

    f.read_i32::<LittleEndian>().unwrap(); // unused, cell_num
    f.read_i32::<LittleEndian>().unwrap(); // unused, cell_off

    // unused for conversion atm
    let _grid_max = (
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
    );
    let _grid_min = (
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
        f.read_f32::<LittleEndian>().unwrap(),
    );

    f.read_i32::<LittleEndian>().unwrap(); // unused, uv_anim_num
    f.read_i32::<LittleEndian>().unwrap(); // unused, tex_anim_num
    f.read_i32::<LittleEndian>().unwrap(); // unused, uv_anim_off
    f.read_i32::<LittleEndian>().unwrap(); // unused, tex_anim_off
    /*let mut uv_anim_off_cur = uv_anim_off as u64;
    let mut tex_anim_off_cur = tex_anim_off as u64;*/

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

        // ReadPolyData
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

    // Everything under here is unused atm

    /*
    //
    // ReadObjPosInfo
    //
    f.seek(SeekFrom::Start(obj_pos_off as u64)).unwrap();
    let mut obj_pos_arr = vec![];
    for _ in 0..obj_pos_num {
        obj_pos_arr.push(ObjPos {
            vec: (
                f.read_f32::<LittleEndian>().unwrap(),
                f.read_f32::<LittleEndian>().unwrap(),
                f.read_f32::<LittleEndian>().unwrap(),
            ),
            angle: -f.read_f32::<LittleEndian>().unwrap(),
            type_: f.read_i32::<LittleEndian>().unwrap(),
        });
    }

    //
    // ReadPVS
    //
    f.seek(SeekFrom::Start(cell_off as u64)).unwrap();
    let mut cell_list = vec![];
    for _ in 0..cell_num {
        cell_list.push(Cell {
            numcells: f.read_i32::<LittleEndian>().unwrap(),
            numtri: f.read_i32::<LittleEndian>().unwrap(),
            numuv: f.read_i32::<LittleEndian>().unwrap(),
            triofs: f.read_i32::<LittleEndian>().unwrap(),
            uvofs: f.read_i32::<LittleEndian>().unwrap(),
            cellofs: f.read_i32::<LittleEndian>().unwrap(),
        });
    }

    //
    // ReadCellData
    //
    let mut cell_info_list = vec![];
    for x in &cell_list {
        f.seek(SeekFrom::Start(x.cellofs as u64)).unwrap();

        let mut cell_arr = vec![];
        for _ in 0..x.numcells {
            cell_arr.push(f.read_i32::<LittleEndian>().unwrap());
        }

        let mut cell_tri_arr = vec![];
        for _ in 0..x.numtri {
            cell_tri_arr.push(f.read_i32::<LittleEndian>().unwrap());
        }

        let mut cell_uv_arr = vec![];
        for _ in 0..x.numuv {
            cell_uv_arr.push(f.read_i32::<LittleEndian>().unwrap());
        }

        cell_info_list.push(CellInfo {
            tri_idx_buf: cell_tri_arr,
            uv_idx_buf: cell_uv_arr,
            pvscell: cell_arr,
            numtri: x.numtri,
            numuv: x.numuv,
            numcell: x.numcells,
        });
    }

    //
    // ReadUVAnimInfo
    //
    let mut uv_anim_arr = vec![];
    for _ in 0..uv_anim_num {
        f.seek(SeekFrom::Start(uv_anim_off_cur)).unwrap();
        let flow_u = f.read_f32::<LittleEndian>().unwrap();
        let flow_v = f.read_f32::<LittleEndian>().unwrap();
        let uv_index_num = f.read_i32::<LittleEndian>().unwrap();
        let ofs = f.read_i32::<LittleEndian>().unwrap();
        uv_anim_off_cur = f.seek(SeekFrom::Current(0)).unwrap();

        //
        // ReadUVIndexInfo
        //
        f.seek(SeekFrom::Start(ofs as u64)).unwrap();

        let mut uv_index_arr = vec![];
        for _ in 0..uv_index_num {
            uv_index_arr.push(f.read_i32::<LittleEndian>().unwrap());
        }
        uv_anim_arr.push(UvAnimation {
            flow_u,
            flow_v,
            uv_index_array: uv_index_arr,
        });
    }

    //
    // ReadTextureAnimInfo
    //
    let mut tex_anim_arr = vec![];
    for _ in 0..tex_anim_num {
        f.seek(SeekFrom::Start(tex_anim_off_cur)).unwrap();
        let type_ = f.read_i32::<LittleEndian>().unwrap();
        let ofs = f.read_i32::<LittleEndian>().unwrap();
        let uv_index_num = f.read_i32::<LittleEndian>().unwrap();
        tex_anim_off_cur = f.seek(SeekFrom::Current(0)).unwrap();

        //
        // ReadTextureUVIndexInfo
        //
        f.seek(SeekFrom::Start(ofs as u64)).unwrap();

        let mut uv_index_arr = vec![];
        for _ in 0..uv_index_num {
            uv_index_arr.push(f.read_u16::<LittleEndian>().unwrap());
        }
        tex_anim_arr.push(TexAnimation {
            type_,
            uv_index_array: uv_index_arr,
        });
    }
    */
}
