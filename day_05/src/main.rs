extern crate vec_from_file;

use vec_from_file::input_to_vec;

fn main() {
    let input_vec = input_to_vec("input");

    task_one(&input_vec);
    task_two(&input_vec);
}

fn task_two(input: &Vec<String>){
    let mut crates = create_stack(&input);

    for line in input.iter(){

        if !line.contains("move"){
            continue;
        }
        
        let commands = read_commands(line);
        crates = crate_mover_9001(crates, commands);

    }

    println!("{:?}", write_result(crates));

}

fn task_one(input: &Vec<String>){
    let mut crates = create_stack(&input);

    for line in input.iter(){

        if !line.contains("move"){
            continue;
        }
        
        let commands = read_commands(line);
        crates = crate_mover_9000(crates, commands);

    }

    println!("{:?}", write_result(crates));
}

fn write_result(crates: Vec<Vec<String>>) -> String{
    let mut result: String = String::new();

    for stack in crates.iter(){
        if stack.len() == 0{
            continue;
        }
        let top = &stack[stack.len()-1];
        let temp_char: Vec<char> = top.chars().collect(); 
        result.push(temp_char[1]);
    }

    result

}

fn read_commands(line: &String) -> Vec<i32>{
    let items_saaa: Vec<&str> = line.split(" ").collect();

    let mut commands: Vec<i32> = Vec::new();

    for i in items_saaa{
        let test = match i.parse::<i32>(){
            Ok(n) => n,
            _ => continue
        };
        commands.push(test);
    }
    commands
}
/*
crate mover 9001 can move multiple containers at once
 */
fn crate_mover_9001(mut crates: Vec<Vec<String>>, commands: Vec<i32>) -> Vec<Vec<String>>{
    println!("{:?}", commands);
    let num_move = commands[0];
    let from = commands[1]-1;
    let to = commands[2]-1;

    let move_index = crates[from as usize].len() - num_move as usize;

    let mut stack = crates[from as usize].split_off(move_index as usize);

    crates[to as usize].append(&mut stack);

    crates
}
/*
crate mover 9000 can only move one by one
 */
fn crate_mover_9000(mut crates: Vec<Vec<String>>, commands: Vec<i32>) -> Vec<Vec<String>>{
    for _ in 0..commands[0]{
        let pop_os = match crates[(commands[1] as usize)-1].pop(){
            Some(n) => n,
            None => continue
        };
        crates[(commands[2] as usize)-1].push(pop_os);
    }

    crates
}

fn print_crates(crates : &Vec<Vec<String>>){
    for c in crates.iter(){
        println!("{:?}", c);
    }
}

fn create_stack(input: &Vec<String>) -> Vec<Vec<String>>{

    let mut drawing: Vec<String> = Vec::new();
    let mut arangement: Vec<Vec<String>> = vec![vec![]; 9];

    for line in input.iter(){
        if line.is_empty(){
            break;
        }

        drawing.push(line.clone());
    }

    drawing.reverse();

    for line  in drawing.iter(){

        if line.is_empty(){
            continue;
        }

        let mut word: String = String::new();
        println!("{:?}", line);

        for (i, letter) in line.chars().enumerate(){
            if letter.is_whitespace(){
                continue;
            }
            word.push(letter);
            if word.ends_with("]"){
                let index: i32 = (i as i32 +1)/4;
                arangement[index as usize].push(word);
                word = String::new();
            }
        }
    }

    println!("__________________");

    for c in arangement.iter(){
        println!("{:?}", c);
    }
    
    println!("__________________");

    arangement
}
