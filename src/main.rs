fn main() {
    println!("{}", get_message(None));
}

fn get_message(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, world!".to_string(),
    }
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
        assert_eq!("Hello, Bob!", &get_message(Some("Bob")));
    }
}