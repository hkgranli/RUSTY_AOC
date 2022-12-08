extern crate vec_from_file;

use vec_from_file::input_to_vec;

fn main() {
    let input: Vec<String> = input_to_vec("input");
    let grid = create_grid(&input);

    println!("{:?}", task_one(&grid));
    println!("Highest scenic score: {:?}", task_two(&grid));
}

fn task_one(grid: &Vec<Vec<i8>>) -> i32 {
    let mut score : i32 = 0;
    for (i, row) in grid.iter().enumerate(){
        if i == 0 || (i == grid.len()-1){
            score += row.len() as i32;
            continue;
        }
        for (j, _col) in row.iter().enumerate(){
            if j == 0 || j == grid.len() -1 {
                score += 1;
                continue;
            }
            if visible(grid, i, j){
                println!("{}, {} is visibile", i, j);
                score += 1;
            }
        }
    }

    score
}

fn visible(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> bool{

    let mut vis_top = true;
    let mut vis_bot = true;
    let mut vis_r = true;
    let mut vis_l = true;

    let target_size = grid[row][col];

    for x in 0..grid.len(){
        if x == row {
            continue;
        }
        if x < row {
            if grid[x][col] >= target_size{
                vis_top = false
            }
        } else {
            if grid[x][col] >= target_size{
                vis_bot = false
            }
        }
    }

    if vis_top || vis_bot {
        return true
    }

    for z in 0..grid[row].len(){
        if z == col{
            continue;
        }
        if z < col {
            if grid[row][z] >= target_size{
                vis_l = false
            }
        } else {
            if grid[row][z] >= target_size{
                vis_r = false
            }
        }
    }

    return vis_l || vis_r

}

fn task_two(grid: &Vec<Vec<i8>>) -> i32 {
    let mut biggest_score = 0;

    for (i, row) in grid.iter().enumerate(){
        for (j, _col) in row.iter().enumerate(){
            let s = scenic_score(grid, i, j);
            if s > biggest_score{
                biggest_score = s;
            }
        }
    }

    biggest_score

}

fn scenic_score(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> i32 {
    let mut score_top = 0;
    let mut score_bot = 0;
    let mut score_l = 0;
    let mut score_r = 0;

    let tree_size = grid[row][col];

    let mut x = col as i32;

    while x > 0{
        //println!("x: {}", x);
        x -= 1;
        score_l += 1;

        if grid[row][x as usize] >= tree_size {
            x = -1;
        }
    }

    x = col as i32;
    
    while x < grid[row].len() as i32 - 1 {
        x += 1;
        score_r += 1;
        if grid[row][x as usize] >= tree_size {
            x = grid[row].len() as i32 + 1;
        }
    }
    
    x = row as i32;

    while x > 0{
        x -= 1;
        score_top += 1;
        if grid[x as usize][col] >= tree_size {
            x = -1;
        }
    }
    
    x = row as i32;

    while x < grid.len() as i32 - 1 {
        x += 1;
        score_bot += 1;
        if grid[x as usize][col] >= tree_size {
            x = grid.len() as i32 + 1;
        }
    }

    println!("Score for {},{} is: {}*{}*{}*{}={}", row, col, score_bot, score_top, score_l, score_r, score_bot*score_l*score_r*score_top);

    return score_bot * score_top * score_l * score_r
}

fn create_grid(input: &Vec<String>) -> Vec<Vec<i8>>{
    let mut out: Vec<Vec<i8>> = Vec::new();

    for line in input.iter(){
        let mut line_vec: Vec<i8> = Vec::new();
        for c in line.chars(){
            line_vec.push(
                c as i8
            );
        }
        if !line_vec.is_empty(){
            out.push(line_vec);
        }
    }
    out
}
