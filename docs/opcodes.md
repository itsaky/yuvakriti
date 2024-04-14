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

| **_ldc_**     | Description                                                                                                                                                                                                                                                                                                        |
|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Loads a constant from the constant pool to the operand stack.                                                                                                                                                                                                                                                      |
| Operands      | `u2 constant_pool_index`                                                                                                                                                                                                                                                                                           |
| Forms         | _ldc_ = 0x13                                                                                                                                                                                                                                                                                                       |
| Operand stack | `... -> ..., value`                                                                                                                                                                                                                                                                                                |
| Description   | The unsigned `indexbyte1` and `indexbyte2` are assembled into an unsigned 16-bit index into the run-time constant pool of the current file, where the value of the index is calculated as `(indexbyte1 << 8) \| indexbyte2`. The index must be a valid index into the run-time constant pool of the current class. |

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

## `sub`

Subtract two values.

| **_sub_**     | Description                                                                                                                                                               |
|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Operation     | Pops two values from the operand stack, subtracts the second from the first, and pushes the result back onto the operand stack.                                           |
| Operands      | None                                                                                                                                                                      |
| Forms         | _sub_ = 0x03                                                                                                                                                              |
| Operand stack | `..., value1, value2 -> ..., (value1 - value2)`                                                                                                                           |
| Description   | The `sub` instruction pops the top two values from the operand stack, subtracts the second value from the first value, and pushes the result back onto the operand stack. |
