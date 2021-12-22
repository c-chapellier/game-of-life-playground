
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

    gui.draw_cells(&cells);
    gui.refresh();

    let mut sum: u128 = 0;
    let mut n = 0;

    let mut status = PAUSED;
    let mut selected_pattern: String = String::from("glider");

    let mut refresh_rate = 1_000_000_000u32 / 4;

    'running: loop {

        let now = std::time::Instant::now();
        if status == STARTED {

            cells_tmp = cells;
            step(&mut cells, &mut cells_tmp);
            cells = cells_tmp;
            
            gui.draw_cells(&cells);
            gui.refresh();
        }
        let elapsed = now.elapsed();
        sum += elapsed.as_micros();
        n += 1;
        println!("Elapsed: {:.2?}", elapsed);

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
                    if status == PAUSED {
                        for i in 0..HEIGHT {
                            for j in 0..WIDTH {
                                cells[i][j] = DEAD;
                            }
                        }
                        gui.draw_cells(&cells);
                        gui.refresh();
                    }
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => {
                    selected_pattern = String::from("centinalreflector");
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Equals), .. } => {
                    refresh_rate /= 3;
                    println!("ref {}", refresh_rate);
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), .. } => {
                    refresh_rate *= 3;
                    println!("ref {}", refresh_rate);
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
                    selected_pattern = String::from("glider");
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => {
                    break 'running
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => {
                    status = if status == STARTED { PAUSED } else { STARTED };
                }
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
                    if status == PAUSED {
                        for i in 0..HEIGHT {
                            for j in 0..WIDTH {
                                cells[i][j] = rng.gen_range(0..2);
                            }
                        }
                        gui.draw_cells(&cells);
                        gui.refresh();
                    }
                }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    // x == width, y == height
                    if status == PAUSED {
                        let i = y as usize / (CELL_HEIGHT + BORDER_SIZE);
                        let j = x as usize / (CELL_WIDTH + BORDER_SIZE);

                        let mut filename = String::from("./src/embed/");
                        filename.push_str(&selected_pattern);
                        filename += ".rle";
                        println!("path {}", filename);

                        patterns::draw_pattern(filename, &mut cells, i, j);
                        gui.draw_cells(&cells);
                        gui.refresh();
                    }
                }
                _ => {}
            }
        }

        let now2 = std::time::Instant::now();
        {
            std::thread::sleep(std::time::Duration::new(0, refresh_rate));
        }
        println!("sleep: {:.2?}", now2.elapsed());

        // if n > 20 {
            // break ;
        // }
    }

    println!("Mean: {:.2?}µs", sum / n);

    Ok(())
}

fn step(cells: &mut [[u8; WIDTH]; HEIGHT], cells_tmp: &mut [[u8; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {

            let mut n_neighbours_alive = -(cells[i][j] as i8);

            for k in 0..3 {
                for l in 0..3 {

                    let m = i as i32 - 1 + k;
                    let l = j as i32 - 1 + l;

                    if m >= 0 && l >= 0 && m < HEIGHT as i32 && l < WIDTH as i32 {
                        n_neighbours_alive += cells[m as usize][l as usize] as i8;
                    }
                }
            }

            if cells[i][j] == ALIVE && n_neighbours_alive != 2 && n_neighbours_alive != 3 {
                cells_tmp[i][j] = DEAD;
            } else if cells[i][j] == DEAD && n_neighbours_alive == 3 {
                cells_tmp[i][j] = ALIVE;
            }
        }
    }
}
