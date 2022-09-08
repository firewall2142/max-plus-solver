# Introduction
This is an implementation of the algorithm in [[1]](#1) (see also [[2]](#2)). This library contains two solvers
- `solve` : To solve A⊗x=B⊗x
- `cgb_solve` : To solve A⊗x=B⊗y

# Usage
Two options:
  - Directly: `cargo run -r [INPUT_FILE]` 
  - Build and use the binary:
      1. `cargo build --release`
      2. `./target/release/mpsolver.exe [INPUT_FILE]`

## Input Format

An example of input file is
```
x0-1, x1+1 = x2
x2 + 1, x3 = x0 + 2, x0 - 1
```
it corresponds to the equation:
```
max(x0 - 1, x1 + 1) = max(x2)
max(x2 + 1, x3) = max(x0 + 2, x0 - 1)
```

You can use variable names that don't have `+ - =`.

Another example:
```
foo + 1 = bar - 1
foo - 1, baz = bar + 13, thud + 99
qux = waldo + 1, thud
```

# References
<a id="1">[1]</a> Cuninghame-Green, R.A., and P. Butkovic. “The Equation A⊗x=B⊗y over (Max,+).” Theoretical Computer Science 293, no. 1 (February 2003): 3–12. https://doi.org/10.1016/S0304-3975(02)00228-1.

<a id="2">[2]</a> Aminu, Abdulhadi, and P Butkoviˇc. “Comparison of Methods for Solving Two-Sided Systems in Max-Algebra,” n.d., 7.