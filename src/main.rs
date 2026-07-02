mod isa;
mod vm;
mod assembler;
mod disassembler;
mod bytecode;
mod errors;

use isa::Op;
use vm::VM;

fn main() {
    let mut vm = VM::new();

    let program = vec![
        Op::Push(7),
        Op::Push(3),
        Op::Add,
        Op::Print,
        Op::Halt,
    ];

    vm.run(program);
}