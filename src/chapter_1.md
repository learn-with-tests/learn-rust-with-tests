# Chapter 1: Hello, You!

In the previous chapter we wrote a greet function and _then_ wrote the tests.

```rust
{{#include ./chapter_0_examples/hello_world_with_tests/src/main.rs}}
```

From this point on we will be writing tests first.

Our next requirement is to let us specify the recipient of the greeting.

Let's start by capturing these requirements in a test. This is basic test driven development and allows us to make sure our test is actually testing what we want. When you retrospectively write tests there is the risk that your test may continue to pass even if the code doesn't work as intended.

## Write the test first

Add another test within the `tests` module using the `#[test]` annotation that expects a recipient of the greeting:

```rust
#[test]
fn test_greet_recipient() {
    assert_eq!("Hello, Lisa!", greet_recipient("Lisa"));
}
```

## Try to run the test

```rust
error[E0425]: cannot find function `greet_recipient` in this scope
  --> src/main.rs:20:37
   |
20 |         assert_eq!("Hello, Earth!", greet_recipient("Earth"));
   |                                     ^^^^^^^^^^^^^^^ not found in this scope
```

Well, this makes sense. The method has not yet been defined. Add the method with one
expected parameter, but do not even use it yet. After all, this is the step where
we are just trying to run the test and defining the method should suffice, right?:

```rust
error[E0425]: cannot find function `greet_recipient` in this scope
  --> src/main.rs:24:37
   |
24 |         assert_eq!("Hello, Earth!", greet_recipient("Earth"));
   |                                     ^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
15 +     use crate::greet_recipient;
```

What is going on here. Since `tests` is a separate module, the methods available to it
must be defined explicitly. Instead of importing the function as the compiler recommends,
it is common practice to import all methods from the parent module into the `tests` module:

```rust
use super::*;

## Write the minimal amount of code for the test to run and check the failing test output

The test compiled and ran! Checking the output we see the following:

```rust
running 2 tests
test tests::test_greet ... ok
test tests::test_greet_recipient ... FAILED

failures:

---- tests::test_greet_recipient stdout ----
thread 'tests::test_greet_recipient' panicked at src/main.rs:24:9:
assertion `left == right` failed
  left: "Hello, Earth!"
 right: "Hello, World!"
```

This is what was expected, since the recipient was not even used. In fact, the Rust
compiler even mentioned this at the beginning of the test output:

```rust
warning: unused variable: `recipient`
 --> src/main.rs:5:20
  |
5 | fn greet_recipient(recipient: &str) -> String {
  |                    ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_recipient`
  |
  = note: `#[warn(unused_variables)]` on by default
```

## Write enough code to make it pass

Instead of using recipient, the test can be made to pass by hardcoding the expected value:

```rust
fn greet_recipient(recipient: &str) -> String {
    String::from("Hello, Earth!")
}
```

Okay, sure, the test passes since the value was hardcoded. This calls for adding some other test cases:

```rust
#[test]
fn test_greet_recipient() {
    assert_eq!("Hello, Earth!", greet_recipient("Earth"));
    assert_eq!("Hello, Chris!", greet_recipient("Chris"));
}
```

The test now fails as expected:

```rust
running 2 tests
test tests::test_greet ... ok
test tests::test_greet_recipient ... FAILED

failures:

---- tests::test_greet_recipient stdout ----
thread 'tests::test_greet_recipient' panicked at src/main.rs:25:9:
assertion `left == right` failed
  left: "Hello, Chris!"
 right: "Hello, Earth!"
```

Okay, let's get real. As mentioned earlier, Rust has a concept called
macros that can be used to generate code itself. One such macro is
[`format!`](https://doc.rust-lang.org/std/macro.format.html)
which can use the same interface as `println!` but can return a value:

```rust
fn greet_recipient(recipient: &str) -> String {
    String::from(format!("Hello, {}!", recipient))
}
```

Running the tests shows they pass:

```rust
running 2 tests
test tests::test_greet ... ok
test tests::test_greet_recipient ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Refactor

There isn't too much refactoring to be done here, but it can't hurt to split things up some:

```rust
fn greet_recipient(recipient: &str) -> String {
    let output = format!("Hello, {}!", recipient);
    String::from(output)
}
```

Finally, some additional test cases can be added:

```rust
#[test]
fn test_greet_recipient() {
    assert_eq!("Hello, Earth!", greet_recipient("Earth"));
    assert_eq!("Hello, Chris!", greet_recipient("Chris"));
    assert_eq!("Hello, Dave!", greet_recipient("Dave"));
    assert_eq!("Hello, Ruth!", greet_recipient("Ruth"));
}
```

The tests all pass.

There is one small, final refactor that can be completed. In chapter 0 the usage of semicolons
was mentioned briefly, along with the special reason that they can sometimes be omitted. When
a semicolon is omitted Rust is defining an **expression**. When semicolons are used, a **statement**
is being used. In the method currently, the line defining `output` is a statement while the final
`output` call is an expression that is returning the `String`.

The statement that generates the string can be in-lined with the expression that returns it, making
the method a little more concise:

```rust
fn greet_recipient(recipient: &str) -> String {
    format!("Hello, {}!", recipient)
}
```
