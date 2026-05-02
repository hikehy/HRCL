//hrcl imports
use mlua::{self};
use std::fs;
use std::io::Write;
use std::process::exit;

//taking a string and changing it into a vec by whitespace
fn parse_args(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

//this starts by taking a vector string, and then it does error handling,
// then it creates a file path and runs that file path with lua while creating a lua global (the string before)
// i did this so we could have multiline commands
fn run_lua(command: &str, args: Vec<&str>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let filepath = format!("src/functions/{}.lua", command.trim());
    let script = fs::read_to_string(&filepath)?;

    let lua = mlua::Lua::new();

    let lua_args = lua.create_table()?;
    for (i, arg) in args.iter().enumerate() {
        lua_args.set(i + 1, *arg)?;
    }
    lua.globals().set("args", lua_args)?;

    lua.load(&script).exec()?;
    Ok(())
}

//main command loop
// it takes input, and if its not exit, it turns it into a vector and runs that into the run lua function, ofc with amazing error handling
fn main() {
    loop {
        print!("HRCL$ ");
        std::io::stdout().flush().expect("Could not flush stdout");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            exit(0);
        }

        let mut parts = parse_args(input);

        if parts.is_empty() {
            continue;
        }

        let command = parts.remove(0);

        if let Err(e) = run_lua(command, parts) {
            eprintln!("error: {}", e);
        }
    }
}

//thats the main.rs file, as of right now its just a lua file runner
