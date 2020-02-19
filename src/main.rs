use std::io::{stdout, Result, Write};
use itertools::Itertools;
use easy_io::InputReader;
use cpu::{CPU, ExitCode};

fn read_program(path: &str) -> Result<Vec<u32>> {
  let buf = std::fs::read(path)?;
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

fn write_buf(buf: &mut Vec<u8>) -> Result<()> {
  stdout().write_all(&buf)?;
  stdout().flush()?;
  buf.clear();
  Ok(())
}

fn run_cpu(cpu: &mut CPU) -> Result<()> {
  let mut input = InputReader::new();
  let mut buf = Vec::new();
  loop {
    match cpu.execute() {
      ExitCode::Output(i) => buf.push(i as u8),
      ExitCode::NeedInput => {
        write_buf(&mut buf)?;
        cpu.push_str(&input.next_line());
      },
      ExitCode::Halted => break,
    }
  }
  write_buf(&mut buf)?;
  Ok(())
}

fn codex() -> Result<()> {
  let program = read_program("files/codex.umz")?;
  let mut cpu = CPU::new(&program);
  cpu.push_str("(\\b.bb)(\\v.vv)06FHPVboundvarHRAk\np");
  for _ in 0..195 { cpu.execute(); }
  run_cpu(&mut cpu)
}

fn sandmark() -> Result<()> {
  let program = read_program("files/sandmark.umz")?;
  let mut cpu = CPU::new(&program);
  run_cpu(&mut cpu)
}

fn umix(args: &[String]) -> Result<()> {
  let program = read_program("files/umix.umz")?;
  let mut cpu = CPU::new(&program);
  if let Some(s) = args.get(2) {
    let path = format!("inputs/{}.txt", s);
    let input = std::fs::read_to_string(path)?;
    cpu.push_str(input.trim());
  }
  run_cpu(&mut cpu)
}

fn main() -> Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() < 2 { return umix(&args); }
  match &args[1][..] {
    "sandmark" => sandmark(),
    "codex"    => codex(),
    _          => umix(&args),
  }
}
