use std::collections::HashMap;
use std::error::Error;
use std::fmt;
// use std::borrow::Borrow;

/// Fixed stack size
pub const STACK_SIZE: usize = 1024;

/// A path is either a register or a stack pointer
#[derive(Copy, Clone, Debug)]
pub enum Path {
    REG(Reg),
    STK(isize), // zero means the head of the stack, +ve means before it, -ve means after it
}

/// Path error when invalid register or stack location is accessed. Invalid register means any
/// register that does not exists, invalid stack location means location outside the stack memory
/// vector.
#[derive(Debug)]
pub enum PathError {
    StackErr,
    RegErr,
}

impl Error for PathError {}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
            PathError::StackErr => {
                format!("invalid stack pointer")
            }

            PathError::RegErr => {
                format!("invalid register access")
            }
        };
        write!(f, "{}", err_message)
    }
}

/// A really simple ISA
#[derive(Copy, Clone, Debug)]
pub enum Inst {
    /// Push an integer to the stack
    PSH(i32),

    /// Pop the stack
    POP,

    /// Integer addition
    ADD,

    /// Integer subtraction
    SUB,

    /// Integer multiplication
    MUL,

    /// Integer division
    DIV,

    /// Set a register value
    SET(Reg, i32),

    /// Move data from one location(register or stack pointer) to another
    CPY(Path, Path),

    /// Move the instruction pointer from its current position
    JMP(isize),

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
    sp: isize,
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
            sp: -1,
        }
    }

    fn _get_sp(&self) -> isize {
        self.sp
    }

    /// get value at stack index
    fn get_at_idx(&self, idx: isize) -> Result<i32, PathError> {
        assert!(self.sp >= 0 && (self.sp as usize) < self.memory.len());
        if idx >= 0 {
            self.memory
                .get(self.sp as usize - idx as usize)
                .ok_or(PathError::StackErr)
                .copied()
        } else {
            self.memory
                .get(self.sp as usize + (-1 * idx) as usize)
                .ok_or(PathError::StackErr)
                .copied()
        }
    }

    /// set value at stack index
    fn set_at_idx(&mut self, idx: isize, val: i32) -> Result<(), PathError> {
        assert!(self.sp >= 0 && (self.sp as usize) < self.memory.len());
        let reference = if idx >= 0 {
            self.memory.get_mut(self.sp as usize - idx as usize)
        } else {
            self.memory.get_mut(self.sp as usize + (-1 * idx) as usize)
        };
        match reference {
            Some(elem) => {
                *elem = val;
                Ok(())
            }
            None => Err(PathError::StackErr),
        }
    }

    /// Pop from the stack
    fn pop(&mut self) -> Result<i32, StackError> {
        if self.memory.len() > 0 {
            self.sp -= 1;
            Ok(self.memory.pop().unwrap())
        } else {
            Err(StackError::PopErr)
        }
    }

    /// Push something on to the stack
    fn push(&mut self, value: i32) -> Result<(), StackError> {
        if self.memory.len() < STACK_SIZE {
            self.sp += 1;
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
                    let arg_2 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing addition argument: {}", e),
                    };
                    let arg_1 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing addition argument: {}", e),
                    };
                    if let Err(e) = self.stack.push(arg_1 + arg_2) {
                        panic!("{}", e);
                    }
                    println!("machine: add: {arg_1} {arg_2}");
                }
                Some(Inst::SUB) => {
                    let arg_2 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing subtraction argument: {}", e),
                    };
                    let arg_1 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing subtraction argument: {}", e),
                    };
                    if let Err(e) = self.stack.push(arg_1 - arg_2) {
                        panic!("{}", e);
                    }
                    println!("machine: sub: {arg_1} {arg_2}");
                }
                Some(Inst::MUL) => {
                    let arg_2 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing multiplication argument: {}", e),
                    };
                    let arg_1 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing multiplication argument: {}", e),
                    };
                    if let Err(e) = self.stack.push(arg_1 * arg_2) {
                        panic!("{}", e);
                    }
                    println!("machine: mul: {arg_1} {arg_2}");
                }
                Some(Inst::DIV) => {
                    let arg_2 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing division argument: {}", e),
                    };
                    if arg_2 == 0 {
                        panic!("attempted to divide by zero");
                    }
                    let arg_1 = match self.stack.pop() {
                        Ok(arg) => arg,
                        Err(e) => panic!("missing division argument: {}", e),
                    };
                    if let Err(e) = self.stack.push(arg_1 / arg_2) {
                        panic!("{}", e);
                    }
                    println!("machine: div: {arg_1} {arg_2}");
                }
                Some(Inst::POP) => {
                    let val = match self.stack.pop() {
                        Ok(val) => val,
                        Err(e) => panic!("{}", e),
                    };
                    println!("machine: pop: {val}");
                }
                Some(Inst::SET(reg, val)) => {
                    match self.set_reg_value(reg, val) {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    };
                    println!("machine: set: {reg:?} {val}");
                }
                Some(Inst::CPY(dst, src)) => {
                    let val = match self.get_from_path(src) {
                        Ok(val) => val,
                        Err(e) => panic!("{}", e),
                    };
                    match self.set_at_path(dst, val) {
                        Ok(_) => (),
                        Err(e) => panic!("{}", e),
                    };
                    println!("machine: cpy {dst:?} {src:?}");
                }
                Some(Inst::JMP(step)) => {
                    if step < 0 {
                        self.ip -= (-1 * (step - 1)) as usize;
                    } else if step > 0 {
                        self.ip += step as usize - 1;
                    } else {
                    }
                }
                Some(Inst::HLT) => {
                    println!("machine: halting...");
                    self.dump();
                    break;
                }
                None => panic!("error: illegal instruction...abrupt halt"),
            }
        }
    }

    fn get_from_path(&self, path: Path) -> Result<i32, PathError> {
        match path {
            Path::REG(reg) => self.get_reg_value(&reg),
            Path::STK(rel_idx) => self.stack.get_at_idx(rel_idx),
        }
    }

    fn set_at_path(&mut self, path: Path, val: i32) -> Result<(), PathError> {
        match path {
            Path::REG(reg) => self.set_reg_value(reg, val),
            Path::STK(rel_idx) => self.stack.set_at_idx(rel_idx, val),
        }
    }

    fn dump(&self) {
        println!("\n\nmachine dump:");
        println!("\tprogram: {:?}", self.program);
        println!("\tip: {}", self.ip);
        println!("\tstack: {:?}", self.stack);
        println!("\tregisters: {:?}", self.registers);
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

    fn get_reg_value(&self, reg: &Reg) -> Result<i32, PathError> {
        match self.registers.get(reg) {
            Some(val) => Ok(*val),
            None => Err(PathError::RegErr),
        }
    }

    /// It will panic on trying to insert in a non existent register.
    fn set_reg_value(&mut self, reg: Reg, value: i32) -> Result<(), PathError> {
        // insert returns None if the key didn't existed.
        if self.registers.contains_key(&reg) {
            self.registers.insert(reg, value).unwrap();
            Ok(())
        } else {
            Err(PathError::RegErr)
        }
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
