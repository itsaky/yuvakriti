# YuvaKriti Compilation Process

A YuvaKriti program is compiled in multiple phases :

1. Lexical analysis
  - Verifies the lexical correctness of the program.
2. Syntactical analysis
  - Verifies the syntactical correctness of the program.
3. Attribution - context-dependant analysis
  - Name resolution - This task performs the following checks :
    - All identifiers must be defined and accessible before use.
    - There must not be multiple definitions of the same identifier, within the same scope (or in the parent scope).
  - Constant folding
    - Evaluates constant expressions in the program and reduces the tree.
4. Bytecode generation
  - Generates the bytecode for the program.
  - Writes the bytecode to a file.