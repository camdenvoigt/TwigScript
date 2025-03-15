# TwigScript

Author: Camden Voigt

## What is Twigscript

Twigscript is a super simple interpretor with the ability to do simple calculations on integers, boolean comparisons, and write strings. Twigscript can also store the results of these computations in named variables that can be accessed later to be used in more computations.

Twigscript's lexer and parser are made using the Pest crate (https://github.com/pest-parser/pest) and a PEG grammar. The parsed input is than made into a simple AST and interpreted to provide an output.

## How to run

1. clone the project
`git clone https://github.com/camdenvoigt/TwigScript`

2. In cloned directory run cargo build
`cargo build --release`

3. Run the compiled code
`./target/release/twigscript`

**OR**

`git clone https://github.com/camdenvoigt/TwigScript`

2. In cloned directory run cargo run
`cargo run --release`

## How to Use
After running the Twigscript interpretor you will be shown a prompt symbol `>` after seeing this you can type any valid Twigscript (see below) press enter and the return value should be shown. If you provide invalid Twigscript or you run into a runtime error it will be shown instead. Type `exit` to quit.

## Questions

### How was testing done
There are unit tests to make sure the basic functionality works as expected. These could be even more though and probably should be as even basic functionality like `>=` needs to always work in a programming language. I also did a lot of manual testing running twigscript and writing programs.

### What worked?
Honestly, most of what I tried worked. I especially liked using rust Enum types and match statements. They work really well for a problem like this. The enum types just really felt like a natural way to express an AST and then parse an AST. 

### What didn't work?
Using a 3rd party parser was tough. It was hard to get exactly what I wanted how I wanted it. It did help simplify the problem, but a lot of my debugging time was working with the grammar to make sure things were parsed the way I wanted rather than solving the problems.

### What would I change in the future?
So much. I would love to add all the basic programming language stuff. Control flow like loops and ifs. I'd love to add simple javascript like objects. For changes to existing code. Right now all variables are stored on the heap, even simple ones and I'd like to find a way to change that. Part of the problem is currently there is no real idea of a stack right now.

Also, if I got to do a full rewrite I would probably not specify my parser in PEG. Near the end I discovered an example of how to write a pest parser without using the PEG to generate the rules. This looked a lot more flexible and easy to use as it would just be writing rust code.

## License Info
Licensed under MIT/Apache

## Bibliography
### basic
  - Rust Programming Docs (https://doc.rust-lang.org/std/index.html)
  - Rust Programming Book (https://doc.rust-lang.org/book/title-page.html)
  - Programming Rust (Jim Blandy)
  - Pest Docs (https://docs.rs/pest/latest/pest/index.html)
  - Pest Book (https://pest.rs/book/intro.html)
  - Simple but Powerful Pratt Parsing (https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)

### Stack Overflow
- https://stackoverflow.com/questions/53124930/how-do-you-test-for-a-specific-rust-error/53124931#53124931
- https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests
- https://stackoverflow.com/questions/22355273/writing-to-a-file-or-stdout-in-rust
- https://stackoverflow.com/questions/30416320/why-is-a-lifetime-needed-when-implementing-a-trait-on-a-reference-type-if-the-li
- https://stackoverflow.com/questions/9109872/how-do-you-access-enum-values-in-rust
- https://stackoverflow.com/questions/13579266/how-can-i-read-user-input-in-rust
- https://stackoverflow.com/questions/55461617/how-do-i-convert-a-boolean-to-an-integer-in-rust

### Reddit
- https://www.reddit.com/r/rust/comments/gmjgr2/newbie_question_asserting_errors_in_rust/

### Rust Forums
- https://users.rust-lang.org/t/dereferencing-a-boxed-value/86768
- https://users.rust-lang.org/t/is-the-lifetime-of-a-btreemap-get-result-attached-to-the-key-as-well-as-the-map/83568
- https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665/4
- https://users.rust-lang.org/t/why-cant-move-element-of-vector/30454
