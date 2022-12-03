use std::{fs, collections::HashMap};
fn main() {
     let file = fs::read_to_string("src/input.txt")
            .expect("Should have worked");
    let inputs = file.lines();
    let vec: Vec<&str> = inputs.collect();
    let mut priority:i32 = 0;
    let mut steps = 0;
    let mut in_first_half:HashMap<u8,u8> = HashMap::new();
    let mut in_first_half_other:HashMap<u8,u8>= HashMap::new();
    for lines in vec {
        let chars = lines.as_bytes();
        
        if steps % 3 == 0 {
            in_first_half.clear();
             
        for char in chars {
            if in_first_half.contains_key(char){
                continue;
            } else {
            if *char > 90 {
                in_first_half.insert(*char, *char - 96);
            } else {
                in_first_half.insert(*char,*char-38);
            }
        }
    }
    } else if (steps+2) % 3 == 0 {
        in_first_half_other.clear(); 
    for char in chars {
        if in_first_half_other.contains_key(char) {
            continue;
        } else {
            if *char > 90 {
                in_first_half_other.insert(*char, *char - 96);
            } else {
                in_first_half_other.insert(*char,*char-38);
            }
        }
    }

    } else {
        for char in chars {
            if in_first_half.contains_key(char) && in_first_half_other.contains_key(char) {
                match in_first_half.get(char) {
                    Some(value) => priority += *value as i32,
                    None => println!("actually not possible"),
                };
                break;
            }
        }
        }
        steps += 1;
    }
    println!("{}", priority);
}
