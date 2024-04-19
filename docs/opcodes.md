# Opcodes

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
