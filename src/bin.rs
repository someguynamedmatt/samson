use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("samson > ");
    let _ = std::io::stdout().flush();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        }
        Err(error) => println!("[ERROR]: {}", error),
    }
}

