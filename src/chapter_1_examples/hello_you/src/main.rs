fn greet() -> String {
    String::from("Hello, World!")
}

fn greet_recipient(recipient: &str) -> String {
    format!("Hello, {}!", recipient)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, World!", greet());
    }

    #[test]
    fn test_greet_recipient() {
        assert_eq!("Hello, Earth!", greet_recipient("Earth"));
        assert_eq!("Hello, Chris!", greet_recipient("Chris"));
        assert_eq!("Hello, Dave!", greet_recipient("Dave"));
        assert_eq!("Hello, Ruth!", greet_recipient("Ruth"));
    }
}
