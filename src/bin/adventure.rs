use std::fs;
use std::collections::HashSet;
use std::io::Result;
use itertools::Itertools;

#[derive(Debug,Default,Clone)]
struct Item {
  name: String,
  sub_name: String,
  missing: Vec<Item>,
}

impl PartialEq for Item {
  fn eq(&self, other: &Self) -> bool {
    if self.name != other.name { return false; }
    if self.missing.len() != other.missing.len() { return false; }
    self.missing.iter()
      .map(|i| other.missing.iter().find(|&j| j == i))
      .all(|o| o.is_some())
  }
}
impl Eq for Item {}

#[derive(Debug,PartialEq)]
enum Token { ParOpen, ParClose, Id(String), Missing }

fn tokenize(s: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut buf = String::new();
  for c in s.chars() {
    match c {
      ')'|' ' => {
        match &buf[..] {
          "a"|"" => {},
          "missing"|"and" => tokens.push(Token::Missing),
          _ => tokens.push(Token::Id(buf.clone())),
        }
        if c == ')' { tokens.push(Token::ParClose); }
        buf.clear();
      }
      '(' => tokens.push(Token::ParOpen),
      _   => buf.push(c),
    }
  }
  if !buf.is_empty() {
    tokens.push(Token::Id(buf.clone()));
  }
  tokens
}

fn find_matching_paren(tokens: &[Token]) -> usize {
  let mut depth = 0;
  for (i,t) in tokens.iter().enumerate() {
    match t {
      Token::ParClose => {
        depth -= 1;
        if depth == 0 { return i; }
        assert!(depth > 0);
      }
      Token::ParOpen => depth += 1,
      _   => {}
    }
  }
  unreachable!("Invalid tokens")
}

impl Item {
  /*
    My attempt at a formal grammar to parse this.
    It might be *somewhat* incorrect but was enough
    for me to be able to figure out the parsing:
      ID    = display|P-9887-WFE|F-9247-QRI|...
      THING = a ID | (ITEM)
      ITEM  = THING missing THING [and THING]*
  */
  fn from_str(s: &str) -> Option<Self> {
    let mut iter = s.split(".");
    let mut name_iter = iter.next()?.split(" ");

    let name = name_iter.next()?.to_string();
    let sub_name = name_iter.next().unwrap_or("").to_string();
    let missing = match iter.next() {
      Some("")|None => vec![],
      Some(s) => Self::item(&tokenize(s)).missing,
    };

    Some(Self{ name, sub_name, missing })
  }

  fn thing(tokens: &[Token]) -> (usize, Self) {
    match &tokens[0] {
      Token::Id(name) => {
        let name = name.clone();
        let this = Self{ name, ..Self::default() };
        (1, this)
      },
      Token::ParOpen => {
        let next = find_matching_paren(tokens);
        let this = Self::item(&tokens[1..next]);
        (next+1, this)
      }
      _ => panic!("Invalid tokens"),
    }
  }

  fn item(tokens: &[Token]) -> Self {
    let (mut next, mut this) = Self::thing(tokens);
    while next < tokens.len() {
      assert!(tokens[next] == Token::Missing);
      next += 1;
      let (advance, child) = Self::thing(&tokens[next..]);
      next += advance;
      this.missing.push(child);
    }
    this
  }

  fn take(&self) -> String {
    format!("take {}", self.name_str())
  }

  fn incinerate(&self) -> String {
    format!("incinerate {}", self.name_str())
  }

  fn name_str(&self) -> String {
    if self.sub_name.is_empty() {
      return self.name.clone();
    }
    format!("{} {}", self.sub_name, self.name)
  }

  fn is_irrelevant(&self, relevant_items: &HashSet<(String,String)>) -> bool {
    let t = (self.name.clone(), self.sub_name.clone());
    !relevant_items.contains(&t)
  }
}

fn produce(items: &[Item], wanted: &Item) -> Option<(HashSet<(String,String)>,HashSet<(String,String)>)> {
  items.iter()
    .filter(|i| i.name == wanted.name)
    .map(|current| {
      let mut combinations = HashSet::new();
      let mut relevant_items = HashSet::new();
      for i in &current.missing {
        if wanted.missing.contains(i) { continue; }
        match produce(items, i) {
          Some((_combinations, _relevant_items)) => {
            combinations.extend(_combinations);
            relevant_items.extend(_relevant_items);
          },
          None => return None,
        }
        combinations.insert((current.name.clone(), i.name.clone()));
      }
      relevant_items.insert((current.name.clone(),current.sub_name.clone()));
      Some((combinations,relevant_items))
    })
    .find(|o| o.is_some())
    .unwrap_or(None)
}

fn try_combine(combinations: &HashSet<(String,String)>, inv: &mut HashSet<String>) -> Vec<String> {
  let possible = inv.iter()
    .combinations(2)
    .flat_map(|t| vec![
      (t[0].clone(),t[1].clone()),
      (t[1].clone(),t[0].clone()),
    ])
    .unique()
    .filter(|t| combinations.contains(t))
    .collect::<Vec<_>>();
  for (_,b) in &possible { inv.remove(b); }
  possible.iter()
    .map(|(a,b)| format!("combine {} {}", a, b))
    .collect()
}

fn find_commands(items: &[Item], combinations: &HashSet<(String,String)>, relevant_items: &HashSet<(String,String)>) -> (Vec<String>, usize) {
  let mut cmds = Vec::new();
  let mut inv = HashSet::new();
  let mut max_inv_size = 0;
  for i in items {
    cmds.push(i.take());
    if i.is_irrelevant(relevant_items) {
      cmds.push(i.incinerate());
      continue;
    }
    inv.insert(i.name.clone());
    max_inv_size = std::cmp::max(max_inv_size,inv.len());
    cmds.extend(try_combine(combinations, &mut inv));
  }
  (cmds, max_inv_size)
}

fn main() -> Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  let wanted_item = args[1].clone();
  let path = format!("./files/adventure/{}.txt", wanted_item);
  let items = fs::read_to_string(path)?
    .trim()
    .split("\n")
    .filter_map(Item::from_str)
    .collect::<Vec<_>>();
  let display = Item{ name: wanted_item, ..Item::default() };
  let (combinations, relevant_items) = produce(&items, &display).unwrap();
  let (cmds, max_inv_size) = find_commands(&items, &combinations, &relevant_items);
  println!("{}", cmds.join("\n"));
  println!("\nmax inv size: {}", max_inv_size);
  Ok(())
}
