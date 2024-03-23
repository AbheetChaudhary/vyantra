use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Fixed stack size
pub const STACK_SIZE: usize = 1024;

/// A really simple ISA
#[derive(Copy, Clone, Debug)]
pub enum Inst {
    /// Push an integer to the stack
    PSH(i32),

    /// Add two topmost stack entries and replace them with the result
    ADD,

    /// Pop the stack
    POP,

    /// Set a register value
    SET(Reg, i32),

    /// Halt the program execution, end the machine
    HLT,
}

/// Six general purpose registers
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug)]
struct Stack {
    memory: Vec<i32>,
}

#[derive(Debug)]
enum StackError {
    PushErr,
    PopErr,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
            StackError::PushErr => {
                format!(
                    "stack overflow: cannot push more than {} elements on stack",
                    STACK_SIZE
                )
            }

            StackError::PopErr => {
                format!("cannot pop from an empty stack")
            }
        };
        write!(f, "{}", err_message)
    }
}

impl Error for StackError {}

impl Stack {
    fn new() -> Self {
        Stack {
            memory: Vec::with_capacity(STACK_SIZE),
        }
    }

    /// Pop from the stack
    fn pop(&mut self) -> Result<i32, StackError> {
        if self.memory.len() > 0 {
            Ok(self.memory.pop().unwrap())
        } else {
            Err(StackError::PopErr)
        }
    }

    /// Push something on to the stack
    fn push(&mut self, value: i32) -> Result<(), StackError> {
        if self.memory.len() < STACK_SIZE {
            self.memory.push(value);
            Ok(())
        } else {
            Err(StackError::PushErr)
        }
    }
}

pub struct Machine {
    /// Array of instructions
    program: Vec<Inst>,

    /// Index of the next to-be-executed instruction
    ip: usize,

    /// THE STACK
    stack: Stack,

    /// THE REGISTERS
    registers: HashMap<Reg, i32>,
}

impl Machine {
    /// Create a new machine instance.
    /// It fails if the input program sequence is empty
    pub fn new(program: Vec<Inst>) -> Self {
        assert!(program.len() > 0);

        let mut registers = HashMap::new();
        registers.insert(Reg::A, 0);
        registers.insert(Reg::B, 0);
        registers.insert(Reg::C, 0);
        registers.insert(Reg::D, 0);
        registers.insert(Reg::E, 0);
        registers.insert(Reg::F, 0);

        Machine {
            program,
            ip: 0,
            stack: Stack::new(),
            registers,
        }
    }

    /// Run the machine and execute the program sequentially one instruction at a time.
    /// `HLT` instruction causes the machine to stop execution and report current state.
    /// If the final instruction is not `HLT` then it panics.
    pub fn run(&mut self) {
        loop {
            let inst = self.get_next_inst();
            match inst {
                Some(Inst::PSH(val)) => {
                    if let Err(e) = self.stack.push(val) {
                        panic!("{}", e);
                    }
                    println!("machine: push {val}");
                }
                Some(Inst::ADD) => {
                    let arg_1 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing addition argument: {}", e),
                    };
                    let arg_2 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing addition argument: {}", e),
                    };
                    if let Err(e) = self.stack.push(arg_1 + arg_2) {
                        panic!("{}", e);
                    }
                    println!("machine: add: {arg_2} {arg_1}");
                }
                Some(Inst::POP) => {
                    let val = match self.stack.pop() {
                        Ok(val) => val,
                        Err(e) => panic!("{}", e),
                    };
                    println!("machine: pop: {val}");
                }
                Some(Inst::SET(reg, val)) => {
                    self.set_reg_value(reg, val);
                    println!("set: {reg:?} {val}");
                }
                Some(Inst::HLT) => {
                    println!("\n\nmachine: halting...");
                    println!("program: {:?}", self.program);
                    println!("ip: {}", self.ip);
                    println!("stack: {:?}", self.stack);
                    println!("registers: {:?}", self.registers);
                    break;
                }
                None => panic!("error: abrupt halt"),
            }
        }
    }

    /// get next instruction and update the `ip`
    fn get_next_inst(&mut self) -> Option<Inst> {
        if let Some(inst) = self.program.get(self.ip) {
            self.ip += 1;
            Some(*inst)
        } else {
            None
        }
    }

    fn _get_reg_value(&self, reg: &Reg) -> Option<i32> {
        self.registers.get(reg).copied()
    }

    /// It will panic on trying to insert in a non existent register.
    fn set_reg_value(&mut self, reg: Reg, value: i32) {
        // insert returns None if the key didn't existed.
        self.registers
            .insert(reg, value)
            .expect("error: tried to set non-existant register");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let program = vec![Inst::PSH(5), Inst::PSH(6), Inst::ADD, Inst::POP, Inst::HLT];
        let mut machine = Machine::new(program);
        machine.run();
    }
}
