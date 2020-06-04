fn hello() -> String {
    return String::from("Hello, world!");
}

fn main() {
    println!("{}", hello());
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_hello() {
        assert_eq!(super::hello(), "Hello, world!");
    }
    
}