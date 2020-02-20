use std::io::{Write, Read};

fn main() {
    let content = std::fs::read_to_string("./helpme.bf").unwrap();
    let mut i = Interpreter::parse(&content);
    std::io::stdout().flush().unwrap();

    // dbg!(&i);

    i.run();

    std::io::stdout().flush().unwrap();
}

#[derive(Default, Debug)]
struct Interpreter {
    inst: Vec<Instruction>,
    inst_cursor: usize,
    mem: Vec<u8>,
    mem_cursor: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    MoveRight,
    MoveLeft,
    Inc,
    Dec,
    Write,
    Read,
    JumpForward, // [->] // IF CELL IS ZERO
    JumpBack, // ]->[ // IF CELL IS NOT ZERO
}

impl Interpreter {
    pub fn parse(s: &str) -> Self {
        let mut inst = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '>' => inst.push(Instruction::MoveRight),
                '<' => inst.push(Instruction::MoveLeft),
                '+' => inst.push(Instruction::Inc),
                '-' => inst.push(Instruction::Dec),
                '.' => inst.push(Instruction::Write),
                ',' => inst.push(Instruction::Read),
                '[' => inst.push(Instruction::JumpForward),
                ']' => inst.push(Instruction::JumpBack),
                _ => {}
            }
        }

        Self { inst, ..Default::default() }
    }

    pub fn run(&mut self) {
        while self.inst_cursor < self.inst.len() {
            self.step()
        }
    }

    pub fn current_inst(&self) -> Instruction {
        self.inst[self.inst_cursor]
    }

    pub fn current_cell(&self) -> u8 {
        self.mem[self.mem_cursor]
    }

    pub fn step(&mut self) {
        if self.mem.len() == 0 {
            self.mem.push(0);
        }

        match self.current_inst() {
            Instruction::MoveRight => {
                if self.mem_cursor == self.mem.len() - 1 {
                    self.mem.push(0);
                }
                self.mem_cursor += 1;
            }
            Instruction::MoveLeft => {
                if self.mem_cursor == 0 {
                    self.mem.insert(0, 0);
                } else {
                    self.mem_cursor -= 1;
                }
            }
            Instruction::Inc => {
                self.mem[self.mem_cursor] = self.current_cell().wrapping_add(1);
            }
            Instruction::Dec => {
                if self.current_cell() != 0 {
                    self.mem[self.mem_cursor] = self.current_cell().wrapping_sub(1);
                }
            }
            Instruction::Write => {
                print!("{}", self.current_cell() as char)
            },
            Instruction::Read => {
                let input: Option<u8> = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok());

                self.mem[self.mem_cursor] = input.expect("Valid input");
            }
            Instruction::JumpForward => {
                if self.current_cell() == 0 {
                    let mut count = 1;
                    while count != 0 {
                        self.inst_cursor += 1;
                        match self.current_inst() {
                            Instruction::JumpForward => { count += 1; }
                            Instruction::JumpBack => { count -= 1; }
                            _ => {}
                        }
                    }
                }
            }
            Instruction::JumpBack => {
                if self.current_cell() != 0 {
                    let mut count = 1;
                    while count != 0 {
                        self.inst_cursor -= 1;
                        match self.current_inst() {
                            Instruction::JumpBack => { count += 1; }
                            Instruction::JumpForward => { count -= 1; }
                            _ => {}
                        }
                    }
                }
            }
        }
        self.inst_cursor += 1;
    }
}
