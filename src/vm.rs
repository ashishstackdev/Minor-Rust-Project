use crate::isa::Op;

pub struct VM {
    pub stack: Vec<i64>,
    pub globals: [i64; 256],
    pub ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: [0; 256],
            ip: 0,
        }
    }

    pub fn run(&mut self, program: Vec<Op>) {
        for instruction in program {
            match instruction {
                Op::Push(n) => {
                    self.stack.push(n);
                }

                Op::Print => {
                    if let Some(v) = self.stack.pop() {
                        println!("{}", v);
                    }
                }

                Op::Halt => {
                    break;
                }

                _ => {}
            }
        }
    }
}