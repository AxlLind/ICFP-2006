use std::collections::VecDeque;

const CMOVE: usize = 0x0; const RMEM: usize = 0x1;
const WMEM:  usize = 0x2; const ADD:  usize = 0x3;
const MUL:   usize = 0x4; const DIV:  usize = 0x5;
const NAND:  usize = 0x6; const HALT: usize = 0x7;
const ALLOC: usize = 0x8; const FREE: usize = 0x9;
const OUT:   usize = 0xa; const IN:   usize = 0xb;
const JUMP:  usize = 0xc; const IMM:  usize = 0xd;

pub enum ExitCode { Output(u32), NeedInput, Halted }

pub struct CPU {
  reg: [u32;8],
  pc: usize,
  heap: Vec<Vec<u32>>,
  input: VecDeque<u8>,
  free_list: VecDeque<u32>,
}

impl CPU {
  pub fn new(program: &[u32]) -> Self {
    Self {
      reg: [0;8],
      pc: 0,
      heap: vec![program.into()],
      input: VecDeque::new(),
      free_list: VecDeque::new(),
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
        OUT   => return ExitCode::Output(self.reg[c]),
        IN    => match self.input.pop_front() {
          Some(i) => self.reg[c] = i as u32,
          None => {
            self.pc -= 1;
            return ExitCode::NeedInput;
          }
        },
        _ => unreachable!("invalid op {}", w >> 28),
      }
    }
  }

  pub fn push_str(&mut self, s: &str) {
    self.input.extend(s.bytes());
    self.input.push_back(b'\n');
  }

  fn jump(&mut self, i: u32, pc: u32) {
    self.pc = pc as usize;
    if i > 0 { self.heap[0] = self.heap[i as usize].clone(); }
  }

  fn alloc(&mut self, size: u32) -> u32 {
    let size = size as usize;
    if let Some(i) = self.free_list.pop_front() {
      self.heap[i as usize].resize(size, 0);
      return i;
    }
    self.heap.push(vec![0; size]);
    self.heap.len() as u32 - 1
  }

  fn free(&mut self, i: u32) {
    self.heap[i as usize].clear();
    self.free_list.push_back(i);
  }
}
