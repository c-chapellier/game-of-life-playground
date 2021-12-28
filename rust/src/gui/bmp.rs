
use byteorder::{LittleEndian, WriteBytesExt};

use std::io::Write;

const HEADER_SIZE: usize = 14;
const DIB_HEADER_SIZE: usize = 40;

const HRES: i32 = 2835;
const VRES: i32 = 2835;

pub fn write_file(pixels: &[[[u8; 3]; 100]; 100], filename: &str) -> std::io::Result<()> {

    let width = pixels.len();
    let height = pixels[0].len();
    let data_size = height * width * 4;

    let mut headers = Vec::with_capacity(HEADER_SIZE + DIB_HEADER_SIZE + data_size);

    headers.write_all(&[0x42, 0x4D])?;
    headers.write_u32::<LittleEndian>((HEADER_SIZE + DIB_HEADER_SIZE + data_size) as u32)?;
    headers.write_u16::<LittleEndian>(0)?;
    headers.write_u16::<LittleEndian>(0)?;
    headers.write_u32::<LittleEndian>((HEADER_SIZE + DIB_HEADER_SIZE) as u32)?; // pixel_offset

    headers.write_u32::<LittleEndian>(DIB_HEADER_SIZE as u32)?;
    headers.write_i32::<LittleEndian>(width as i32)?;
    headers.write_i32::<LittleEndian>(height as i32)?;
    headers.write_u16::<LittleEndian>(1)?; // num_planes
    headers.write_u16::<LittleEndian>(32)?; // bits_per_pixel
    headers.write_u32::<LittleEndian>(0)?; // compress_type
    headers.write_u32::<LittleEndian>(data_size as u32)?;
    headers.write_i32::<LittleEndian>(HRES)?;
    headers.write_i32::<LittleEndian>(VRES)?;
    headers.write_u32::<LittleEndian>(0)?; // num_colors
    headers.write_u32::<LittleEndian>(0)?; // num_imp_colors

    let mut fd = std::fs::File::create(filename)?;

    fd.write_all(headers.as_slice())?;

    for i in (0..height).rev() {
        for j in 0..width {
            fd.write_all(&[pixels[i][j][2], pixels[i][j][1], pixels[i][j][0], 0])?;
        }
    }

    Ok(())
}
