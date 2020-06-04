# notes

usual stack heap la de da should be discussed (or just link to Rust book)

`str` = always lives on stack, immutable (fixed size suitable for stack), known at compile time

Problem is you don't always know what strings go into your program (user input, API calls, etc etc)

`String`, stored on heap. Suitable for strings unknown at compile time

`let s = String::from("hello");`