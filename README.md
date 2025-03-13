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

### What worked?

### What didn't work?

### What would I change in the future?


## License Info
Licensed under MIT/Apache
