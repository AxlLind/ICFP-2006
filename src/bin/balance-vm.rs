use std::fmt;
use itertools::Itertools;

const SCIENCE: u8 = 0b000; const MATH:  u8 = 0b001;
const PHYSICS: u8 = 0b011; const LOGIC: u8 = 0b010;

const TEST_MULT_PROGRAM: [u8;12] = [0b011_00010, 0b011_11100, 0b011_11111, 0b001_11001, 0b011_10010, 0b000_00000, 0b010_00110, 0b011_11111, 0b011_11100, 0b000_00001, 0b001_11101, 0b000_11110];

fn sext5(w: u8) -> u8 { ((w & 0x1f) ^ 0x10) - 0x10 }

struct BalanceVM {
  ip: usize,
  is: u8,
  sr: [u8; 4],
  dr: [u8; 2],
  mem: [u8; 256],
  code: Vec<u8>,
}

impl fmt::Display for BalanceVM {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "BalanceVm")?;
    writeln!(f, "ip: {}", self.ip)?;
    writeln!(f, "is: {}", self.is)?;
    writeln!(f, "dr: {:?}", self.dr)?;
    writeln!(f, "sr: {:?}", self.sr)?;
    writeln!(f, "mem:")?;
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
      let (w,imm,d,s1,s2) = self.fetch_inst();
      match w >> 5 {
        MATH => {
          self.wmem(d+1, self.rmem(s1+1) - self.rmem(s2+1));
          self.wmem(d, self.rmem(s1) + self.rmem(s2));
        }
        LOGIC => {
          self.wmem(d+1, self.rmem(s1+1) ^ self.rmem(s2+1));
          self.wmem(d, self.rmem(s1) & self.rmem(s2));
        }
        SCIENCE => {
          if self.rmem(0) != 0 { self.is = imm; }
          if self.is == 0 { return; }
        }
        PHYSICS => {
          let Self { sr, dr, ..} = self;
          let l = [dr[1], dr[0], sr[3], sr[2], sr[1], sr[0] + imm];
          let mut cd = (0..6)
            .filter(|i| (w >> i) & 1 == 1)
            .collect::<Vec<_>>();
          cd.push(cd[0]);
          for (&i,&j) in cd.iter().tuple_windows() {
            match j {
              0 => self.dr[1] = l[i],
              1 => self.dr[0] = l[i],
              2 => self.sr[3] = l[i],
              3 => self.sr[2] = l[i],
              4 => self.sr[1] = l[i],
              5 => self.sr[0] = l[i],
              _ => unreachable!(),
            }
          }
        }
        _ => return,
      }
      self.ip += self.is as i8 as usize;
      self.ip %= self.code.len();
    }
  }

  fn fetch_inst(&self) -> (u8,u8,u8,u8,u8) {
    let w = self.code[self.ip];
    let imm = sext5(w);
    let d = (w >> 4) & 1;
    let s1 = (w >> 2) & 3;
    let s2 = w & 3;
    (w,imm,d,s1,s2)
  }

  fn rmem(&self, sr: u8) -> u8 {
    self.mem[self.sr[sr as usize % 4] as usize]
  }

  fn wmem(&mut self, dr: u8, val: u8) {
    self.mem[self.dr[dr as usize % 2] as usize] = val;
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
