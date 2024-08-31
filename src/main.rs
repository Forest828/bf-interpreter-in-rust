use std::env;
use std::fs;
use std::io;

fn main() {
    let mut vec: Vec<u8> = vec!(0; 3001);
    let mut index: usize = 1;
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Unable to read file");

    for (j, i) in contents.chars().enumerate() {
        match i {
            '+' => vec[index] += 1,
            '-' => vec[index] -= 1,
            '>' => index += 1,
            '<' => index -= 1,
            '.' => print!("{}", vec[index] as char),
            ',' => {
                let mut temp = String::new();
                let _ = io::stdin().read_line(&mut temp);
                vec[index] = (temp.as_bytes()[0] as char) as u8;
            },
            '[' => {
                let loop_variable: usize = index;
                let mut k: usize = j + 1;
                let mut loop_section: String = String::new();

                while contents.as_bytes()[k] as char != ']'{
                    loop_section.push(contents.as_bytes()[k] as char);
                    k += 1;
                }

                let mut loop_section_index: usize = 0;

                while vec[loop_variable] != 0 {
                    match loop_section.as_bytes()[loop_section_index] as char {
                        '+' => vec[index] += 1,
                        '-' => vec[index] -= 1,
                        '>' => index += 1,
                        '<' => index -= 1,
                        _ => {},
                    }
                    
                    if loop_section_index + 1 != loop_section.len() {
                        loop_section_index += 1;    
                    } else {
                        loop_section_index = 0;
                    }
                }
            },
            _ => {},
        }
    }
}
