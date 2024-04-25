# The Constant Pool

The `constant_pool` is a table of structures representing various string constants, class names, field names, and other
constants that are referred to within the `YKBFile` structure and its substructures. The format of each `constant_pool`
table entry is indicated by its first `tag` byte.

This document describes the type of entries in the constant pool. Each section for the entries in this document contains
a table defining the format of the constant pool entry. Each entry starts with a `tag` value of 8-bit byte, followed by
one or more bytes containing the information represented by the entry.

## `Utf8Info`

| Utf8Info              |
|-----------------------|
| u1 tag = `0x00`       |
| u2 byte_count         |
| u8 bytes[bytes_count] |

The `Utf8Info` structure is used to represent constant string values. The items of the `Utf8Info` structure are as
follows :

- `tag`
    - The tag value for the structure. It has the value `0x00`.
- `byte_count`
    - The value of `byte_count` gives the number of bytes in the `bytes` array.
- `bytes`
    - The `bytes` array contains the bytes of the string.

## `NumberInfo`

| NumberInfo      |
|-----------------|
| u1 tag = `0x01` |
| u4 high_bytes   |
| u4 low_bytes    |

The `NumberInfo` structure is used to represent constant 64-bit floating point values. The items of the `NumberInfo`
structure are as follows :

- `tag`
    - The tag value for the structure. It has the value `0x01`.
- `high_bytes`
    - The value of `high_bytes` is the most significant 32-bits of the floating point number.
- `low_bytes`
    - The value of `low_bytes` is the least significant 32-bits of the floating point number.

## `StringInfo`

| StringInfo      |
|-----------------|
| u1 tag = `0x03` |
| u2 string_index |

The `StringInfo` structure is used to represent constant values of type String. The items of the `StringInfo` structure
are as follows :

- `tag`
    - The tag value for the structure. It has the value `0x02`.
- `string_index`
    - The `string_index` is index in the `constant_pool` containing the value of the string constant. The entry in
      the `constant_pool` at this given index is of type `Utf8Info`.
