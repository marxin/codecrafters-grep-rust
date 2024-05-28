use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::str::FromStr;

use clap::command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Extended expression
    #[arg(short = 'E')]
    extended: String,
}

#[derive(Debug)]
enum RegexToken {
    CharacterGroup(HashSet<char>),
}

#[derive(Debug)]
struct RegEx {
    tokens: Vec<RegexToken>,
}

impl FromStr for RegEx {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::new();
        let mut chars: VecDeque<_> = s.chars().collect();

        while !chars.is_empty() {
            let c = chars.pop_front().unwrap();

            match c {
                _ => tokens.push(RegexToken::CharacterGroup(HashSet::from([c]))),
            }
        }

        Ok(RegEx { tokens })
    }
}

impl RegEx {
    fn search(&self, s: &str) -> bool {
        let chars: Vec<_> = s.chars().collect();

        for i in 0..chars.len() {
            if self.search_from(i, &chars[i..]) {
                return true;
            }
        }

        false
    }

    fn search_from(&self, _start: usize, mut s: &[char]) -> bool {
        for token in self.tokens.iter() {
            if s.is_empty() {
                return false;
            }

            match token {
                RegexToken::CharacterGroup(group) => {
                    if !group.contains(&s[0]) {
                        return false;
                    }
                    s = &s[1..];
                }
            }
        }

        true
    }
}

fn main() {
    let args = Args::parse();
    let regex = RegEx::from_str(&args.extended).unwrap();

    let mut line = String::new();
    let mut handle = std::io::stdin().lock();
    handle.read_line(&mut line).unwrap();

    // strip the trailing newline
    line = line.strip_suffix("\n").unwrap().to_string();
    let searched = regex.search(&line);

    println!("regex: {regex:?}, line: {line}, searched: {searched}");

    std::process::exit(!searched as i32);
}
