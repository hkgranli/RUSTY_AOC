extern crate vec_from_file;
use vec_from_file::input_to_vec;

fn main() {
    let input_vec: Vec<String> = input_to_vec("input");
    task_one(&input_vec);
    task_two(&input_vec);
}

fn task_one(input: &Vec<String>){

    let mut score: u32 = 0;

    for line in input.iter(){
        let (part1, part2) = line.split_at(line.len()/2);
        score += line_score(part1, part2);

    }

    println!("Task 1: Score was {}", score)
}

fn line_score(part1: &str, part2: &str) -> u32{
    for character in part1.chars(){
        for baracter in part2.chars(){
            if character == baracter {
                return prio(character);
            }
        }
    }
    0
}

fn task_two(input: &Vec<String>){
    let mut group_number = 0;

    let mut score: u32 = 0;

    while group_number < input.len()/3{
        let group: Vec<Vec<char>> = vec![input[group_number*3].clone().chars().collect(), input[group_number*3 + 1].clone().chars().collect(), input[group_number * 3 + 2].clone().chars().collect()];
        score += group_score(group);
        group_number += 1;
    }
    println!("Task 2: {}", score);
}

fn group_score(group: Vec<Vec<char>>) -> u32 {
    let i1 = intersect(&group[0], &group[1]);
    let i2 = intersect(&i1, &group[2]);
    return prio(i2[0]);
}

fn intersect(list1: &Vec<char>, list2: &Vec<char>) -> Vec<char>{
    let mut common: Vec<char> = Vec::new();
    println!("Intersec: {:?}, {:?}", list1, list2);
    for c in list1.iter(){
        for b in list2.iter(){
            if c == b && !common.contains(b){
                common.push(b.clone());
            }
        }
    }
    println!("{:?}", common);
    common
}

fn prio(item: char) -> u32{
    let d = item as u32;

    if d > 64 && d < 91{
        d - 38
    } else {
        d - 96
    }
}
