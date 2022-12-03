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
        // ROCK A & X
        // PAPER B & Y 
        // SCISSOR C & Z
        if vecvec[0] == "A" {
            if vecvec[1] == "X" {
                score += 4;
            } else if vecvec[1] == "Y" {
                score += 8;
            } else if vecvec[1] == "Z" {
                score+=3;
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
                score += 7;
            } else if vecvec[1] == "Y" {
                score += 2;
            } else if vecvec[1] == "Z" {
                score+=6;
            }
        }
    }
    println!("{}", score);
}