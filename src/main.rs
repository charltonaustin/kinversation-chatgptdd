use std::env;
use grrs::empty_board;
fn main() {
    println!("{}", get_message(get_command_lin_args(env::args().collect())));
    println!("{}", empty_board());
}

fn get_message(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, world!".to_string(),
    }
}

fn get_command_lin_args(args: Vec<String>) -> Option<String> {
    args.into_iter().nth(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!("Hello, world!", &get_message(None));
    }

    #[test]
    fn hello_bob() {
        assert_eq!("Hello, Bob!", &get_message(Some("Bob".to_string())));
    }

    #[test]
    fn get_command_line_args_with_value() {
        let args = vec!["program".to_string(), "Bob".to_string()];
        assert_eq!(Some("Bob".to_string()), get_command_lin_args(args));
    }
    #[test]
    fn get_command_line_args_without_value() {
        let args = vec!["program".to_string()];
        assert_eq!(None, get_command_lin_args(args));
    }
}