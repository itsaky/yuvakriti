# Opcodes

| Symbol | Size in bits |
|--------|--------------|
| `u1`   | 8            |
| `u2`   | 16           |
| `u4`   | 32           |

## `add`

Add two values.

| **_add_**     | Description                                                                                                                         |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, adds them, and pushes the result back onto the operand stack.                               |
| Operands      | None                                                                                                                                |
| Forms         | _add_ = 0x02                                                                                                                        |
| Operand stack | `..., value1, value2 -> ..., (value1 + value2)`                                                                                     |
| Description   | The `add` instruction pops the top two values from the operand stack, adds them, and pushes the result back onto the operand stack. |

## `bpush_0`

Push the boolean `false` to the operand stack.

| **_bpush_0_** | Description                                                                |
|---------------|----------------------------------------------------------------------------|
| Operation     | Pushes the boolean `false` to the operand stack.                           |
| Operands      | _None_                                                                     |
| Forms         | _bpush_0_ = 0x14                                                           |
| Operand stack | `... -> ..., false`                                                        |
| Description   | The `bpush_0` instruction pushes the boolean `false` to the operand stack. |

## `bpush_1`

Push the boolean `true` to the operand stack.

| **_bpush_1_** | Description                                                               |
|---------------|---------------------------------------------------------------------------|
| Operation     | Pushes the boolean `true` to the operand stack.                           |
| Operands      | _None_                                                                    |
| Forms         | _bpush_1_ = 0x15                                                          |
| Operand stack | `... -> ..., true`                                                        |
| Description   | The `bpush_1` instruction pushes the boolean `true` to the operand stack. |

## `div`

Divide two values.

| **_div_**     | Description                                                                                                                                                           |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, divides the first by the second, and pushes the result back onto the operand stack.                                           |
| Operands      | None                                                                                                                                                                  |
| Forms         | _div_ = 0x05                                                                                                                                                          |
| Operand stack | `..., value1, value2 -> ..., (value1 / value2)`                                                                                                                       |
| Description   | The `div` instruction pops the top two values from the operand stack, divides the first value by the second value, and pushes the result back onto the operand stack. |

## `halt`

Halt the program execution.

| **_ldc_**     | Description                                                              |
|---------------|--------------------------------------------------------------------------|
| Operation     | Halts the program execution.                                             |
| Operands      | _None_                                                                   |
| Forms         | _halt_ = 0x01                                                            |
| Operand stack | `... ->`                                                                 |
| Description   | The execution of the program is halted and the resources are cleaned up. |

## `if<cond>`

The `if<cond>` instruction variants are conditional jumps which are used to jump to a specified instruction address if
the top of the stack is truthy or falsy.

| **_if&lt;cond&gt;_** | Description                                                                                                                                                                                                                                                          |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation            | Jump to the specified instruction address.                                                                                                                                                                                                                           |
| Operands             | `i2 address`                                                                                                                                                                                                                                                         |
| Forms                | _iftruthy_ = 0x20 <br> _iffalsy_ = 0x21                                                                                                                                                                                                                              |
| Operand stack        | `... -> ...`                                                                                                                                                                                                                                                         |
| Description          | The operand at the top of the stack is checked for truthy-ness or falsy-ness, **WITHOUT** a pop operation on the stack. If the condition is satisfied, the VM jumps to the instruction specified by `address` and the program resumes at instruction `pc + address`. |

## `if<cmp>`

The `if<cmp>` instruction is used to compare the two operands at the top of the stack.

| **_if&lt;cmp&gt;_** | Description                                                                                                                                                                                                                                                                                       |
|---------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation           | Compare operands and push `true` or `false` to the operand stack.                                                                                                                                                                                                                                 |
| Operands            | `i2 address`                                                                                                                                                                                                                                                                                      |
| Forms               | _ifeq_ = 0x07 <br> _ifne_ = 0x09 <br> _iflt_ = 0x0B <br> _ifle_ = 0x0D <br> _ifgt_ = 0x0F <br> _ifge_ = 0x11                                                                                                                                                                                      |
| Operand stack       | `..., value1, value2 -> ..., result`                                                                                                                                                                                                                                                              |
| Description         | Two operands are popped from the top of the stack are compared with each other and the result of the comparison is pushed to the stack. If the comparison succeeds, then the VM increments the program counter with the value of `address` and the program resumes at instruction `pc + address`. |

## `if<cmp>z`

The `if<cmp>z` instruction is used to compare the operand at the top of the stack with `0`.

| **_if&lt;cmp&gt;z_** | Description                                                                                                                                                                                                                                                                     |
|----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation            | Compare operand with `0` and push `true` or `false` to the operand stack.                                                                                                                                                                                                       |
| Operands             | `i2 address`                                                                                                                                                                                                                                                                    |
| Forms                | _ifeqz_ = 0x08 <br> _ifnez_ = 0x0A <br> _ifltz_ = 0x0C <br> _iflez_ = 0x0E <br> _ifgtz_ = 0x10 <br> _ifgez_ = 0x12                                                                                                                                                              |
| Operand stack        | `..., value -> ..., result`                                                                                                                                                                                                                                                     |
| Description          | Operand at the top of the stack is popped, compared with `0` and the result of the comparison is pushed to the stack. If the comparison succeeds, then the VM increments the program counter with the value of `address` and the program resumes at instruction `pc + address`. |

## `jmp`

Unconditional jump instruction.

