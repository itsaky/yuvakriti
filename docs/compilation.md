# YuvaKriti Compilation Process

A YuvaKriti program is compiled in multiple phases :

- Lexical analysis
  - Verifies the lexical correctness of the program.
- Syntactical analysis
  - Verifies the syntactical correctness of the program.
- Attribution - context-dependant analysis
  - Name resolution - This task performs the following checks :
    - All identifiers must be defined and accessible before use.
    - There must not be multiple definitions of the same identifier, within the same scope (or in the parent scope).
  - Constant folding
    - Evaluates constant expressions in the program and reduces the tree.
- Bytecode generation
  - Generates the bytecode for the program.
  - Writes the bytecode to a file.