use std::io::{Write, Read, BufRead};
use std::collections::VecDeque;

fn main() {
    let content = std::fs::read_to_string("./helpme.bf").unwrap();
    let mut i = Interpreter::new(Instruction::parse(&content));
    i.run();
}

#[derive(Default, Debug)]
struct Interpreter {
    inst: Vec<Instruction>,
    cursor: usize,
    
    memory: VecDeque<u8>,
    pointer: usize,
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
    End,
}

impl Instruction {
    pub fn parse(s: &str) -> Vec<Self> {
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
        inst.push(Instruction::End);
        inst.shrink_to_fit();
        inst
    }
}

impl Interpreter {
    pub fn new(inst: Vec<Instruction>) -> Self {
        Self { inst, ..Default::default() }
    }

    pub fn run(&mut self) {
        while !self.done() {
            self.step()
        }
    }

    pub fn inst(&self) -> Instruction {
        self.inst[self.cursor]
    }

    pub fn cell(&self) -> u8 {
        self.memory[self.pointer]
    }

    pub fn cell_mut(&mut self) -> &mut u8 {
        &mut self.memory[self.pointer]
    }

    pub fn done(&self) -> bool {
        self.inst() == Instruction::End
    }

    fn on_right_edge(&self) -> bool {
        self.pointer == self.memory.len() - 1 
    }

    fn on_left_edge(&self) -> bool {
        self.pointer == 0
    }

    pub fn step(&mut self) {
        if self.memory.len() == 0 {
            self.memory.push_back(0);
        }

        match self.inst() {
            Instruction::End => return,
            Instruction::MoveRight => {
                if self.on_right_edge() {
                    self.memory.push_back(0);
                }
                self.pointer += 1;
            }
            Instruction::MoveLeft => {
                if self.on_left_edge() {
                    self.memory.push_front(0);
                } else {
                    self.pointer -= 1;
                }
            }
            Instruction::Inc => {
                *self.cell_mut() += 1;
            }
            Instruction::Dec => {
                if self.cell() != 0 { // No underflow subtraction for project
                    *self.cell_mut() -= 1;
                }
            }
            Instruction::Write => {
                print!("{}", self.cell() as char)
            },
            Instruction::Read => {
                let mut line = String::new();
                let stdin = std::io::stdin();
                stdin.lock().read_line(&mut line).unwrap();

                *self.cell_mut() = line.chars().next().unwrap() as u8;
            }
            Instruction::JumpForward if self.cell() == 0 => {
                let mut count = 1;
                while count != 0 {
                    self.cursor += 1;
                    match self.inst() {
                        Instruction::JumpForward => count += 1,
                        Instruction::JumpBack => count -= 1,
                        _ => {}
                    }
                }
            }
            Instruction::JumpBack if self.cell() != 0 => {
                let mut count = 1;
                while count != 0 {
                    self.cursor -= 1;
                    match self.inst() {
                        Instruction::JumpBack => count += 1,
                        Instruction::JumpForward => count -= 1,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        self.cursor += 1;
    }
}
