# Syntax & Semantics

## Print

Use the `print` statement to print a value.

```
print "Hello, World!";
```

## Variables

Variables are defined using the `var` keyword, followed by the name of the variable and an optional initializer.

```
// Strings
var a = "String";

// Numbers
var b = 42;

// Booleans
var c = true;
var d = false;

// If variables are not initialized, their value is set to `null`.
var e;
```

## Arithmetic

```
var a = 10;
var b = 20;

print a + b; // 30
print a - b; // -10
print a * b; // 200
print a / b; // 0.5
```

## Comparison

```
var a = 10;
var b = 20;

print a == b; // false
print a != b; // true
print a < b; // true
print a > b; // false
print a <= b; // true
print a >= b; // false
```

## Logical operators

```
var a = true;
var b = false;

print a and b; // false
print a or b; // true
print !a; // false
print !b; // true
```

## Control flow

```
var a = true;
if a {
    print "a is true";
} else {
    print "a is false";
}
```

## For loop

```

// Simple, indexed for loop
for (var i = 0; i < 10; i = i = 1) {
    print i;
}

// Nested for loop
for (var i = 0; i < 10; i = i + 1) {
    for (var j = 0; j < 10; j = j + 1) {
        print i + j;
    }
}
```

## While loop

```
// Simple loop
while true {
    print "Never ending loop";
}

// Nested loop
while true {
    var i = 0;
    while i < 10 {
        print i;
        i = i + 1;
    }
}
```

## Labeled loops

```

outer: while true {
    inner: while true {
        print "Seems never ending, but runs only once";
        break outer;
    }
}

outer: for (var i = 0; i < 10; i = i + 1) {
    inner: for (var j = 0; j < 10; j = j + 1) {
        if i == 5 && j == 5 {
            // jump out of the outer loop...
            break outer;
        }
        
        // ...or continue the outer loop
        continue outer;
    }
}
```