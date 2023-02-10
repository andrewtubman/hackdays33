use std::io;

const HEIGHT: usize = 25;
const WIDTH: usize = 25;

fn main() {
    let mut grid = [[1; WIDTH]; HEIGHT];

    grid = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    
    let mut new_grid = [[1; WIDTH]; HEIGHT];
    
    loop {
        draw(&grid);
        
        let mut x = 0;
        let mut y = 0;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        for row in grid {
            for col in row {
                new_grid[x][y] = next_state(x, y, &grid);
                x += 1;
            }
            y += 1;
            x = 0;
        }
        grid = new_grid;

    }
}

fn next_state(x: usize, y: usize, grid: &[[usize; WIDTH]; HEIGHT]) -> usize {
    let top_left = if x > 0 && y > 0 {
        grid[x - 1][y - 1]
    } else {
        0
    };
    let top_center = if y > 0 { grid[x][y - 1] } else { 0 };
    let top_right = if x < WIDTH - 1 && y > 0 {
        grid[x + 1][y - 1]
    } else {
        0
    };
    let center_left = if x > 0 { grid[x - 1][y] } else { 0 };
    let center_right = if x < WIDTH - 1 { grid[x + 1][y] } else { 0 };
    let bottom_center = if y < HEIGHT - 1 { grid[x][y + 1] } else { 0 };
    let bottom_left = if y < HEIGHT - 1 && x > 0 {
        grid[x - 1][y + 1]
    } else {
        0
    };

    let bottom_right = if y < HEIGHT - 1 && x < WIDTH - 1 {
        grid[x + 1][y + 1]
    } else {
        0
    };

    let neighbor_score = top_left
        + top_center
        + top_right
        + center_left
        + center_right
        + bottom_center
        + bottom_left
        + bottom_right;

    if neighbor_score < 2 || neighbor_score > 3 {
        return 0;
    } else if neighbor_score == 3 {
        return 1;
    } else if neighbor_score == 2 {
        return grid[x][y];
    } else {
        return 0;
    }
}

fn draw(grid: &[[usize; WIDTH]; HEIGHT]) {
    for row in grid {
        for col in row {
            let tile = if *col == 1 {
                '■'
            }
            else {
                '□'
            };
            print!("{} ", tile);
        }
        println!()
    }
}
