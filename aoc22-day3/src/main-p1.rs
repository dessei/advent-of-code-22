use std::{fs, collections::HashMap};
fn main() {
     let file = fs::read_to_string("src/input.txt")
            .expect("Should have worked");
    let inputs = file.lines();
    let vec: Vec<&str> = inputs.collect();
    let mut priority:i32 = 0;
    for lines in vec {
        let mut in_first_half = HashMap::new();
        let chars = lines.as_bytes();
        let cancel = (chars.len())/2;
        for char in 0..cancel {
            if in_first_half.contains_key(&chars[char]){
                continue;
            } else {
            if chars[char] > 90 {
                in_first_half.insert(chars[char], chars[char] - 96);
            } else {
                in_first_half.insert(chars[char],chars[char]-38);
            }
        }
        }
        for char in cancel..chars.len() {
                match in_first_half.remove_entry(&chars[char]) {
                    Some(value) => {
                        priority += value.1 as i32;
                        println!("{}", priority);
                    }
                        
                        ,
                    None => priority += 0
                };

            }
    }
    println!("{}", priority);
}
