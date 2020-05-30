fn main() {
    println!("{}", greet())
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, world!", greet())
    }
}

fn greet() -> String {
    "Hello, world!".to_string()
}