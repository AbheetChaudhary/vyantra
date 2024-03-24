use vyantra::*;

fn main() {
    let program = vec![
        Inst::PSH(5),
        Inst::PSH(6),
        Inst::POP,
        Inst::PSH(21),
        Inst::ADD,
        Inst::POP,
        Inst::SET(Reg::A, 12),
        Inst::SET(Reg::B, 144),
        Inst::PSH(0),
        Inst::CPY(Path::STK(0), Path::REG(Reg::B)),
        Inst::PSH(0),
        Inst::CPY(Path::STK(0), Path::REG(Reg::A)),
        Inst::DIV,
        Inst::POP,
        Inst::JMP(-6),
        Inst::HLT,
    ];
    let mut machine = Machine::new(program);
    machine.run();
}
