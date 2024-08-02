use anyhow::Result;
use bcc::{lex, parse};

fn main() -> Result<()> {
    let input = "struct my_struct *my_fn(struct my_struct *my_param) {}";
    let tokens = lex(input)?;
    for token in &tokens {
        println!("{token}");
    }
    let program = parse(tokens);
    println!("{program:#?}");
    Ok(())
}
