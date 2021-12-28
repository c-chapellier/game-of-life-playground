
extern crate rand;
extern crate time;

const ALIVE: u8 = 1;
const DEAD: u8 = 0;

const WIN_HEIGHT: usize = 800;
const WIN_WIDTH: usize = 800;

const CELL_HEIGHT: usize = 3;
const CELL_WIDTH: usize = 3;
const BORDER_SIZE: usize = 0;

const HEIGHT: usize = WIN_HEIGHT / (CELL_HEIGHT + BORDER_SIZE);
const WIDTH: usize = WIN_WIDTH / (CELL_WIDTH + BORDER_SIZE);


fn main() -> std::io::Result<()> {

    let mut cells = [[0u8; WIDTH]; HEIGHT];
    let mut cells_tmp = [[0u8; WIDTH]; HEIGHT];

    let mut sum: u128 = 0;

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            cells[i][j] = (sum % 2) as u8;
            sum += ((i * HEIGHT) + j) as u128;
        }
    }

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let now = std::time::Instant::now();

    {
        let mut i = 0;
        while i < 200 {

            let now = std::time::Instant::now();
            cells_tmp = cells;
            step(&mut cells, &mut cells_tmp);
            cells = cells_tmp;

            i += 1;
        }
    }

    println!("run_time: {}ms", now.elapsed().as_millis());

    Ok(())
}

fn step(cells: &mut [[u8; WIDTH]; HEIGHT], cells_tmp: &mut [[u8; WIDTH]; HEIGHT]) {

    let mut i = 0;
    while i < HEIGHT {

        let mut j = 0;
        while j < WIDTH {

            let mut n_neighbours_alive = -(cells[i][j] as i8);
            let mut k = 0;

            while k < 3 {

                let mut l = 0;
                
                while l < 3 {

                    let m = i as i32 - 1 + k;
                    let n = j as i32 - 1 + l;

                    if m >= 0 && n >= 0 && m < HEIGHT as i32 && n < WIDTH as i32 {
                        n_neighbours_alive += cells[m as usize][n as usize] as i8;
                    }
                    l += 1;
                }
                k += 1;
            }

            if n_neighbours_alive == 3 {
                cells_tmp[i][j] = ALIVE;
            } else if cells[i][j] == ALIVE && n_neighbours_alive == 2 {
                cells_tmp[i][j] = ALIVE;
            } else {
                cells_tmp[i][j] = DEAD;
            }

            j += 1;
        }
        i += 1;
    }
}
