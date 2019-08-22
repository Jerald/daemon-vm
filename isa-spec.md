# ISA Spec

## Syntax

### Structure

The ISA has a fixed 32-bit instruction size.

Bit layout is currently as follows:

| Opcode  | L field mode | L Field | Operation size | R field mode | R Field | Total |
| :---:   |  :---:  | :---: |:---: | :---: | :---: | :---: |
| `XXXXX` | `XXX` | `XXXXXXXX` | `XXXXX` | `XXX` | `XXXXXXXX`  | `-` |
| 5 bits  | 3 bits | 8 bits | 5 bits | 3 bits | 8 bits  | 32 bits |

### Opcodes

| Opcode name  | Binary equivalent |
| :--- | :---:  |
| `add` - Addition | `00000` |
| `sub` - Subtraction | `00001` |
| `mul` - Multiplication | `00010` |
| `div` - Division | `00011` |
| `jmp` - Jump | `00100` |
| `jez` - Jump if equal to zero| `00101` |
| `jnz` - Jump if not equal to zero| `00110` |
| `jgz` - Jump if greater than zero | `00111` |
| `jlz` - Jump if less than zero| `01000` |
| `hcf` - Halt and Catch Fire | `01001` |
| `mov` - Move | `01010` |
| *unused* | `01011` |
| *...* | *...* |
| *unused* | `11111` |

### Addressing modes

| Mode name  | Binary equivalent |
| :---: | :---: |
| Immediate | `000` |
| Direct | `001` |
| Indirect | `010` |
| Instruction Pointer | `011` |
| *unused* | `100` |
| *...* | *...* |
| *unused* | `111` |

### Operation size

| Mode name  | Binary equivalent |
| :---: | :---: |
| 0 | `00001` |
| 1 | `00010` |
| 2 | `00100` |
| 3 | `01000` |
| 4 | `10000` |

## Semantics

### Numbers

**All** numbers are signed integers encoded in two's complement. This is for consistent and simple usage semantics of the binary format.

### Addressing modes

There are numerous addressing modes that are possible to be used with operations. Regardless of mode, all addressing is relative to the instruction pointer (IP) location at the time the instruction is executed.

#### Immediate

Also known as "literal" mode. The literal value provided is used as the operand.

#### Direct

The value provided is used as the memory address of the actual location to use as an operand.

#### Indirect

The value provided is used as the address of an 8-bit numbers, which is then used as the address of the actual location to be used as the operand.

#### Instruction Pointer

This will cause operations to directly interact with the IP. In almost every case, "reading" from the IP will simply give 0, but this can also be used to "write" to the IP, changing execution.


### Operation Size

Some operations will have a size associated to them. Namely things like `mov`, `add`, and etc. This will change how many bytes are used in performing the operation. For example, using `mov` with size of 3 will move 3 bytes from the source address to the 3 bytes starting at the destination address
