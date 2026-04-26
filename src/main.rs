use std::io::Write;
use std::process::exit;

fn main() {
    loop {
        //Main input starting out
        //Presenting where you are -> flushing before -> taking input -> further goes onto processing input
        print!("HRCL$ ");
        std::io::stdout().flush().expect("Could not flush stdout");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            exit(0);
        } else {
            println!("Invalid Command: {}", input.trim());
        }
    }
}

// Things i need to figure out: Executing funcions in other files
//
// Ill mainly do this through mod
// the file structure will look like this: src/command/mod.rs, other files.rs
// this will allow me to include the commands i want to add in mod.rs, and i should be able to include mod.rs in the main file to execute the functions in the custom
// files. I also want the user to be able to add and rs files they want to, i plan on using an external script to do this so they dont have to recomplie the proj everytime
// anyways its 3:45 am so ima call it. GN
