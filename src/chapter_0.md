# Chapter 0: Hello, world!

In this chapter, as is traditional, we'll introduce Rust by printing "Hello,
world!" to the screen. Along the way we'll look at how to set up a Rust project
using Cargo, what the files

Let's start a new Rust project. If we've already installed Rust and Cargo, this
is as simple as

```sh
$ cargo init hello_world
```

at the command line.

This will create a new directory, `hello_world`. The structure within this
folder looks like this:

```console
hello_world
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` contains metadata about the project:

```toml
{{#include ./chapter_0_examples/hello_world/Cargo.toml:1:5}}
```

This includes information like the name of the package, the authors, which
version of Rust is being used, and any external Rust libraries - crates - that
we're using. We'll look at that in a bit more detail later.

The `src` directory contains all the Rust code that we're going to write - the
source code! And the `main.rs` file inside `src` contains our as yet unwritten
program. Cargo puts a dummy program in there to get us started.

```rust
{{#include ./chapter_0_examples/hello_world/src/main.rs}}
```

Well, blimey! Looks like Cargo has done all the hard work for us.

To run this program we need to _compile_ the Rust code into something that our
computer can execute - a file called a _binary_ - using a program called
a _compiler_.

Cargo provides a nice interface to the Rust compiler through the command

```sh
$ cargo build
```

when we run that inside the `hello_world` directory we see some output:

```sh
Compiling hello_world v0.1.0 (/Users/davidwic/dev/personal/...
 Finished dev [unoptimized + debuginfo] target(s) in 0.40s
```

and a new directory called `target` should appear in your project directory.
This directory contains all the results of compiling your very simple program -
a surprisingly large number of files which are 'intermediate' steps, as well as
a binary file called, unsurprisingly, `hello_world`.

To run our program we need to execute the binary. We can do this on the command
line like so:

```sh
$ ./target/debug/hello_world
```

and we should then see:

```sh
Hello, world!
```

Great success.

To save performing all of the steps above, Cargo provides a command to both
compile and run a program - which is often what we want to do:

```sh
$ cargo run
```

Which should print out.

```sh
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_world`
Hello, world!
```

## But where are the tests?

This is, after all, _Learn Rust With Tests_. Where are they?

...

If we want to test the program that ~we~ Cargo wrote for us, we need something
to test. This might sound obvious, but at the moment we don't really have
something that we can have the computer test for us _automatically_. Our program
is one, admittedly small, lump:

```rust
{{#include ./chapter_0_examples/hello_world/src/main.rs}}
```

The first line we can see creates a new _function_ called `main`. The `fn`
keyword declares the function, then we can see its name (`main`), and then we
get a pair of paretheses where we'd put the names of the _arguments_ that the
function recieves - think of these as the input. It's empty because main doesn't
take any inputs!

The `main` function in Rust is special as it's the 'entry point' for any Rust
program. When your compiled program runs it starts by running the `main`
function - this is sometimes called 'calling' or 'executing' the function.

Then there's a curly brace `{`, which is the beginning of the _function body_.
This is the meat of the function - what actually happens when the function runs.
And in this case:

```rust
{{#include ./chapter_0_examples/hello_world/src/main.rs:2:2}}
```

`println!` is not technically function - but it looks and acts like one! It's
actually a _macro_. We'll look at macros much later. But for now we can think of
it as a function. The function is being called by having a pair of paretheses
after its name.  And it's being called with one argument - one 'thing' inside the parentheses - a _string_ that says `"Hello, world!"`.

Then comes a closing parenthesis and a semicolon (`;`). Lines of Rust code
usually end in a semicolon - there's a special reason for this which we'll see
later. And finally a closing curly brace `}`, which ends the function body.
Whew!

Right now we can only test this program by running it. Which isn't that bad,
really. We can run it quickly and as often as we like, and we can read the
output to see that it says what we expect.

We could even write an automated test in another language - something that runs
the program and compares what it outputs to what we expect. This type of test is
sometimes called an _acceptance test_. We will look at those later too.

We can only test our program this way because the output is the only _interface_
we have access to in the program.

At the moment we're trying to test the steering wheel of a car by watching
somebody else drive the car. Wouldn't it be easier if we just got inside the car
and turned the wheel ourself?

To do this we're going to introduce another surface - another interface - to our
program. We're going to test it _from the inside_ of our program to check that
it works. Then we're going to use that interface when the program actually runs
in order to print "Hello, world!".

Of course, this doesn't guarantee that our program will work. Just as climbing
inside a stationary car, turning the wheel, and watching the wheels turn doesn't
guarantee that the car will actually turn a corner when it's being driven. But
it does give us some confidence that it will.

## My First Test - Rust Edition

How do you test this? It is good to separate your "domain" code from the outside
world (_side effects_). The `println!` is a side effect (printing to stdout) and
the string we send in is our domain.

So let's separate these concerns so it's easier to test.

```rust
{{#include ./chapter_0_examples/hello_world_with_tests/src/main.rs:1:7}}
```

We have created a new function again with `fn`, but this time we've added the
symbol `->` to introduce the type(s) that the function returns, plus the keyword
`String` meaning our function returns a string (technically transfering
_ownership_ of a heap allocated and growable vector of bytes representing a
valid string encoded as UTF-8, but we'll come back to that!). The last
expression in a function if it doesn't end with a semi-colon is used as the
return value without needing the `return` keyword. This seems a bit random at
first, but it will make more sense as we find out more about the language. For
now we'll stick to this convention for very short functions.

The `println!` macro takes a format string literal as the first argument,
followed by zero or more values that are used by that format string. In our case
the format string contains `{}` to embed the value returned from our `hello`
function formatted simply as a string.

Tests can be added to the same file:

```rust
{{#include ./chapter_0_examples/hello_world_with_tests/src/main.rs:9:18}}
```

Before we delve into what's going on here let's run the tests: run `cargo test`
from your terminal - you should see a successful test run with 1 passing test.

The first line is a directive to the compiler meaning that the following item
will only be compiled when running the tests. That item is a _sub-module_ - a
module within the module defined by our `main.rs` file. We've followed the
convention of calling our sub-module "tests", so it does what it says on the
tin! Next the `use` statement is _used_ to bring the greet function into the
scope of our sub-module.

The function with the `#[test]` annotation is where the testing action begins.
The annotation is used by the test runner to identify which function(s) should
be run as tests. The
[`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html) macro is part
of standard library for testing equality between two values. In this case we are
asserting the actual value returned from our function matches the expected
value.

Notice that when it comes to testing, Rust is a batteries included language -
everything you need to get started is included in the language and standard
tools. Also of note is that while there is a little boilerplate required to
separate your testing code, tests are concise with very little ceremony
required.
