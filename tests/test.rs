use anyhow::{anyhow, Result};
use bcc::{lex, parse, PrettyPrint};
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::{
    fmt::Write,
    fs::{read_dir, File},
    io::Read,
    path::PathBuf,
};

struct Fixture {
    name: String,
    input: String,
    expected: String,
}

fn read_suite(fixture: &str) -> Vec<Fixture> {
    let root = PathBuf::from("tests/fixtures").join(fixture);
    let mut fixtures = Vec::new();
    for entry in read_dir(&root).unwrap() {
        let input_path = entry.unwrap().path();
        let mut buffer = String::new();
        File::open(&input_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        let splits = buffer.split("\n==========\n").collect::<Vec<_>>();
        if splits.len() != 2 {
            panic!("unable to parse suite {input_path:?}");
        }
        let fixture = Fixture {
            name: input_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            input: splits[0].to_string(),
            expected: splits[1].to_string(),
        };
        fixtures.push(fixture);
    }
    fixtures.sort_by_key(|x| x.name.clone());
    fixtures
}

fn pretty_diff(actual: &str, expected: &str) -> String {
    let diff = TextDiff::from_lines(actual, expected);
    let mut output = String::new();
    for change in diff.iter_all_changes() {
        let formatted = match change.tag() {
            ChangeTag::Delete => format!("-{change}").red(),
            ChangeTag::Insert => format!("+{change}").green(),
            ChangeTag::Equal => format!(" {change}").normal(),
        };
        write!(output, "{formatted}").unwrap();
    }
    output
}

fn run_suite<F>(suite: &str, f: F) -> u32
where
    F: Fn(&str) -> Result<String>,
{
    let fixtures = read_suite(suite);
    let mut failures = 0;
    for fixture in fixtures {
        let name = fixture.name;
        let output = f(&fixture.input);
        match output {
            Ok(output) if output == fixture.expected => {
                let pass = "PASS".green();
                println!("[{pass}] {suite}/{name}");
            }
            Ok(output) => {
                let fail = "FAIL".red();
                let diff = pretty_diff(&output, &fixture.expected);
                println!("[{fail}] {suite}/{name}\n{diff}");
                failures += 1;
            }
            Err(err) => {
                let fail = "FAIL".red();
                println!("[{fail}] {suite}/{name}\n{err}");
                failures += 1;
            }
        }
    }
    failures
}

#[test]
fn run_all_tests() -> Result<()> {
    let mut failures = 0;
    failures += run_suite("lex", |input| {
        let tokens = match lex(&input) {
            Ok(x) => x,
            Err(err) => {
                return Ok(err.to_string());
            }
        };
        let mut output = String::new();
        for token in tokens {
            writeln!(output, "{}", token.pretty_print()).unwrap();
        }
        Ok(output)
    });
    failures += run_suite("parse", |input| {
        let tokens = match lex(&input) {
            Ok(x) => x,
            Err(err) => {
                return Err(err.context("lex error"));
            }
        };
        let ast = match parse(tokens) {
            Ok(x) => x,
            Err(err) => {
                return Ok(err.to_string());
            }
        };
        Ok(ast.pretty_print())
    });

    if failures > 0 {
        println!();
        Err(anyhow!("{failures} tests failed"))
    } else {
        Ok(())
    }
}
