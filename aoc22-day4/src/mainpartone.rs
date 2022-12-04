use std::fs;
use std::str;

fn main() {
    let file = fs::read_to_string("src/input.txt")
            .expect("Should have worked");
    let inputs = file.lines();
    let vec: Vec<&str> = inputs.collect();
    let mut count: u32 = 0;
    for lines in vec {
        let pair = lines.split(",");
        let mut nums:Vec<u32> = vec![];
        for single in pair {
            let single = single.split("-");
            let single: Vec<&str> = single.collect();
            for numbers in single {
                let numbers = numbers.parse::<u32>().unwrap();
                nums.push(numbers);
            }
        }
        if nums[0] <= nums[2] && nums[1] >= nums[3] || nums[2] <= nums[0] && nums[3] >= nums[1] {
            count+=1;
        }
    }
    println!("{}", count)
}
