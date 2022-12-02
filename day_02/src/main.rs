extern crate vec_from_file;
use vec_from_file::input_to_vec;



fn main() {
    let input_data = input_to_vec("input");
    
    task_one(&input_data);
    task_two(&input_data);

}

fn task_one(input: &Vec<String>){
    let mut score: i32 = 0;

    for line in input.iter(){
        let line_vec: Vec<&str> = line.split(" ").collect();
        score += calc_strategy_one(line_vec[0].to_string(), line_vec[1].to_string());
    }

    println!("total score: {}", score);

}

fn calc_strategy_one(opponent: String, player: String) -> i32 {
    // a = rock, b = paper, c = scissor
    // x = rock, y = paper, z = scissor

    let outcome : i32;
    let selection_score: i32 = match player.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };

    if opponent.eq("A"){
        outcome = match selection_score {
            1 => 3,
            2 => 6,
            3 => 0,
            _ => 0
        };
    } else if opponent.eq("B"){
        outcome = match selection_score {
            1 => 0,
            2 => 3,
            3 => 6,
            _ => 0
        };
    } else {
        outcome = match selection_score {
            1 => 6,
            2 => 0,
            3 => 3,
            _ => 0
        };

    }
    return selection_score + outcome;
    
}

fn task_two(input: &Vec<String>){
    let mut score: i32 = 0;

    for line in input.iter(){
        let line_vec: Vec<&str> = line.split(" ").collect();
        score += calc_strategy_two(line_vec[0].to_string(), line_vec[1].to_string());
    }
    println!("total score: {}", score);
}

fn calc_strategy_two(opponent: String, outcome: String) -> i32{
    // X => need to lose, Y => need to draw, Z => need to win

    let selection_score : i32;
    let outcome_score: i32 = match outcome.as_str(){
        "X" => 0,
        "Y" => 3, 
        "Z" => 6,
        _ => 0
    };
    // a = rock, b = paper, c = scissor

    if opponent.eq("A"){
        selection_score = match outcome_score{
            0 => 3,
            3 => 1,
            6 => 2,
            _ => 0
        };
    } else if opponent.eq("B"){
        selection_score = match outcome_score{
            0 => 1,
            3 => 2,
            6 => 3,
            _ => 0
        };
    } else {
        selection_score = match outcome_score{
            0 => 2,
            3 => 3,
            6 => 1,
            _ => 0
        };
    }

    return outcome_score + selection_score;

}