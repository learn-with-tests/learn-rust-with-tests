fn main() {
    println!("{}", greet(String::from("world")))
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!("Hello, Chris!", greet(String::from("Chris")))
    }
}

fn greet(name: String) -> String {
    format!("Hello, {0}!", name)
}