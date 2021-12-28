
use std::fs;

fn get_data(filename: String, i: usize, j: usize) -> Result<String, String> {

    let contents = fs::read_to_string(filename).expect("Unable to read file");

    let mut line_index = 0;
    let mut x = -1;
    let mut y = -1;
    let mut data = String::new();

    for line in contents.lines() {
        if line.chars().nth(0).unwrap() == '#' {
            continue ;
        }
        if line_index == 0 { 
            for var in line.replace(" ", "").split(',') {
                let var_name = &var[..var.find('=').expect("Can't find var_name")];
                println!("{}", var_name);

                match var_name {
                    "x" => {
                        x = var[var.find('=').expect("Can't find var_name") + 1..var.len()].parse::<i32>().unwrap();
                    }
                    "y" => {
                        y = var[var.find('=').expect("Can't find var_name") + 1..var.len()].parse::<i32>().unwrap();
                    }
                    _ => {}
                }
            }
        } else {
            data += line;
        }
        line_index += 1;
    }

    if j + x as usize >= crate::WIDTH || i + y as usize >= crate::HEIGHT {
        return Err(String::from("Exceed size"));
    }

    Ok(data)
}

pub fn draw_pattern(filename: String, cells: &mut [[u8; crate::WIDTH]; crate::HEIGHT], y: usize, x: usize) {

    let data = get_data(filename, y, x).expect("Exceed size");

    let mut i = 0;
    for row in data.split('$') {
        let mut j = 0;
        let mut n: i32 = 0;
        for c in row.chars() {
            if c == 'o' || c == 'b' {
                if n == 0 {
                    n += 1;
                }
                if c == 'o' {
                    for m in 0..n as usize {
                        cells[y + i][x + j + m] = crate::ALIVE;
                    }
                }
                j += n as usize;
                n = 0;
            } else if c == '!' {
                
            } else {
                n = n * 10 + c as i32 - '0' as i32;
            }
        }
        i += 1;
    }
}