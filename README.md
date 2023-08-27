# Symbolic math library in rust

A library for symbolic mathematical expressions and basic operations with code generation to C.

## Goal

This project is mainly for personal learning purposes to get more familiar with the rust programming language, symbolic mathematics in software and a bit of code generation.
It is inspired by [sympy](https://github.com/sympy/sympy).

## Todos

- [x] Expression modeling
- [x] Expression evaluating
    - [x] primitives
    - [x] anonymous funtions
- [x] Expression transforming
    - [x] Derivative
    - [x] Generic expression transformation
- [x] Code generation
    - [x] C

## Future features

- Structures
    - Complex numbers
    - Quaternions
    - Vectors
    - Matrices
    - n-dimensional tensors[^1]
- Expression simplification
- Common subexpression elimination

[^1]: Very unlikely since I never needed them for my use cases (Mechanical engineering problems).