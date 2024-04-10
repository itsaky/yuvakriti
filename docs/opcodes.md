# Opcodes

## `nop`

Reserved, no-op opcode.

| **_ldc_**     | Description   |
|---------------|---------------|
| Operation     | Does nothing. |
| Operands      | _None_        |
| Forms         | _nop_ = 0x00  |
| Operand stack | `... -> ...`  |
| Description   | Does nothing. |

## `halt`

Halt the program execution.

| **_ldc_**     | Description                                                              |
|---------------|--------------------------------------------------------------------------|
| Operation     | Halts the program execution.                                             |
| Operands      | _None_                                                                   |
| Forms         | _halt_ = 0x01                                                            |
| Operand stack | `... ->`                                                                 |
| Description   | The execution of the program is halted and the resources are cleaned up. |

## `add`

Add two values.

| **_add_**     | Description                                                                                                                         |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, adds them, and pushes the result back onto the operand stack.                               |
| Operands      | None                                                                                                                                |
| Forms         | _add_ = 0x02                                                                                                                        |
| Operand stack | `..., value1, value2 -> ..., (value1 + value2)`                                                                                     |
| Description   | The `add` instruction pops the top two values from the operand stack, adds them, and pushes the result back onto the operand stack. |

## `sub`

Subtract two values.

| **_sub_**     | Description                                                                                                                                                               |
|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, subtracts the second from the first, and pushes the result back onto the operand stack.                                           |
| Operands      | None                                                                                                                                                                      |
| Forms         | _sub_ = 0x03                                                                                                                                                              |
| Operand stack | `..., value1, value2 -> ..., (value1 - value2)`                                                                                                                           |
| Description   | The `sub` instruction pops the top two values from the operand stack, subtracts the second value from the first value, and pushes the result back onto the operand stack. |

## `mult`

Multiply two values.

| **_mult_**    | Description                                                                                                                                |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, multiplies them, and pushes the result back onto the operand stack.                                |
| Operands      | None                                                                                                                                       |
| Forms         | _mult_ = 0x04                                                                                                                              |
| Operand stack | `..., value1, value2 -> ..., (value1 * value2)`                                                                                            |
| Description   | The `mult` instruction pops the top two values from the operand stack, multiplies them, and pushes the result back onto the operand stack. |

## `div`

Divide two values.

| **_div_**     | Description                                                                                                                                                           |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, divides the first by the second, and pushes the result back onto the operand stack.                                           |
| Operands      | None                                                                                                                                                                  |
| Forms         | _div_ = 0x05                                                                                                                                                          |
| Operand stack | `..., value1, value2 -> ..., (value1 / value2)`                                                                                                                       |
| Description   | The `div` instruction pops the top two values from the operand stack, divides the first value by the second value, and pushes the result back onto the operand stack. |

## `print`

Print a value.

| **_print_**   | Description                                                                      |
|---------------|----------------------------------------------------------------------------------|
| Operation     | Pops a value from the operand stack and prints it.                               |
| Operands      | None                                                                             |
| Forms         | _print_ = 0x06                                                                   |
| Operand stack | `..., value -> ...`                                                              |
| Description   | The `print` instruction pops the top value from the operand stack and prints it. |

## `if_eq`

Jump if equal.

| **_if_eq_**   | Description                                                                                                                                                                                                                                          |
|---------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for equality, and jumps to the specified address if they are equal.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                          |
| Forms         | _if_eq_ = 0x07                                                                                                                                                                                                                                       |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                         |
| Description   | The `if_eq` instruction pops the top two values from the operand stack, compares them for equality, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_eq` instruction) if the values are equal. |

## `if_ne`

Jump if not equal.

| **_if_ne_**   | Description                                                                                                                                                                                                                                                |
|---------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for inequality, and jumps to the specified address if they are not equal.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                                |
| Forms         | _if_ne_ = 0x08                                                                                                                                                                                                                                             |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                               |
| Description   | The `if_ne` instruction pops the top two values from the operand stack, compares them for inequality, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_ne` instruction) if the values are not equal. |

## `if_lt`

