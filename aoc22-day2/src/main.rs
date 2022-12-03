use std::fs;

fn main() {
    let file = fs::read_to_string("src/input.txt")
            .expect("Should have worked");
    let inputs = file.lines();
    let vec: Vec<&str> = inputs.collect();
    let mut score:u32 = 0;
    for lines in vec {
        let values = lines.split_ascii_whitespace();
        let mut vecvec:Vec<&str> = vec![];
        for val in values {
            vecvec.push(val);
        }
        // ROCK A 1
        // PAPER B 2 
        // SCISSOR C 3
        // X lose
        // Y draw
        // Z win
        if vecvec[0] == "A" {
            if vecvec[1] == "X" {
                score += 3;
            } else if vecvec[1] == "Y" {
                score += 4;
            } else if vecvec[1] == "Z" {
                score+=8;
            }
        } else if vecvec[0] == "B" {
            if vecvec[1] == "X" {
                score += 1;
            } else if vecvec[1] == "Y" {
                score += 5;
            } else if vecvec[1] == "Z" {
                score+=9;
            }
        } else if vecvec[0] == "C" {
            if vecvec[1] == "X" {
                score += 2;
            } else if vecvec[1] == "Y" {
                score += 6;
            } else if vecvec[1] == "Z" {
                score+=7;
            }
        }
    }
    println!("{}", score);
}