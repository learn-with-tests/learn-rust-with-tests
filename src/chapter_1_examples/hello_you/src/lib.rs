fn greet() -> String {
    String::from("Hello, World!")
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, World!", greet());
    }
}