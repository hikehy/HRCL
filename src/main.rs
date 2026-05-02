use mlua::{self};
use std::fs;
use std::io::Write;
use std::process::exit;

fn parse_args(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

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
