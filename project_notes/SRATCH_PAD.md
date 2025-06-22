# Bytecode Disassembly

## Raw Bytecode Output

| Offset | Line | Instruction      | Operand | Value                | Jump Info                |
| ------ | ---- | ---------------- | ------- | -------------------- | ------------------------ |
| 0000   | 1    | OP_CONSTANT      | 0x01    | 'August'             |                          |
| 0002   |      | OP_DEFINE_GLOBAL | 0x00    | 'month'              |                          |
| 0004   | 3    | OP_GET_GLOBAL    | 0x00    | 'month'              |                          |
| 0006   |      | OP_DEFINE_GLOBAL | 0x02    | '\_\_switch_temp'    |                          |
| 0008   | 4    | OP_GET_GLOBAL    | 0x02    | '\_\_switch_temp'    |                          |
| 0010   |      | OP_CONSTANT      | 0x03    | 'July'               |                          |
| 0012   |      | OP_EQUAL         |         |                      |                          |
| 0013   |      | OP_JUMP_IF_FALSE | 0x0004  |                      | offset: 13 -> offset: 20 |
| 0016   |      | OP_POP           |         |                      |                          |
| 0017   | 5    | OP_CONSTANT      | 0x04    | 'Independence Day'   |                          |
| 0019   |      | OP_PRINT         |         |                      |                          |
| 0020   | 6    | OP_GET_GLOBAL    | 0x02    | '\_\_switch_temp'    |                          |
| 0022   |      | OP_CONSTANT      | 0x01    | 'August'             |                          |
| 0024   |      | OP_EQUAL         |         |                      |                          |
| 0025   |      | OP_JUMP_IF_FALSE | 0x0004  |                      | offset: 25 -> offset: 32 |
| 0028   |      | OP_POP           |         |                      |                          |
| 0029   | 7    | OP_CONSTANT      | 0x05    | 'Tiffany's Birthday' |                          |
| 0031   |      | OP_PRINT         |         |                      |                          |
| 0032   | 8    | OP_GET_GLOBAL    | 0x02    | '\_\_switch_temp'    |                          |
| 0034   |      | OP_CONSTANT      | 0x06    | 'September'          |                          |
| 0036   |      | OP_EQUAL         |         |                      |                          |
| 0037   |      | OP_JUMP_IF_FALSE | 0x0004  |                      | offset: 37 -> offset: 44 |
| 0040   |      | OP_POP           |         |                      |                          |
| 0041   | 9    | OP_CONSTANT      | 0x07    | 'Move into house'    |                          |
| 0043   |      | OP_PRINT         |         |                      |                          |
| 0044   | 10   | OP_RETURN        |         |                      |                          |

## Bytecode Execution Trace

| Offset | Line | Instruction        | Operand | Value                    | Stack State                      | comment                                           |
| ------ | ---- | ------------------ | ------- | ------------------------ | -------------------------------- | ------------------------------------------------- |
| 0000   | 1    | OP_CONSTANT        | 0x01    | 'August'                 | [ August ]                       |                                                   |
| 0002   |      | OP_DEFINE_GLOBAL   | 0x00    | 'month'                  |                                  |                                                   |
| 0004   | 3    | OP_GET_GLOBAL      | 0x00    | 'month'                  | [ August ]                       |                                                   |
| 0006   |      | OP_DEFINE_GLOBAL   | 0x02    | '\_\_switch_temp'        |                                  |                                                   |
| 0008   | 4    | OP_GET_GLOBAL      | 0x02    | '\_\_switch_temp'        | [ August ]                       |                                                   |
| 0010   |      | OP_CONSTANT        | 0x03    | 'July'                   | [ August ][ July ]               |                                                   |
| 0012   |      | OP_EQUAL           |         |                          | [ false ]                        | august != july                                    |
| 0013   |      | OP_JUMP_IF_FALSE   | 0x0004  | offset: 13 -> offset: 20 | [ false ]                        | false is left on the stack when OP_EQUAL is false |
| 0020   | 6    | OP_GET_GLOBAL_FAST | 0x02    | 'August'                 | [ false ][ August ]              |                                                   |
| 0022   |      | OP_CONSTANT        | 0x01    | 'August'                 | [ false ][ August ][ August ]    |                                                   |
| 0024   |      | OP_EQUAL           |         |                          | [ false ][ true ]                | august == august                                  |
| 0025   |      | OP_JUMP_IF_FALSE   | 0x0004  | offset: 25 -> offset: 32 | [ false ][ true ]                |                                                   |
| 0028   |      | OP_POP             |         |                          | [ false ]                        | true is popped from stack when OP_EQUAL is true   |
| 0029   | 7    | OP_CONSTANT        | 0x05    | 'Tiffany's Birthday'     | [ false ][ Tiffany's Birthday ]  |                                                   |
| 0031   |      | OP_PRINT           |         |                          | **Output: Tiffany's Birthday**   |                                                   |
|        |      |                    |         |                          | [ false ]                        |                                                   |
| 0032   | 8    | OP_GET_GLOBAL_FAST | 0x02    | 'August'                 | [ false ][ August ]              |                                                   |
| 0034   |      | OP_CONSTANT        | 0x06    | 'September'              | [ false ][ August ][ September ] |                                                   |
| 0036   |      | OP_EQUAL           |         |                          | [ false ][ false ]               |                                                   |
| 0037   |      | OP_JUMP_IF_FALSE   | 0x0004  | offset: 37 -> offset: 44 | [ false ][ false ]               |                                                   |
| 0044   | 10   | OP_RETURN          |         |                          |                                  |                                                   |
