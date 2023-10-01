# Chapter 1: Hello, You!

In the previous chapter we wrote a greet function and _then_ wrote the tests. 

```rust
{{#include ./chapter_1_examples/hello_you/src/greet.rs}}
```

From this point on we will be writing tests first.

Our next requirement is to let us specify the recipient of the greeting.

Let's start by capturing these requirements in a test. This is basic test driven development and allows us to make sure our test is actually testing what we want. When you retrospectively write tests there is the risk that your test may continue to pass even if the code doesn't work as intended.

## Write the test first

```rust
fn greet() -> String {
    String::from("Hello, World!")
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet_with_name() {
        let got = greet(String::from("Chris"));
        let want = "Hello, Chris!";

        assert_eq!(got, want);
    }
}
```

Now run `cargo test`, you should have a compilation error

```shell
error[E0061]: this function takes 0 arguments but 1 argument was supplied
--> src/greet_with_name.rs:11:19
   |
11 |         let got = greet(String::from("Chris"));
   |                   ^^^^^ ---------------------
   |                         |
   |                         unexpected argument of type `String`
   |                         help: remove the extra argument
   |
note: function defined here
--> src/greet_with_name.rs:1:4
   |
1  | fn greet() -> String {
   |    ^^^^^
```

When using a statically typed language like Rust it is important to listen to the compiler. The compiler understands how your code should snap together and work so you don't have to.

In this case the compiler is telling you what you need to do to continue. We have to change our function `greet` to accept an argument.

Edit the `greet` function to accept an argument of type `String`.

```rust
fn greet(name: String) -> String {
    String::from("Hello, World!")
}
```

Now when you run your tests you should see something like

```shell
failures:

---- greet_with_name::tests::test_greet_with_name stdout ----
thread 'greet_with_name::tests::test_greet_with_name' panicked at 'assertion failed: `(left == right)`
  left: `"Hello, World!"`,
 right: `"Hello, Chris!"`', src/greet_with_name.rs:14:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    greet_with_name::tests::test_greet_with_name

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

We finally have a compiling program but it is not meeting our requirements according to the test.

Let's make the test pass by joining the name argument with the rest of our greeting

```rust
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

We learned about the `println!` macro in Chapter 0. Like `println!`, `format!` takes a format string literal as the first argument, followed by zero or more values that are used by that format string. In our case, the format string contains `{}` to embed the `name` argument formatted simply as a string. Unlike `println!`, `format!` does not print the result, but instead returns it.

When you run the tests they should now pass. Normally as part of the TDD cycle we should now refactor.

## A note on source control
At this point, if you are using source control (which you should!) I would commit the code as it is. We have working software backed by a test.

I wouldn't push to master though, because I plan to refactor next. It is nice to commit at this point in case you somehow get into a mess with refactoring - you can always go back to the working version.

There's not a lot to refactor here, but we can introduce another language feature, constants.

## Try to run the test
## Write the minimal amount of code for the test to run and check the failing test output
## Write enough code to make it pass
## Refactor
