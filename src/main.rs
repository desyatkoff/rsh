use std::{
    io,
    io::Write,
    process::Command
};
use whoami;

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
        let username = whoami::username();
        let hostname = whoami::hostname();
        let input = get_input(
            &format!(
                "{}@{} > ",
                &username,
                &hostname,
            )[..]
        );
        let mut input_parts = input
            .trim()
            .split_whitespace();
        let cmd = input_parts
            .next()
            .unwrap();
        let exec_cmd = Command::new(cmd)
            .args(input_parts)
            .spawn();

        match exec_cmd {
            Ok(mut exec_cmd) => {
                exec_cmd.wait();
            },
            Err(e) => {
                eprintln!("{}", e);
            },
        }
    }
}

