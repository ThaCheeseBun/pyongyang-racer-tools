use std::io::Write;

use crate::common;

pub fn write_obj<W: Write>(
    f: &mut W,
    mtl: &str,
    vtx: &Vec<(f32, f32, f32)>,
    uv: &Vec<(f32, f32)>,
    tri: &Vec<common::Tri>,
) -> Result<(), std::io::Error> {
    // write material info
    f.write(format!("mtllib {}\nusemtl default\n", mtl).as_bytes())?;
    // write points
    for x in vtx {
        let str = format!("v {} {} {}\n", x.0, x.1, x.2);
        f.write(str.as_bytes())?;
    }
    // write texture UVs
    for x in uv {
        let str = format!("vt {} {}\n", x.0, x.1);
        f.write(str.as_bytes())?;
    }
    // write faces
    for x in tri {
        let str = format!(
            "f {}/{} {}/{} {}/{}\n",
            x.c + 1,
            x.tc + 1,
            x.b + 1,
            x.tb + 1,
            x.a + 1,
            x.ta + 1
        );
        f.write(str.as_bytes())?;
    }
    // flush data to output
    f.flush()
}

pub fn write_obj_alt<W: Write>(
    f: &mut W,
    mtl: &str,
    vtx: &Vec<(f32, f32, f32)>,
    uv: &Vec<(f32, f32)>,
    tri: &Vec<(u16, u16, u16)>,
) -> Result<(), std::io::Error> {
    // write material info
    f.write(format!("mtllib {}\nusemtl default\n", mtl).as_bytes())?;
    // write points
    for x in vtx {
        let str = format!("v {} {} {}\n", x.0, x.1, x.2);
        f.write(str.as_bytes())?;
    }
    // write texture UVs
    for x in uv {
        let str = format!("vt {} {}\n", x.0, x.1);
        f.write(str.as_bytes())?;
    }
    // write faces
    for x in tri {
        let str = format!(
            "f {}/{} {}/{} {}/{}\n",
            x.2 + 1,
            x.2 + 1,
            x.1 + 1,
            x.1 + 1,
            x.0 + 1,
            x.0 + 1,
        );
        f.write(str.as_bytes())?;
    }
    // flush data to output
    f.flush()
}

pub fn write_mtl<W: Write>(f: &mut W, tex_name: &str) -> Result<(), std::io::Error> {
    let mtl_str = format!("newmtl default\nKa 1.0 1.0 1.0\nKd 1.0 1.0 1.0\nKs 0.0 0.0 0.0\nTr 0.0\nillum 1\nNs 0.0\nmap_Kd {}", tex_name);
    f.write(mtl_str.as_bytes())?;
    f.flush()
}
