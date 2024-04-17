# Possible compiler optimizations

## Redundant instructions

Consider the program :
```
var a = 0;
print a;
```

The bytecode instructions for the above program are :
```
========= YKB =========
major version: 0
minor version: 1
Constant pool: 
    #1: NumberInfo           0
    #2: Utf8Info             i
    #3: Utf8Info             Code
Attributes: 
    Code: max_stack=1
        ldc #1 // 0
        store_0 
        load_0 
        print
```

It is seen that the `store_0` and `load_0` instructions are redundant. Maybe we could add an optimizer
which optimizes bytecode instructions like above.