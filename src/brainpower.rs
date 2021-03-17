use std::iter;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct BrainPower {
    // instruction
    instruction: Vec<char>,
    // dynamic brainfuck array
    // default length is 0x100
    array: Vec<i8>,
    // register
    pc: usize,
    sp: usize,
}

impl BrainPower {
    pub fn new(inst: &str) -> BrainPower {
        let s: Vec<char> = inst.chars().filter(|c| match c {
            'A' | 'E' | 'e' | 'I' | 'O' | 'o' | 'U' | 'J' => true,
            _ => false,
        }).collect();

        if cfg!(debug_assertions) {
            println!("init data:");
            for c in &s {
                print!("{}", c);
            }
            println!("\ndata len {}", s.len());
        }

        BrainPower {
            instruction: s,
            array: vec![0; 100],
            pc: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self) -> &Vec<i8> {
        let mut input = io::stdin();
        let mut output = io::stdout();
        let stop = self.instruction.len();

        loop {
            let curr = self.instruction[self.pc];

            if cfg!(debug_assertions) {
                println!("PC: {}, Inst: {}", self.pc, curr);
            }

            match curr {
                // ++SP
                'o' => {
                    // max size of array is 0x7500 (29952)
                    if (self.sp + 1) >= 0x7500 {
                        panic!("Fuck: stack out of range (max 0x7500)");
                    }
                    // extend array
                    if (self.sp + 1) >= self.array.len() {
                        self.array.extend(iter::repeat(0).take(0x100));
                    }
                    self.sp += 1;
                },
                // --SP
                'A' => {
                    // bound check
                    if self.sp == 0 {
                        panic!("Fuck: stack less then zero at {}", self.pc);
                    }
                    self.sp -= 1;
                },
                // *SP += 1
                'E' => {
                    self.array[self.sp] += 1;
                },
                // *SP -= 1
                'e' => {
                    self.array[self.sp] -= 1;
                },
                // Output
                'I' => {
                    let s: u8 = self.array[self.sp] as u8;
                    output.write(&[s]).expect("Fuck: output error");
                },
                // Input
                'J' => {
                    let mut s: [u8; 1] = [0];
                    input.read(&mut s).expect("Fuck: input error");
                    self.array[self.sp] = s[0] as i8;
                },
                // Loop begin
                'O' => {
                    if self.array[self.sp] == 0 {
                        if self.pc + 1 >= stop {
                            panic!("Fuck: invalid '[' at {}", self.pc);
                        }
                        // find forward ']'
                        let mut count: i32 = 1;
                        for i in (self.pc + 1)..stop {
                            let c = self.instruction[i];
                            if c == '[' {
                                count += 1
                            }
                            else if c == ']' {
                                count -= 1;
                                if count == 0 {
                                    if cfg!(debug_assertions) {
                                        println!("PC forward to {}", i);
                                    }
                                    self.pc = i;
                                    break;
                                }
                            }
                        }
                        // not found
                        if count != 0 {
                            panic!("Fuck: invalid '[' at {}", self.pc);
                        }
                    }
                },
                // Loop end
                'U' => {
                    if self.array[self.sp] != 0 {
                        if self.pc == 0 {
                            panic!("Fuck: invalid ']' at 0");
                        }
                        // find back '['
                        let mut count: i32 = 1;
                        for i in (0..(self.pc - 1)).rev() {
                            let c = self.instruction[i];
                            if c == '[' {
                                count -= 1;
                                if count == 0 {
                                    if cfg!(debug_assertions) {
                                        println!("PC back to {}", i);
                                    }
                                    self.pc = i;
                                    break;
                                }
                            }
                            else if c == ']' {
                                count += 1;
                            }
                        }
                        if count != 0 {
                            panic!("Fuck: invalid ']' at {}", self.pc);
                        }
                    }
                },
                s => {
                    panic!("Fuck: Unknown char {} at {}", s, self.pc);
                }
            }

            // next instruction
            self.pc += 1;
            if self.pc >= stop {
                break;
            }
        }

        // return restul stack
        &self.array
    }
}
