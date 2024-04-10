# The YuvaKriti Bytecode (YKB) File Format

This document describes the YKB file format of the YuvaKriti Virtual Machine (YKVM). The `ykb` file format is heavily
influenced by the Java `class` file format. As a result, the structure of the `ykb` file is similar to that of a `class`
file.

The YKB file consists of a stream of 8-bit bytes (`u1`). 16-bit (`u2`) or 32-bit (`u4`) quantities are construted by
reading 2 or 4 8-bit bytes (`u1`), respectively.

## YKB File Structure

A `ykb` file consists of a single `YKBFile` structure :

```
YKBFile {
    u4 magic_number;
    u2 major_version;
    u2 minor_version;
    u2 constant_pool_count;
    cp_info constant_pool[constant_pool_count-1];
    u2 attribute_count;
    attribute_info attributes[attribute_count];
}
```

### `magic_number`

_Size: 32-bit_

The magic number identifies the `ykb` format; it has the value `0x59754B72`, representing `YuKr` in ASCII.

### `major_version`, `minor_version`

_Size: 16-bit, 16-bit_

The values of the `minor_version` and `major_version` items are the minor
and major version numbers of this `ykb` file. Together, a major and a minor
version number determine the version of the `ykb` file format. If a `ykb` file
has major version number `M` and minor version number `m`, we denote the version
of its `ykb` file format as `M.m`.

### `constant_pool_count`

_Size: 16-bit_

The value of `constant_pool_count` is the number of entries available in the `constant_pool` + 1.

### `constant_pool`

_Size: Variable_

The `constant_pool` is a table of structures representing various string
constants, class names, field names, and other constants that are
referred to within the `YKBFile` structure and its substructures. The format of each `constant_pool` table entry is
indicated by its first `tag` byte.

The constant_pool table is indexed from `1` to `constant_pool_count - 1`. See [Constant Pool](./constant-pool) for more
details.

### `attribute_count`

_Size: 16-bit_

The value of `attribute_count` is the number of entries available in the `attributes`.

### `attributes`

_Size: Variable_

The `attributes` is an array of structures representing various
attributes of the `YKBFile` structure. See [Attributes](./attributes.md) for more details.
