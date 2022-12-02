extern crate vec_from_file;
use vec_from_file::input_to_vec;

fn calc_elf_cal(input: Vec<String>) -> Vec<i64> {
    let mut cal_count: Vec<i64>  = Vec::new();

    let mut current_cal_count : i64 = 0;

    for line in input{
        if line.eq(""){
            cal_count.push(current_cal_count);
            current_cal_count = 0;
            continue;
        }

        let line_int : i64 = line.parse::<i64>().unwrap();
        current_cal_count += line_int;
    }
    return cal_count;
}

fn part_one(cal_count: &Vec<i64>) {
    let max_val = cal_count.iter().max();
    match max_val {
        Some(max) => println!("the max value is {}", max),
        None => println!("vec is empty")
    }
}

struct BigThree{
    one : i64,
    two : i64,
    three : i64,
}

impl BigThree{
    fn total(&self) -> i64{
        return self.one + self.two + self.three;
    }
}

fn part_two(cal_count: &Vec<i64>){
    let mut big_three: BigThree = BigThree {
        one : 0, two : 0, three : 0,
    };

    for val in cal_count{
        let v = val.clone();
        if v > big_three.three {
            if v > big_three.one {
                big_three = BigThree {one : v, two : big_three.one, three: big_three.two};
            } else if v > big_three.two {
                big_three = BigThree {one : big_three.one, two : v, three: big_three.two};
            } else {
                big_three = BigThree {one : big_three.one, two : big_three.two, three: v};
            }
        }
    }

    println!("the three biggest boys are total {}", big_three.total());
}

fn main(){

    let input = input_to_vec("./input");
    let elf_cal = calc_elf_cal(input);

    part_one(&elf_cal);
    part_two(&elf_cal);

}