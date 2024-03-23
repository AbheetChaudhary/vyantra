use vyantra::*;

fn main() {
    let program = vec![
        Inst::PSH(5),
        Inst::PSH(6),
        Inst::POP,
        Inst::PSH(21),
        Inst::ADD,
        Inst::POP,
        Inst::HLT,
    ];
    let mut machine = Machine::new(program);
    machine.run();
}
