use std::{
    io,
    io::Write,
    process::Command
};

fn get_input(query: &str) -> String {
    let mut input = String::new();

    print!("{}", query);

    io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    return input
        .trim()
        .to_owned();
}

fn main() {
    loop {
        let input = get_input("> ");
        let mut input_parts = input
            .trim()
            .split_whitespace();
        let cmd = input_parts
            .next()
            .unwrap();

        Command::new(cmd)
            .args(input_parts)
            .spawn()
            .unwrap()
            .wait();
    }
}

