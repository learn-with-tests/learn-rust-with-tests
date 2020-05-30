fn main() {
    println!("{}", greet(None))
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        let name = "Chris";
        assert_eq!("Hello, Chris!", greet(Option::from(name)))
    }
}

const DEFAULT_GREET: &str = "world"; //todo: work into constant

fn greet(name: Option<&str>) -> String {
    format!("Hello, {0}!", name.unwrap_or(DEFAULT_GREET))
}

/* notes
- Hooray for option back in my life
- str vs String eugh, https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
- str = immutable, fixed length string in memory
- String = growable, on heap

You can only ever interact with str as a borrowed type aka &str. This is called a string slice, an immutable view of a string. This is the preferred way to pass strings around, as we shall see.

_The only real use case I can think of is if you want to pass a mutable reference to a function that needs to modify the string_:
 */