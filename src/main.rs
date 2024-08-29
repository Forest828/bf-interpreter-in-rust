use std::env;
use std::fs;
use std::io;

fn main() {
    let mut vec: Vec<u8> = vec!(0; 3001);
    let mut index: i32 = 1;
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Unable to read file");

    for (j, i) in contents.chars().enumerate() {
        match i {
            '+' => vec[index as usize] += 1,
            '-' => vec[index as usize] -= 1,
            '>' => index += 1,
            '<' => index -= 1,
            '.' => print!("{}", vec[index as usize] as char),
            ',' => {
                let mut temp = String::new();
                let _ = io::stdin().read_line(&mut temp);
                let temp = (temp.as_bytes()[0] as char) as u8;
                vec[index as usize] = temp;
            },
            '[' => {
                //println!("loop");
                let loop_variable: i32 = index;
                let mut loop_index: i32 = (j + 1).try_into().unwrap();
                let mut loop_section: String = String::new();                
                while contents.as_bytes()[loop_index as usize] as char != ']'{

                    loop_section.push(contents.as_bytes()[loop_index as usize] as char);
                    loop_index += 1;
                }
                let mut loop_section_index: i32 = 0;
                while vec[loop_variable as usize] != 0 {
                    match loop_section.as_bytes()[loop_section_index as usize] as char {
                        '+' => vec[index as usize] += 1,
                        '-' => vec[index as usize] -= 1,
                        '>' => index += 1,
                        '<' => index -= 1,
                        _ => {},
                    }
                    if loop_section_index + 1 != loop_section.len() as i32 {
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