Jump if less than.

| **_if_lt_**   | Description                                                                                                                                                                                                                                                                                   |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for a less-than relationship, and jumps to the specified address if the first value is less than the second.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                                                                   |
| Forms         | _if_lt_ = 0x09                                                                                                                                                                                                                                                                                |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                                                                  |
| Description   | The `if_lt` instruction pops the top two values from the operand stack, compares them for a less-than relationship, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_lt` instruction) if the first value is less than the second value. |

## `if_le`

Jump if less than or equal.

| **_if_le_**   | Description                                                                                                                                                                                                                                                                                                        |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for a less-than-or-equal relationship, and jumps to the specified address if the first value is less than or equal to the second.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                                                                                        |
| Forms         | _if_le_ = 0x0A                                                                                                                                                                                                                                                                                                     |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                                                                                       |
| Description   | The `if_le` instruction pops the top two values from the operand stack, compares them for a less-than-or-equal relationship, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_le` instruction) if the first value is less than or equal to the second value. |

## `if_gt`

Jump if greater than.

| **_if_gt_**   | Description                                                                                                                                                                                                                                                                                         |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for a greater-than relationship, and jumps to the specified address if the first value is greater than the second.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                                                                         |
| Forms         | _if_gt_ = 0x0B                                                                                                                                                                                                                                                                                      |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                                                                        |
| Description   | The `if_gt` instruction pops the top two values from the operand stack, compares them for a greater-than relationship, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_gt` instruction) if the first value is greater than the second value. |

## `if_ge`

Jump if greater than or equal.

| **_if_ge_**   | Description                                                                                                                                                                                                                                                                                                              |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, compares them for a greater-than-or-equal relationship, and jumps to the specified address if the first value is greater than or equal to the second.                                                                                                                            |
| Operands      | `u2 offset`                                                                                                                                                                                                                                                                                                              |
| Forms         | _if_ge_ = 0x0C                                                                                                                                                                                                                                                                                                           |
| Operand stack | `..., value1, value2 -> ...`                                                                                                                                                                                                                                                                                             |
| Description   | The `if_ge` instruction pops the top two values from the operand stack, compares them for a greater-than-or-equal relationship, and jumps to the specified address (calculated by adding the signed `offset` to the address of the `if_ge` instruction) if the first value is greater than or equal to the second value. |

## `ldc`

Loads a constant.

| **_ldc_**     | Description                                                                                                                                                                                                                                                                                                        |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Loads a constant from the constant pool to the operand stack.                                                                                                                                                                                                                                                      |
| Operands      | `u2 constant_pool_index`                                                                                                                                                                                                                                                                                           |
| Forms         | _ldc_ = 0x0D                                                                                                                                                                                                                                                                                                       |
| Operand stack | `... -> ..., value`                                                                                                                                                                                                                                                                                                |
| Description   | The unsigned `indexbyte1` and `indexbyte2` are assembled into an unsigned 16-bit index into the run-time constant pool of the current file, where the value of the index is calculated as `(indexbyte1 << 8) \| indexbyte2`. The index must be a valid index into the run-time constant pool of the current class. |

## `bpush_0`

Push the boolean `false` to the operand stack.

| **_bpush_0_** | Description                                                                |
|---------------|----------------------------------------------------------------------------|
| Operation     | Pushes the boolean `false` to the operand stack.                           |
| Operands      | _None_                                                                     |
| Forms         | _bpush_0_ = 0x0E                                                           |
| Operand stack | `... -> ..., false`                                                        |
| Description   | The `bpush_0` instruction pushes the boolean `false` to the operand stack. |

## `bpush_1`

Push the boolean `true` to the operand stack.

| **_bpush_1_** | Description                                                               |
|---------------|---------------------------------------------------------------------------|
| Operation     | Pushes the boolean `true` to the operand stack.                           |
| Operands      | _None_                                                                    |
| Forms         | _bpush_1_ = 0x0F                                                          |
| Operand stack | `... -> ..., true`                                                        |
| Description   | The `bpush_1` instruction pushes the boolean `true` to the operand stack. |

