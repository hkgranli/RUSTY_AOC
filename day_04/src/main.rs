extern crate vec_from_file;

use vec_from_file::input_to_vec;

fn main() {
    let input_vec: Vec<String> = input_to_vec("input");
    task_one(&input_vec);
    task_two(&input_vec);
}

struct CleanupJob{
    start: i32,
    end: i32
}
impl CleanupJob{
    fn overlap(&self, partner: &CleanupJob) -> bool{
        return (self.start <= partner.start && self.end >= partner.end) || (partner.start <= self.start && partner.end >= self.end);
    }
    fn overlap_ranges(&self, partner: &CleanupJob) -> bool {
        return (self.start <= partner.start && self.end >= partner.start) || (partner.start <= self.start && partner.end >= self.start);
    }
}

fn task_one(input: &Vec<String>){
    let mut score : i32 = 0;
    for line in input.iter(){

        let (worker_one, worker_two) = line_to_workers(line);

        if worker_one.overlap(&worker_two){
            println!("{},{} overlap {},{}", worker_one.start, worker_one.end, worker_two.start, worker_two.end);
            score += 1;
        }
    }
    println!("Overlap-score: {}", score);
}

fn line_to_workers(line:&String) -> (CleanupJob, CleanupJob) {
        let worker_list: Vec<&str> = line.split(",").collect();
        let worker_one_string = worker_list[0].to_string();
        let worker_two_string = worker_list[1].to_string();
        let worker_one_info: Vec<&str> = worker_one_string.split("-").collect();
        let worker_two_info: Vec<&str> = worker_two_string.split("-").collect();
        let worker_one = CleanupJob{
            start : worker_one_info[0].to_string().parse::<i32>().unwrap(),
            end : worker_one_info[1].to_string().parse::<i32>().unwrap(),
        };
        let worker_two = CleanupJob {
            start : worker_two_info[0].to_string().parse::<i32>().unwrap(),
            end : worker_two_info[1].to_string().parse::<i32>().unwrap(),
        };

        return (worker_one, worker_two);
}

fn task_two(input: &Vec<String>){
    let mut score: i32 = 0;

    for line in input.iter(){
        let (worker_one, worker_two) = line_to_workers(line);
        
        if worker_one.overlap_ranges(&worker_two){
            score += 1;
        }
    }

    println!("Overlap-range-score: {}", score);

}