use std::fs;
use std::io::{Result, Write};
use itertools::Itertools;
use easy_io::OutputWriter;

const CMOVE: u32 = 0;  const RMEM: u32 = 1;
const WMEM:  u32 = 2;  const ADD:  u32 = 3;
const MUL:   u32 = 4;  const DIV:  u32 = 5;
const NAND:  u32 = 6;  const HALT: u32 = 7;
const ALLOC: u32 = 8;  const FREE: u32 = 9;
const OUT:   u32 = 10; const IN:   u32 = 11;
const JUMP:  u32 = 12; const IMM:  u32 = 13;

fn read_program(path: &str) -> Result<Vec<u32>> {
  let buf = fs::read(path)?;
  let program = buf.iter()
    .tuples()
    .map(|(&a,&b,&c,&d)| {
      let a = (a as u32) << 0x18;
      let b = (b as u32) << 0x10;
      let c = (c as u32) << 0x08;
      let d = (d as u32) << 0x00;
      a | b | c | d
    })
    .collect::<Vec<_>>();
  Ok(program)
}

fn main() -> Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  let program = read_program(&args[1])?;
  let mut out = OutputWriter::new();
  for (pc,w) in program.iter().enumerate() {
    let a = (w >> 6) & 7u32;
    let b = (w >> 3) & 7u32;
    let c = (w >> 0) & 7u32;
    write!(out, "{:#06x}: ", pc)?;
    match w >> 28 {
      CMOVE => writeln!(out, "cmove ${} ${} ${}", a, b, c)?,
      RMEM  => writeln!(out, "rmem  ${} ${} ${}", a, b, c)?,
      WMEM  => writeln!(out, "wmem  ${} ${} ${}", a, b, c)?,
      ADD   => writeln!(out, "add   ${} ${} ${}", a, b, c)?,
      MUL   => writeln!(out, "mul   ${} ${} ${}", a, b, c)?,
      DIV   => writeln!(out, "div   ${} ${} ${}", a, b, c)?,
      NAND  => writeln!(out, "nand  ${} ${} ${}", a, b, c)?,
      HALT  => writeln!(out, "halt")?,
      ALLOC => writeln!(out, "alloc ${} ${}", b, c)?,
      FREE  => writeln!(out, "free  ${}", c)?,
      OUT   => writeln!(out, "out   ${}", c)?,
      IN    => writeln!(out, "in    ${}", c)?,
      JUMP  => writeln!(out, "jump  ${} ${}", b, c)?,
      IMM   => writeln!(out, "imm   ${} {}", (w >> 25) & 7u32, w & 0x1FFFFFF)?,
      _     => writeln!(out)?,
    }
  }
  Ok(())
}
