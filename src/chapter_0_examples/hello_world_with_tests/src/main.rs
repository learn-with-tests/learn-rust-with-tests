fn greet() -> String {
    String::from("Hello, World!")
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}
