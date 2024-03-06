# The YK Lexer

## Lexical tokens

The following are the valid lexical tokens in YuvaKriti.

### Keywords

YuvaKriti includes a set of reserved keywords that have special meanings in the language. These keywords cannot be
used as identifiers.

- `and` - Logical AND
- `or` - Logical OR
- `if`
- `else`
- `while`
- `nil`
- `return`
- `true`
- `false`
- `fun`
- `for`
- `var`

### Operators

YuvaKriti supports various operators for arithmetic, comparison, and logical operations.

Arithmetic Operators

- `+` : Addition
- `-` : Subtraction
- `*` : Multiplication
- `/` : Division

Comparison Operators

- `==`: Equal to
- `!=`: Not equal to
- `<` : Less than
- `<=`: Less than or equal to
- `>` : Greater than
- `>=`: Greater than or equal to

### Punctuation

Punctuation symbols used for grouping and separating elements in the code.

- `(` : Left Parenthesis
- `)` : Right Parenthesis
- `{` : Left Brace
- `}` : Right Brace
- `[` : Left Bracket
- `]` : Right Bracket
- `,` : Comma
- `.` : Dot
- `;` : Semicolon

### Miscellaneous

Other token types that contribute to the structure of the language :

- `Number`: Numeric literals, e.g., 123, 3.14
- `String`: String literals, as described in the previous section
- `Identifier`: User-defined names for variables, functions, etc.

Example :

```
// Keywords
if true and false {
    return nil;
}

// Operators
var result = 5 + 3 * (2 - 1);
if result > 0 and result != 7 {
    print "Result is valid";
}

// Punctuation
var array = [1, 2, 3];

// Miscellaneous
var num = 42;
var str = "Hello, YuvaKriti!";
```

## Strings

A string literal in YuvaKriti is a sequence of characters enclosed in double quotes (`"`). The characters within the
quotes may include regular printable characters as well as escape sequences. Escape sequences start with a backslash (`\`)
and represent special characters or control sequences. The valid escape sequences are:

- `\"`: Double quote (")
- `\'`: Single quote (')
- `\\`: Backslash (\)
- `\n`: Newline character
- `\r`: Carriage return
- `\t`: Horizontal tab
- `\b`: Backspace
- `\f`: Form feed

Additionally, "YuvaKriti" supports Unicode escape sequences in the form of \u followed by four hexadecimal digits,
representing a Unicode code point.

Examples:

```
"Hello, World!"
"This is a string with an escape sequence: \n"
"This string contains Unicode character: \u03A9"
```

## Identifiers

Identifiers in YuvaKriti are used to name variables, functions, and other program entities. An identifier must start
with an alphabetic character (`a` to `z` or `A` to `Z`) or an underscore (`_`). Following the initial character, the
identifier may consist of alphabetic characters, digits (`0` to `9`), or underscores. YuvaKriti is case-sensitive, so
uppercase and lowercase letters are distinct.

Examples

```
variableName
my_function
ClassWithDigits123
_this_is_valid_too
```

## Identifier Recognition Process

The identifier recognition begins by checking whether the current character is a
valid starting character for an identifier using the `is_identifier_start` function.
Once a valid starting character is identified, the lexer continues scanning for
characters that are valid parts of an identifier, as determined by the
`is_identifier_part` function.

The `identifier` method handles the scanning process by iteratively advancing
through the characters until an invalid character is encountered or the end of
the input source is reached. The resulting sequence of characters forms the
potential identifier.

The `identifier_type` method then analyzes the identified sequence to determine
whether it matches any of the predefined keywords in the language. If a match is
found, the corresponding keyword token type is returned; otherwise, the token
type is set to `TokenType::Identifier`.

## Deterministic Finite Automaton (DFA) Algorithm

The DFA algorithm is employed to differentiate between keywords and normal
identifiers efficiently. In this context, the DFA consists of a set of states
representing the progress of identifier recognition. Each state corresponds to a
character in the keyword, and transitions between states occur based on the
current character being processed.

The `match_word_rest` method implements the DFA logic. It checks whether the
characters in the current word vector, starting at a specified index, match the
remaining characters in a keyword. If a match is successful, it returns the
corresponding keyword token type; otherwise, it returns `None`.

## Usage of DFA for Keyword Differentiation

The DFA algorithm is applied during the `identifier_type` analysis. For each
potential identifier, the lexer checks whether it matches any of the predefined
keywords by invoking the `match_word_rest` method. If a match is found, the
corresponding keyword token type is returned; otherwise, it defaults to
`TokenType::Identifier`.
