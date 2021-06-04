<h1 align="center">
😱🤣-rs
</h1>

<h6 align="center">
A Rust-based interpreter for the Omgrofl programming language
</h6>

## Running

```bash
$ cargo run examples/fizzbuzz.omgrofl
```

See other examples in the folder alongside what I
_think_ is the first and only implementation of
FizzBuzz in Omgrofl!

## Advanced usage

While writing a program it might be handy to dump
out the variable/stack state or the syntax tree.

There are environment variables for doing just that

```bash
$ DUMP_VARS=1 DUMP_AST=1 cargo run examples/fizzbuzz.omgrofl
```

## About

This will be one of the last remaining interpreters for omgrofl. To my knowledge there is only **one** other surviving mostly complete implementation [here](https://github.com/OlegSmelov/omgrofl-interpreter).

I would highly recommend writing another one yourself - it's a fun little language :)
