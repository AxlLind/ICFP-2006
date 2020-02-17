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
  pc: usize,
}

impl CPU {
  pub fn new(program: &[u32]) -> Self {
    Self {
      reg: [0;8],
      heap: vec![program.into()],
      input: VecDeque::new(),
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
        ADD   => self.reg[a] = self.reg[b] + self.reg[c],
        MUL   => self.reg[a] = self.reg[b] * self.reg[c],
        DIV   => self.reg[a] = self.reg[b] / self.reg[c],
        NAND  => self.reg[a] = !(self.reg[b] & self.reg[c]),
        RMEM  => self.reg[a] = self.heap[self.reg[b] as usize][self.reg[c] as usize],
        WMEM  => self.heap[self.reg[a] as usize][self.reg[b] as usize] = self.reg[c],
        FREE  => self.heap[self.reg[c] as usize].clear(),
        ALLOC => self.reg[b] = self.calloc(self.reg[c]) as u32,
        IMM   => self.reg[((w >> 25) & 0x7)] = (w & 0x1FFFFFF) as u32,
        HALT  => return ExitCode::Halted,
        OUT   => return ExitCode::Output(self.reg[c] as u8 as char),
        CMOVE => if self.reg[c] != 0 { self.reg[a] = self.reg[b]; },
        JUMP  => {
          if self.reg[b] > 0 {
            self.heap[0] = self.heap[self.reg[b] as usize].clone();
          }
          self.pc = self.reg[c] as usize;
        },
        IN => match self.input.pop_front() {
          Some(i) => self.reg[c] = i,
          None    => {
            self.pc -= 1;
            return ExitCode::NeedInput;
          }
        },
        _   => unreachable!("invalid op {}", w >> 28),
      }
    }
  }

  pub fn push_str(&mut self, s: &str) {
    for b in s.bytes() { self.push_input(b); }
    self.push_input(b'\n');
  }

  fn calloc(&mut self, size: u32) -> usize {
    let size = size as usize;
    match self.heap.iter().position(|v| v.is_empty()) {
      Some(i) => {
        self.heap[i].resize(size,0);
        i
      },
      None => {
        self.heap.push(vec![0;size]);
        self.heap.len() - 1
      }
    }
  }

  fn push_input<T: Into<u32>>(&mut self, t: T) {
    self.input.push_back(t.into());
  }
}
