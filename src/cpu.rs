use std::collections::VecDeque;

const CMOVE: usize = 0;  const RMEM: usize = 1;
const WMEM:  usize = 2;  const ADD:  usize = 3;
const MUL:   usize = 4;  const DIV:  usize = 5;
const NAND:  usize = 6;  const HALT: usize = 7;
const ALLOC: usize = 8;  const FREE: usize = 9;
const OUT:   usize = 10; const IN:   usize = 11;
const JUMP:  usize = 12; const IMM:  usize = 13;

pub enum ExitCode { Output(char), NeedInput, Halted }

pub struct CPU {
  reg: [u32;8],
  heap: Vec<Vec<u32>>,
  input: VecDeque<u32>,
  free_list: Vec<usize>,
  pc: usize,
}

impl CPU {
  pub fn new(program: &[u32]) -> Self {
    Self {
      reg: [0;8],
      heap: vec![program.into()],
      input: VecDeque::new(),
      free_list: Vec::new(),
      pc: 0,
    }
  }

  pub fn execute(&mut self) -> ExitCode {
    loop {
      let w = self.heap[0][self.pc] as usize;
      let a = (w >> 6) & 0x7;
      let b = (w >> 3) & 0x7;
      let c = (w >> 0) & 0x7;
      self.pc += 1;
      match w >> 28 {
        CMOVE => if self.reg[c] != 0 { self.reg[a] = self.reg[b]; },
        ADD   => self.reg[a] = self.reg[b] + self.reg[c],
        MUL   => self.reg[a] = self.reg[b] * self.reg[c],
        DIV   => self.reg[a] = self.reg[b] / self.reg[c],
        NAND  => self.reg[a] = !(self.reg[b] & self.reg[c]),
        RMEM  => self.reg[a] = self.heap[self.reg[b] as usize][self.reg[c] as usize],
        WMEM  => self.heap[self.reg[a] as usize][self.reg[b] as usize] = self.reg[c],
        ALLOC => self.reg[b] = self.alloc(self.reg[c]),
        FREE  => self.free(self.reg[c]),
        JUMP  => self.jump(self.reg[b], self.reg[c]),
        IMM   => self.reg[(w >> 25) & 0x7] = w as u32 & 0x1FF_FFFF,
        HALT  => return ExitCode::Halted,
        OUT   => return ExitCode::Output(self.reg[c] as u8 as char),
        IN    => match self.input.pop_front() {
          Some(i) => self.reg[c] = i,
          None    => {
            self.pc -= 1;
            return ExitCode::NeedInput;
          }
        },
        _ => unreachable!("invalid op {}", w >> 28),
      }
    }
  }

  pub fn push_str(&mut self, s: &str) {
    for b in s.bytes() { self.input.push_back(b as u32); }
    self.input.push_back(b'\n' as u32);
  }

  fn jump(&mut self, i: u32, new_pc: u32) {
    self.pc = new_pc as usize;
    if i > 0 { self.heap[0] = self.heap[i as usize].clone(); }
  }

  fn alloc(&mut self, size: u32) -> u32 {
    let size = size as usize;
    if let Some(i) = self.free_list.pop() {
      self.heap[i].resize(size, 0);
      return i as u32;
    }
    self.heap.push(vec![0; size]);
    self.heap.len() as u32 - 1
  }

  fn free(&mut self, i: u32) {
    let i = i as usize;
    self.heap[i].truncate(0);
    self.free_list.push(i);
  }
}
