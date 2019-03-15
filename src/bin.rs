use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut flag = false;
    'main_loop: loop {
        let mut input = String::new();
        if flag {
            print!("...\t");
        } else {
            print!("samson > ");
        }
        let _ = std::io::stdout().flush();
        stdin.read_line(&mut input);

        if !input.contains(";\n") {
            flag = true;
        }
        if input.contains(";\n"){
            break 'main_loop
        }
    }
}
