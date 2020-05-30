use crate::Language::{English, Spanish, French};

fn main() {
    println!("{}", greet(English,None));
    println!("{}", greet(Spanish,Option::from("Pepper")));
    println!("{}", greet(French,Option::from("Lisa")));
}

// refactor to and talk about: type safety, descriptiveness, caller doesn't need magic strings
enum Language {
    English,
    Spanish,
    French,
}

// refactor to and talk about: constants
const DEFAULT_GREET: &str = "World";

// refactor to and talk about: option
fn greet(lang: Language, name: Option<&str>) -> String {
    // type safety around pattern matching
    let greeting = match lang {
        Language::English => "Hello",
        Language::Spanish => "Hola",
        Language::French => "Bonjour",
    };
    // formatting strings, dealing with Some vs None
    format!("{0}, {1}!", greeting, name.unwrap_or(DEFAULT_GREET))
}

// talk about: separate modules for tests so they're not compiled into the program
#[cfg(test)]
mod tests {
    use super::greet;
    use crate::Language::{Spanish, English, French};

    #[test]
    fn test_greet() {
        assert_eq!("Hello, Chris!", greet(English, Option::from("Chris")));
        assert_eq!("Hola, Dave!", greet(Spanish, Option::from("Dave")));
        assert_eq!("Bonjour, Ruth!", greet(French, Option::from("Ruth")));
    }
}


/* notes
- Hooray for option back in my life
- str vs String eugh, https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
- str = immutable, fixed length string in memory
- String = growable, on heap

You can only ever interact with str as a borrowed type aka &str. This is called a string slice, an immutable view of a string. This is the preferred way to pass strings around, as we shall see.

_The only real use case I can think of is if you want to pass a mutable reference to a function that needs to modify the string_:
 */