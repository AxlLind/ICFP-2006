use std::fmt;
use std::iter;
use itertools::Itertools;

const SCIENCE: u8 = 0b000; const MATH:  u8 = 0b001;
const PHYSICS: u8 = 0b011; const LOGIC: u8 = 0b010;

const TEST_MULT_PROGRAM: [u8;12] = [0b011_00010, 0b011_11100, 0b011_11111, 0b001_11001, 0b011_10010, 0b000_00000, 0b010_00110, 0b011_11111, 0b011_11100, 0b000_00001, 0b001_11101, 0b000_11110];

fn sext5(w: u8) -> u8 { ((w & 0x1f) ^ 0x10) - 0x10 }

struct BalanceVM {
  ip: usize,
  is: u8,
  dr: [u8;2],
  sr: [u8;4],
  mem: [u8;256],
  code: Vec<u8>,
}

impl fmt::Display for BalanceVM {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "BalanceVm")?;
    writeln!(f, "ip: {}", self.ip)?;
    writeln!(f, "is: {}", self.is)?;
    writeln!(f, "dr: {:?}", self.dr)?;
    writeln!(f, "sr: {:?}", self.sr)?;
    for i in 0..256 {
      let d = if i % 32 == 31 {'\n'} else {','};
      write!(f, "{}{}", self.mem[i], d)?;
    }
    Ok(())
  }
}

impl BalanceVM {
  fn execute(&mut self) {
    loop {
      let w = self.code[self.ip];
      let dr = (w >> 4) & 1;
      let s1 = (w >> 2) & 3;
      let s2 = (w >> 0) & 3;
      match w >> 5 {
        MATH => {
          self.wmem(dr+1, self.rmem(s1+1) - self.rmem(s2+1));
          self.wmem(dr, self.rmem(s1) + self.rmem(s2));
        }
        LOGIC => {
          self.wmem(dr+1, self.rmem(s1+1) ^ self.rmem(s2+1));
          self.wmem(dr, self.rmem(s1) & self.rmem(s2));
        }
        SCIENCE => {
          if self.rmem(0) != 0 { self.is = sext5(w); }
          if self.is == 0 { return; }
        }
        PHYSICS => {
          let Self { sr, dr, .. } = self;
          let l = [dr[1], dr[0], sr[3], sr[2], sr[1], sr[0] + sext5(w)];
          (0..6).rev()
            .filter(|i| (w >> i) & 1 == 1)
            .chain(iter::once(5))
            .tuple_windows()
            .for_each(|(i,j)| match i {
              0 => self.dr[1] = l[j],
              1 => self.dr[0] = l[j],
              2 => self.sr[3] = l[j],
              3 => self.sr[2] = l[j],
              4 => self.sr[1] = l[j],
              5 => self.sr[0] = l[j],
              _ => unreachable!(),
            });
        }
        _ => return,
      }
      self.ip += self.is as i8 as usize;
      self.ip %= self.code.len();
    }
  }

  fn rmem(&self, sr: u8) -> u8 {
    self.mem[self.sr[sr as usize & 3] as usize]
  }

  fn wmem(&mut self, dr: u8, val: u8) {
    self.mem[self.dr[dr as usize & 1] as usize] = val;
  }
}

fn main() {
  let mut mem = [0;256];
  mem[0] = 3;
  mem[1] = 3;
  let mut vm = BalanceVM {
    ip: 0,
    is: 1,
    sr: [0,1,2,3],
    dr: [4,5],
    mem,
    code: TEST_MULT_PROGRAM[..].into(),
  };
  vm.execute();
  assert_eq!(vm.mem[2], 3 * 3);
  print!("{}", vm);
}
