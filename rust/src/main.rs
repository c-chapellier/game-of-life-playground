
extern crate sdl2;
extern crate rand;
extern crate time;

use rand::Rng;

mod gui;
mod patterns;

const ALIVE: u8 = 1;
const DEAD: u8 = 0;

const STARTED: u8 = 1;
const PAUSED: u8 = 0;

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

    let mut rng = rand::thread_rng();

    let (mut gui, mut event_pump) = gui::create_gui();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            cells[i][j] = rng.gen_range(0..2);
        }
    }
    gui.draw_cells(&cells);
    gui.refresh();

    let mut sum: u128 = 0;
    let mut sum2: u128 = 0;
    let mut sum3: u128 = 0;
    let mut sum4: u128 = 0;

    let mut n = 0;

    // let mut status = PAUSED;
    // let mut selected_pattern: String = String::from("glider");

    let mut refresh_rate = 1_000_000_000u32 / 1;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let now = std::time::Instant::now();
    'running: loop {

        let now2 = std::time::Instant::now();
        // if status == STARTED {

            cells_tmp = cells;
            step(&mut cells, &mut cells_tmp);
            sum2 += now2.elapsed().as_micros();
            cells = cells_tmp;

            let now3 = std::time::Instant::now();
            gui.draw_cells(&cells);
            sum3 += now3.elapsed().as_micros();

            let now4 = std::time::Instant::now();
            gui.refresh();
        // }
        sum4 += now4.elapsed().as_micros();
        n += 1;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
        //             if status == PAUSED {
        //                 for i in 0..HEIGHT {
        //                     for j in 0..WIDTH {
        //                         cells[i][j] = DEAD;
        //                     }
        //                 }
        //                 gui.draw_cells(&cells);
        //                 gui.refresh();
        //             }
        //         }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => {
        //             selected_pattern = String::from("centinalreflector");
        //         }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Equals), .. } => {
        //             refresh_rate /= 3;
        //             println!("ref {}", refresh_rate);
        //         }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), .. } => {
        //             refresh_rate *= 3;
        //             println!("ref {}", refresh_rate);
        //         }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
        //             selected_pattern = String::from("glider");
        //         }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => {
                    break 'running
                }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => {
        //             status = if status == STARTED { PAUSED } else { STARTED };
        //         }
        //         sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
        //             if status == PAUSED {
        //                 for i in 0..HEIGHT {
        //                     for j in 0..WIDTH {
        //                         cells[i][j] = rng.gen_range(0..2);
        //                     }
        //                 }
        //                 gui.draw_cells(&cells);
        //                 gui.refresh();
        //             }
        //         }
        //         sdl2::event::Event::MouseButtonDown { x, y, .. } => {
        //             // x == width, y == height
        //             if status == PAUSED {
        //                 let i = y as usize / (CELL_HEIGHT + BORDER_SIZE);
        //                 let j = x as usize / (CELL_WIDTH + BORDER_SIZE);

        //                 let mut filename = String::from("./src/embed/");
        //                 filename.push_str(&selected_pattern);
        //                 filename += ".rle";
        //                 println!("path {}", filename);

        //                 patterns::draw_pattern(filename, &mut cells, i, j);
        //                 gui.draw_cells(&cells);
        //                 gui.refresh();
        //             }
        //         }
                _ => {}
            }
        }

        // let now2 = std::time::Instant::now();
        // {
        //     std::thread::sleep(std::time::Duration::new(0, refresh_rate));
        // }
        // println!("sleep: {:.2?}", now2.elapsed());

        if n > 200 {
            break ;
        }
    }

    println!("n: {}", n);
    println!("step: {:.2?}", sum2);
    println!("draw: {:.2?}", sum3);
    println!("refresh: {:.2?}", sum4);

    println!("run_time: {:.2?}", now.elapsed());

    // println!("Mean: {:.2?}µs", sum / n);

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

            if cells[i][j] == ALIVE && n_neighbours_alive != 2 && n_neighbours_alive != 3 {
                cells_tmp[i][j] = DEAD;
            } else if cells[i][j] == DEAD && n_neighbours_alive == 3 {
                cells_tmp[i][j] = ALIVE;
            }

            j += 1;
        }
        i += 1;
    }
}
