# Chapter 1: Hello, You!

In the previous chapter we wrote a greet function and _then_ wrote the tests. 

```rust
{{#include ./chapter_1_examples/hello_you/src/lib.rs}}
```

From this point on we will be writing tests first.

Our next requirement is to let us specify the recipient of the greeting.

Let's start by capturing these requirements in a test. This is basic test driven development and allows us to make sure our test is actually testing what we want. When you retrospectively write tests there is the risk that your test may continue to pass even if the code doesn't work as intended.

## Write the test first
## Try to run the test
## Write the minimal amount of code for the test to run and check the failing test output
## Write enough code to make it pass
## Refactor