# do-compiler
Experimenting with creating a data oriented compiler, inspired by [this talk](https://youtu.be/ZI198eFghJk) about Carbon.

As the current purpose of this compiler implementation is mostly to learn, this is a "half-blind" implementation.
This means that features are first implemented using as little research as possible to begin with, to as high of a standard as I'm currently able.
I then research and implement more standard techniques, with my prior implementation serving as a comparison point for my own learning.

Despite being a basis to learn about compilers, this project is intended to seriously strive to reach a pair of goals:

1. Compilation should be fast.
    - Specifically I'm aiming for the same average runtime goals as mentioned in [the above talk](https://youtu.be/ZI198eFghJk) of:
      - Lexing and parsing at 100 ns/line.
      - Semantic analysis at 1 μs/line.
2. Compilation should be descriptive. Error diagnostics should be clear, reference related code as precisely as possible, and suggest context sensitive helpful changes.

While inspiration will inevitably be taken from Carbon's toolchain, I expect this project to diverge in approach with time as the above goals are reconsidered, and I learn about other language/compiler's approaches (eg. Zig).

Currently there is no specific language syntax being implemented, and this is intentional.
I expect that this approach to writing a compiler will impose restrictions on, or benefit greatly from language design that is aware of compiler internals.
Currently the syntax is being heavily inspired by Rust, Kotlin and Crystal.
