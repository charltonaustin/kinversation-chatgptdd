fn main() {
    println!("{}", get_message());
}

fn get_message() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!("Hello, world!", &get_message());
    }
}