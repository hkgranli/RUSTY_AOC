extern crate vec_from_file;
use vec_from_file::input_to_vec;

fn main() {
    let input: String = input_to_vec("input").get(0).unwrap().clone();
    // task one
    find_seq_start(&input, 4);
    // task two
    find_seq_start(&input, 14)
}

fn find_seq_start(input: &String, seq_len: usize){
    let string_clone = input.clone();
    let char_vec: Vec<char> = string_clone.chars().collect();

    for (i, _) in char_vec.iter().enumerate(){
        let sequence = &char_vec[i..(i+seq_len)];
        if !has_duplicate(sequence){
            println!("The number you are looking for is {}", i+seq_len);
            return 
        }
    }
}

fn has_duplicate(seq: &[char]) -> bool{
    let mut sequence = Vec::from(seq);
    sequence.sort();
    for (i, c) in sequence.iter().skip(1).enumerate(){
        if c.clone() == sequence[i]{
            return true
        }
    }
    false
}
