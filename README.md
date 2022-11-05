# Syntax and Lexical Analyzer

## How to use:
- navigate to the folder via the command-line interface.
- execute ./program.sh 
- You can change which of the test the program exeuctes by midfying the line "cargo run test0.sc -s > scheme-Yarrabozaed/mytest.scm" to include a different test file.

## Project description:
- This project simulates a syntax and lexical analyzer for the following grammar (written by Dr. Carols Arias of SPU):

```
PROGRAM     -->   definitions: 
                     DEFS
                  operations:
                     OPERATIONS
                  end.
DEFS        -->   DEF | DEF; DEFS
DEF         -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)
OPERATIONS  -->   OPERATION | OPERATION; OPERATIONS
OPERATION   -->   print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)
ID          -->   LETTER+
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
NUM         -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
```

The tokens of this grammar are (some lexemes are examples):
| Token | Lexeme |
| ----- | ------ |
| `ID` | `alpha` |
| `NUM` |  `256` |
| `SEMICOLON` | `;` |
| `COLON` | `:` |
| `COMMA` | `,` |
| `PERIOD` | `.` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `ASSIGN` | `=` |
| `DEFINITIONS` | `definitions` |
| `OPERATIONS` | `operations` |
| `POINT` | `point` |
| `CIRCLE` | `circle` |
| `SQUARE` | `square` |
| `PRINT` | `print` |
| `CONTAINED` | `contained` |
| `INTERSECTS` | `intersects` |
| `END` | `end` |

- The program conducts lexical analysis baed on the provided code in Rust. Based on the given flag (-s or -p) it then creates executable code for either the scheme program or the prolog program. 
- Then, it uses the prolog and scheme code to exeucte the given commands (finding if the intersection or containment of one 2D shape is possible in another).
