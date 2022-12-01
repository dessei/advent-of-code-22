use std::fs;

fn main() {
    let file = fs::read_to_string("src/input.txt")
            .expect("Should have worked");
    let inputs = file.split("\n\n");
    let vec: Vec<&str> = inputs.collect();
    let mut sums: Vec<i32> = [].to_vec();
    for x in vec {
        let y = x.split_ascii_whitespace();
        let y_arr: Vec<&str> = y.collect();
        let mut temp_sum:i32 = 0;
        for num in y_arr {
            let nummed_num: i32 = num.parse().unwrap();
            temp_sum+=nummed_num;
        }
        sums.push(temp_sum);
    }
    let mut largest_three= vec![0,0,0];
    for single_sum in sums {
       if single_sum > largest_three[0] {
        let tmp= largest_three[0];
        largest_three[0]=single_sum;
        largest_three[2] = largest_three[1];
        largest_three[1] = tmp;
       } else if single_sum > largest_three[1] {
        largest_three[2] = largest_three[1];
        largest_three[1] = single_sum;
       } else if single_sum > largest_three[2] {
        largest_three[2] = single_sum;
       }
    }
    println!("{}",largest_three[0]+largest_three[1]+largest_three[2]);
}
