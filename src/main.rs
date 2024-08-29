use std::env;
use std::fs;
use std::io;

fn main() {
    let mut vec: Vec<u8> = vec!(0; 3000);
    let mut index: i32 = 0;
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Unable to read file");

    for i in contents.chars() {
        if i == '+' {
            vec[index as usize] += 1;
        } else if i == '-' {
            vec[index as usize] -= 1;
        } else if i == '>' {
            index += 1;
        } else if i == '<' {
            index -= 1;
        } else if i == '.' {
            print!("{}", vec[index as usize] as char)
        } else if i == ',' {
            let mut temp = String::new();
            let _ = io::stdin().read_line(&mut temp);
            let temp = (temp.as_bytes()[0] as char) as u8;
            vec[index as usize] = temp;
        }
    }    
}
