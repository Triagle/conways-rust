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
        (x, y) if y == 0 || x + 1 == grid[0].len() => false,
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
        (x, _) if x + 1 == grid[0].len() => false,
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
        (x, y) if y + 1 == grid.len() || x + 1 == grid[0].len() => false,
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
        (x, y) if x < grid[0].len() && y < grid.len() => {
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
fn main() {
    let mut grid = vec![
        vec![false, false, false, false, false],
        vec![false, false, true, false, false],
        vec![false, false, false, true, false],
        vec![false, true, true, true, false],
        vec![false, false, false, false, false]];

    loop {
        println!("--------------");
        print_grid(&grid);
        println!("--------------");
        thread::sleep(Duration::from_millis(1000));
        grid = update(&grid);
    }
}
