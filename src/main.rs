use minifb::{Key, Window, WindowOptions, KeyRepeat, MouseMode, MouseButton};

const CELL_SIZE: usize = 10;
const NUMBER_CELLS_WIDTH: usize = 100;
const NUMBER_CELLS_HEIGHT: usize = 100;

const WIDTH: usize = CELL_SIZE * NUMBER_CELLS_WIDTH;
const HEIGHT: usize = CELL_SIZE * NUMBER_CELLS_HEIGHT;

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1), (1, 0),  (1, 1)
];

#[derive(Clone)]
struct Cell {
    current_state: u32,
    next_state: u32,
}

fn update_buffer(buffer: &mut Vec<u32>, cell_grid: &Vec<Vec<Cell>>) {
    for y in 0..NUMBER_CELLS_HEIGHT {
        for x in 0..NUMBER_CELLS_WIDTH {
            let cell = &cell_grid[y][x];
            let color: u32 = if cell.current_state == 0 { 0x393E46 } else { 0x00ADB5 };

            let y_start: usize = y * (CELL_SIZE);
            let y_end: usize = y_start + CELL_SIZE;
            let x_start: usize = x * (CELL_SIZE);
            let x_end: usize = x_start + CELL_SIZE;

            for py in y_start..y_end {
                for px in x_start..x_end {
                    let index: usize = py * WIDTH + px;
                    buffer[index] = color;
                }
            }
        }
    }
}

fn main() {
    let mut cell_grid: Vec<Vec<Cell>> = vec![vec![Cell { current_state: 0, next_state: 0 }; NUMBER_CELLS_WIDTH]; NUMBER_CELLS_HEIGHT];
    let mut buffer: Vec<u32> = vec![0x393E46; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            for y in 0..NUMBER_CELLS_HEIGHT {
                for x in 0..NUMBER_CELLS_WIDTH {
                    let cell = &cell_grid[y][x];
                    let mut live_neighbors = 0;

                    for &(dy, dx) in &NEIGHBOR_OFFSETS {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
            
                        if ny >= 0 && ny < NUMBER_CELLS_HEIGHT as isize &&
                        nx >= 0 && nx < NUMBER_CELLS_WIDTH as isize {
                            if cell_grid[ny as usize][nx as usize].current_state == 1 {
                                live_neighbors += 1;
                            }
                        }
                    }

                    let next_state = match (cell.current_state, live_neighbors) {
                        (1, 2) | (1, 3) => 1,
                        (0, 3) => 1,
                        _ => 0,
                    };
            
                    cell_grid[y][x].next_state = next_state;
                }
            }

            for y in 0..NUMBER_CELLS_HEIGHT {
                for x in 0..NUMBER_CELLS_WIDTH {
                    cell_grid[y][x].current_state = cell_grid[y][x].next_state;
                }
            }
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                let cx: usize = x as usize / CELL_SIZE;
                let cy: usize = y as usize / CELL_SIZE;

                cell_grid[cy][cx].current_state = 1;
            }
            if window.get_mouse_down(MouseButton::Right) {
                let cx: usize = x as usize / CELL_SIZE;
                let cy: usize = y as usize / CELL_SIZE;

                cell_grid[cy][cx].current_state = 0;
            }
        }

        update_buffer(&mut buffer, &cell_grid);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
