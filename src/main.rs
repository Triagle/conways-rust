use std::error::Error;
use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use std::thread;
/*
A Conway's game of life implementation in Rust, as a training exercise for me learning the language
Goals:
- Pure, i.e subscribes to functional programming ideals
- Works, ideally
 */
fn above_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (_, y) if y == 0 => false,
        (x, y) => grid[y - 1][x]
    }
}
fn above_right_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, y) if y == 0 || x + 1 == grid[y].len() => false,
        (x, y) => grid[y - 1][x + 1]
    }
}
fn above_left_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, y) if y == 0 || x == 0 => false,
        (x, y) => grid[y - 1][x - 1]
    }
}

fn left_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, _) if x == 0 => false,
        (x, y) => grid[y][x - 1]
    }
}
fn right_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, y) if x + 1 == grid[y].len() => false,
        (x, y) => grid[y][x + 1]
    }
}
fn below_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (_, y) if y + 1 == grid.len() => false,
        (x, y) => grid[y + 1][x]
    }
}
fn below_right_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, y) if y + 1 == grid.len() || x + 1 == grid[y].len() => false,
        (x, y) => grid[y + 1][x + 1]
    }
}
fn below_left_alive(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    match pos {
        (x, y) if y + 1 == grid.len() || x == 0 => false,
        (x, y) => grid[y + 1][x - 1]
    }
}

// This function counts the number of live tiles next to any given tile
fn count_neighbours(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> usize {
    let directional_functions = [above_alive, above_right_alive, above_left_alive, left_alive, right_alive, below_alive, below_right_alive, below_left_alive];
    match pos {
        (x, y) if x < grid[y].len() && y < grid.len() => {
            directional_functions.into_iter().filter(|f| f(grid, pos)).count()
        },
        _ => 0,
    }
}
fn update(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter().zip((0usize..grid.len())).map(|(row, y)| {
        row.iter().zip((0usize..row.len())).map(|(point, x)| {
            match count_neighbours(grid, (x, y)) {
                n if n < 2 || n > 3 => false,
                n if n == 2 => *point,
                _ => true
            }
        }).collect::<Vec<bool>>()
    }).collect()
}
fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for point in row {
            if *point {
                print!(" x ");
            } else {
                print!(" . ");
            }
        }
        println!("\n");
    }
}
fn read_grid(path: &Path) -> Vec<Vec<bool>> {
    let mut file = match File::open(path) {
        Err(why) => panic!("Error opening {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s.lines().map(|l| l.split_whitespace().map(|p| if p == "x" {true} else {false}).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>()
}
fn valid_grid(grid: &Vec<Vec<bool>>) -> bool {
    match grid.len() {
        0 => false,
        _ => grid.iter().map(|l| l.len()).all(|l| l == grid[0].len()),
    }
}
fn clear_lines(lines: usize) {
    for _ in 0..lines {
        println!("\x1b[K\x1b[1A");
    }
}
fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut grid = read_grid(Path::new(&path));
        if valid_grid(&grid) {
            loop {
                print_grid(&grid);
                thread::sleep(Duration::from_millis(200));
                grid = update(&grid);
                clear_lines(grid.len());
            }
        } else {
            println!("Please provide a valid grid (must be regular rectangle of equal length for every row)");
        }
    } else {
        println!("Usage conway <file>");
    }
}
