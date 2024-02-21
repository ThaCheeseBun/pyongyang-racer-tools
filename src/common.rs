use std::{error, io::Read};

// all formats seem to share a common version number
// it's just "1" in big endian but read as little
pub const FORMAT_VERSION: i32 = 16777216;

#[derive(Debug)]
pub struct Tri {
    pub a: i32,
    pub ta: i32,
    pub b: i32,
    pub tb: i32,
    pub c: i32,
    pub tc: i32,
}

pub fn read_string<R: Read>(r: &mut R, l: u8) -> Result<String, Box<dyn error::Error>> {
    let mut buf = vec![0u8; l as usize];
    r.read_exact(&mut buf)?;
    let str = String::from_utf8(buf)?;
    Ok(str)
}
