A Virtual Machine

- TODO
    - Implement these instructions: cmp, jz, jnz

- Language

    - `PSH(i32)` to Push an integer to the stack

    - `POP` to pop the stack

    - `ADD` to integer addition

    - `SUB` to do integer subtraction

    - `MUL` to do integer multiplication

    - `DIV` to do integer division. Integer arithemetic instructions operate on the last two stack elements and push the result on to the stack.

    - `SET(Reg, i32)` to set a register value. 

    - `CPY(Path, Path)` to move data from one location(register or stack pointer) to another

    - `JMP(isize)` to move the instruction pointer from its current position

    - `HLT` to Halt the program execution, end the machine

See `src/main.rs` to see usage of these instructions
