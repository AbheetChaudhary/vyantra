use std::collections::HashMap;

/// A really simple ISA
#[derive(Copy, Clone, Debug)]
pub enum Inst {
    PSH(i32),      // push an integer to the stack
    ADD,           // add two topmost stack entries and replace them with the resutl
    POP,           // pop the stack
    SET(Reg, i32), // set a register value
    HLT,           // halt the program execution, end the machine
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

pub struct Machine {
    program: Vec<Inst>,           // array of instructions
    ip: usize,                    // index of the next to-be-executed instruction
    stack: Vec<i32>,              // THE STACK
    registers: HashMap<Reg, i32>, // THE REGISTERS
}

impl Machine {
    pub fn new(program: Vec<Inst>) -> Self {
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
            stack: Vec::new(),
            registers,
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst = self.get_next_inst();
            match inst {
                Some(Inst::PSH(val)) => {
                    self.push(val);
                    println!("push {val}");
                }
                Some(Inst::ADD) => {
                    let arg_1 = self.pop().expect("missing argument");
                    let arg_2 = self.pop().expect("missing argument");
                    self.push(arg_1 + arg_2);
                    println!("add: {arg_2} {arg_1}");
                }
                Some(Inst::POP) => {
                    let val = self.pop().expect("implement Result here");
                    println!("pop: {val}");
                }
                Some(Inst::SET(reg, val)) => {
                    self.set_reg_value(reg, val);
                    println!("set: {reg:?} {val}");
                }
                Some(Inst::HLT) => {
                    println!("\n\nhalting...");
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

    /// pop from the stack
    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    /// push something on to the stack
    fn push(&mut self, value: i32) {
        self.stack.push(value);
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
