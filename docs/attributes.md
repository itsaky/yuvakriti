# Bytecode attributes

Attributes are used in `YKBFile` and method structures to add additional information to the bytecode. All attributes
have the following general format:

```
attribute_info {
    u2 attribute_name_index;
    u1 info[...];
}
```

The `attribute_name_index` is the index in the `constant_pool` table of a `Utf8Info` entry, representing the name of the
attribute. The `info` field contains the attribute information. Depending on the type of the attribute, the `info` field
contains a variable number of bytes.

## `Code` attribute

The `Code` attribute is used to represent the bytecode of a method or the top-level statements in a `yk` file.
The `Code` attribute contains the bytecode of the method in the form of an array of `u1` bytes. The following is the
structure of the `Code` attribute :

```
Code {
    u2 attribute_name_index;
    u2 max_stack;
    u2 max_locals;
    u4 code_length;
    u1 code[code_length];
}
```

| Code attribute            | Description                                                                                                  |
|---------------------------|--------------------------------------------------------------------------------------------------------------|
| `u2 attribute_name_index` | The index of the `Utf8Info` entry in the `constant_pool` table. The value at this index is always `Code`     |
| `u2 max_stack`            | The maximum depth of the operand stack at any point during the instruction execution of this code attribute. |
| `u2 max_locals`           | The maximum number of local variables at any point during the instruction execution of this code attribute.  |
| `u4 code_length`          | The number of bytes of instructions in this code attribute.                                                  |
| `u1 code[code_length]`    | The instructions in this code attribute.                                                                     |

## `SourceFile` attribute

The `SourceFile` attribute contains the source file name in the form of a `Utf8Info` entry in the constant pool. The
following is the structure
of the `SourceFile` attribute :

```
SourceFile {
    u2 attribute_name_index;
    u2 source_file_index;
}
```

| SourceFile attribute      | Description                                                                                                             |
|---------------------------|-------------------------------------------------------------------------------------------------------------------------|
| `u2 attribute_name_index` | The index of the `Utf8Info` entry in the `constant_pool` table. The value at this index is always `SourceFile`          |
| `u2 source_file_index`    | The index of the `Utf8Info` entry in the `constant_pool` table. The value at this index is the name of the source file. | 