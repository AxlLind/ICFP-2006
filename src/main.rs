#![allow(dead_code)]
use std::fs;
use std::io::{Result, Write};
use itertools::Itertools;
use easy_io::{InputReader, OutputWriter};
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

fn run_cpu(cpu: &mut CPU) -> Result<()> {
  let mut out = OutputWriter::new();
  let mut input = InputReader::new();
  loop {
    match cpu.execute() {
      ExitCode::Output(c) => out.print(c),
      ExitCode::NeedInput => {
        out.flush()?;
        cpu.push_str(&input.next_line());
      },
      ExitCode::Halted    => break,
    }
  }
  Ok(())
}

fn codex_umz() -> Result<()> {
  let program = read_program("files/codex.umz")?;
  let mut cpu = CPU::new(&program);
  cpu.push_str("(\\b.bb)(\\v.vv)06FHPVboundvarHRAk\np");
  run_cpu(&mut cpu)
}

fn sandmark() -> Result<()> {
  let program = read_program("files/sandmark.umz")?;
  let mut cpu = CPU::new(&program);
  run_cpu(&mut cpu)
}

fn step_2() -> Result<()> {
  let program = read_program("files/step_2.umz")?;
  let mut cpu = CPU::new(&program);
  run_cpu(&mut cpu)
}

fn main() -> Result<()> {
  sandmark()
}
