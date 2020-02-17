use std::fs;
use std::io::Result;
use itertools::Itertools;
use easy_io::InputReader;

mod cpu;
use cpu::{CPU, ExitCode};

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
  let program = read_program("./codex.umz")?;
  let mut cpu = CPU::new(&program);
  let mut input = InputReader::new();

  cpu.push_str("(\\b.bb)(\\v.vv)06FHPVboundvarHRAk");
  loop {
    match cpu.execute() {
      ExitCode::Output(c) => print!("{}", c),
      ExitCode::NeedInput => cpu.push_str(&input.next_line()),
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}