| **_jmp_**     | Description                                                                                                              |
|---------------|--------------------------------------------------------------------------------------------------------------------------|
| Operation     | Unconditional jump to the specified instruction address.                                                                 |
| Operands      | `i2 address`                                                                                                             |
| Forms         | _jmp_ = 0x22                                                                                                             |
| Operand stack | `... -> ...`                                                                                                             |
| Description   | The VM increments the program counter with the value of `address` and the program resumes at instruction `pc + address`. |

## `ldc`

Loads a constant.

| **_ldc_**     | Description                                                                                                                              |
|---------------|------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Loads a constant from the constant pool to the operand stack.                                                                            |
| Operands      | `u2 constant_pool_index`                                                                                                                 |
| Forms         | _ldc_ = 0x13                                                                                                                             |
| Operand stack | `... -> ..., value`                                                                                                                      |
| Description   | The `ldc` instruction loads the constant at index `constant_pool_index` from the runtime constant pool of the VM into the operand stack. |

## `load`

The `load` instruction is used to load the value of a variable at a specified index onto the operand stack.

| **_load_**    | Description                                                                                                 |
|---------------|-------------------------------------------------------------------------------------------------------------|
| Operation     | Loads the value of the variable at the specified index onto the operand stack.                              |
| Operands      | `u2 var_index`                                                                                              |
| Forms         | _load_ = 0x1B                                                                                               |
| Operand stack | `... -> ..., value`                                                                                         |
| Description   | The `load` instruction loads the value of the variable at `var_index` and pushes it onto the operand stack. |

## `load_<n>`

These instructions are variants of `load` with a fixed, constant index value.

| **_load_&lt;n&gt;_** | Description                                                                                                                                   |
|----------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| Operation            | Loads the value of the variable at the specified constant index onto the operand stack.                                                       |
| Operands             | _None_                                                                                                                                        |
| Forms                | _load_0_ = 0x1C <br> _load_1_ = 0x1D <br> _load_2_ = 0x1E <br> _load_3_ = 0x1F                                                                |
| Operand stack        | `... -> ..., value`                                                                                                                           |
| Description          | The `load_<n>` instruction loads the value of the variable at index `n` and pushes it onto the operand stack. The index can be 0, 1, 2, or 3. |

## `mult`

Multiply two values.

| **_mult_**    | Description                                                                                                                                |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, multiplies them, and pushes the result back onto the operand stack.                                |
| Operands      | None                                                                                                                                       |
| Forms         | _mult_ = 0x04                                                                                                                              |
| Operand stack | `..., value1, value2 -> ..., (value1 * value2)`                                                                                            |
| Description   | The `mult` instruction pops the top two values from the operand stack, multiplies them, and pushes the result back onto the operand stack. |

## `nop`

Reserved, no-op opcode.

| **_ldc_**     | Description   |
|---------------|---------------|
| Operation     | Does nothing. |
| Operands      | _None_        |
| Forms         | _nop_ = 0x00  |
| Operand stack | `... -> ...`  |
| Description   | Does nothing. |

## `pop`

Pops the operand at the top of the stack.

| **_pop_**     | Description                               |
|---------------|-------------------------------------------|
| Operation     | Pops the stack.                           |
| Operands      | _None_                                    |
| Forms         | _pop_ = 0x23                              |
| Operand stack | `..., value -> ...`                       |
| Description   | Pops the operand at the top of the stack. |

## `print`

Print a value.

| **_print_**   | Description                                                                      |
|---------------|----------------------------------------------------------------------------------|
| Operation     | Pops a value from the operand stack and prints it.                               |
| Operands      | None                                                                             |
| Forms         | _print_ = 0x06                                                                   |
| Operand stack | `..., value -> ...`                                                              |
| Description   | The `print` instruction pops the top value from the operand stack and prints it. |

## `store`

The `store` instruction is used to store a value from the operand stack into a variable at a specified index.

| **_store_**   | Description                                                                                                     |
|---------------|-----------------------------------------------------------------------------------------------------------------|
| Operation     | Removes a value from the operand stack and stores it in the variable at the specified index.                    |
| Operands      | `u2 var_index`                                                                                                  |
| Forms         | _store_ = 0x16                                                                                                  |
| Operand stack | `..., value -> ...`                                                                                             |
| Description   | The `store` instruction pops the top value from the operand stack and stores it in the variable at `var_index`. |

## `store_<n>`

These instructions are variants of `store` with a fixed, constant index value.

| **_store_&lt;n&gt;_** | Description                                                                                                                                       |
|-----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation             | Removes a value from the operand stack and stores it in the variable at the specified constant index.                                             |
| Operands              | _None_                                                                                                                                            |
| Forms                 | _store_0_ = 0x17 <br> _store_1_ = 0x18 <br> _store_2_ = 0x19 <br> _store_3_ = 0x1A                                                                |
| Operand stack         | `..., value -> ...`                                                                                                                               |
| Description           | The `store_<n>` instruction pops the top value from the operand stack and stores it in the variable at index `n`. The index can be 0, 1, 2, or 3. |

## `sub`

Subtract two values.

| **_sub_**     | Description                                                                                                                                                               |
|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, subtracts the second from the first, and pushes the result back onto the operand stack.                                           |
| Operands      | None                                                                                                                                                                      |
| Forms         | _sub_ = 0x03                                                                                                                                                              |
| Operand stack | `..., value1, value2 -> ..., (value1 - value2)`                                                                                                                           |
| Description   | The `sub` instruction pops the top two values from the operand stack, subtracts the second value from the first value, and pushes the result back onto the operand stack. |
