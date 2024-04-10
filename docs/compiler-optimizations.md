# YuvaKriti Compiler Optimizations

This document describes the optimizations performed by the compiler during the compilation process. The the name of the
compiler feature that can be enabled/disabled at compile-time is mentioned wherever applicable.
The `-e/--enable-features` and `-d/--disable-features` argument can be passed to the compiler to enable or disable
possible optimizations. Multiple feature names can be provided by separating them with `,`,

## Constant folding

_Feature: const-folding_

Expressions which produce a constant value are evaluated and the result is directly
written to the bytecode, hence reducing the number of instructions that need to be processed at runtime :

| Node types        | Description                                                                                                                                                                                       |
|-------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Binary expresions | If the the left and right operands are numbers (`1+2`), the expresion is evaluated and written to the constant pool. At runtime, this constant is loaded instead of evaluating binary expression. |

## Operand stack size computation

For `Code` attributes (both top-level or method-level), the maximum depth of the operand stack at any point
during the instruction execution, is calculated at compile-time. This makes it possible to allocate the operand stack of
exactly the required size.
