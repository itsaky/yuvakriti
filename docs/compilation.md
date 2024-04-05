# YuvaKriti Compilation Process

- Lexical Analysis
- Syntactical Analysis
- Two-pass compilation
  - Visit the AST and store the declarations
  - Compile the statements which may be using the defined declarations
  - This is necessary for handling forward-references