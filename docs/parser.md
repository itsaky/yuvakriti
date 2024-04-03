## YKParser

This document describes the `YKParser`, a component responsible for parsing source code written in the YuvaKriti
programming language. The YKParser's role is to analyze the source code and convert it into a structured representation
(Abstract Syntax Tree or AST) that can be further processed by other parts of the compiler.

### Methodology

The YKParser employs a recursive descent parsing approach. It starts from the first token in the source code and
iteratively applies a set of parsing rules based on the token type. These rules define how to construct the AST based on
the encountered tokens and how to proceed to the next part of the code.

The parser heavily relies on the following functions to achieve its functionality:

- `accept` - Consumes the current token if it matches the expected type, otherwise reports an error.
- `tmatch` - Similar to `accept` but does not report an error on mismatch.
- `advance` - Moves the parser forward to the next token in the source code.
- `peek` - Returns the current token without consuming it.

### Potential Problems

Currently, the provided YKParser implementation might encounter the following potential problems:

- **Error Handling:** While the parser reports errors when encountering unexpected tokens or constructs, it might not
  provide very specific or user-friendly error messages. This could make it difficult for developers to identify and fix
  the underlying issues in their code.
- **Left Recursion:** The grammar might contain left-recursive constructs which could lead to infinite recursion during
  parsing. The parser would need to be refactored to handle left recursion using techniques like left factoring.
- **Incomplete Parsing:** The provided parser implementation might not support all the features of the YuvaKriti
  language. It's essential to ensure the parser is up-to-date with the latest language specifications.

### Working of the Parser

Here's a breakdown of how the YKParser works step by step:

1. **Initialization:** The parser takes a YKLexer (lexer for YuvaKriti language) and a diagnostic handler as input. It
  initializes its internal state with the first two tokens from the lexer.
2. **Parsing Loop:** The parser enters a loop that continues until the end of the source code is reached (EOF).
3. **Declaration or Statement:** Inside the loop, the parser tries to identify whether the current token sequence
  corresponds to a declaration or a statement. It achieves this by calling the `decl` or `try_parse_stmt_decl` function
  depending on the token type.
   - **`decl`:** This function handles function declarations by consuming the `fun` keyword, followed by the function
   - name, parameters, and the body block. It also supports variable declarations using the `var` keyword.
   - **`try_parse_stmt_decl`:** This function attempts to parse various statement types, including print, return, for,
   - if, while, and expressions. It checks for the specific keywords that introduce these statements and then consumes
   - the following tokens accordingly.
4. **Expression Parsing:** If the parser encounters an expression as part of a statement or as a standalone expression,
  it calls the `expr` function. This function follows a recursive descent approach to parse various expression
  constructs, including binary expressions (with operators like +, -, *, etc.), unary expressions (with operators like
  negation, etc.), primary expressions (literals, identifiers, etc.), and grouping expressions (parentheses).
5. **Error Reporting:** Throughout the parsing process, the parser utilizes the `report` function to signal any errors
  encountered. This function creates a Diagnostic object containing information about the error type, message, and
  location in the source code.
6. **AST Construction:** As the parser successfully consumes tokens and constructs syntactic elements, it builds the
  corresponding AST nodes. These nodes represent the parsed code structure, including declarations, statements,
  expressions, etc.
7. **Output:** After reaching the end of the source code, the parser returns the complete AST representing the parsed
  program.