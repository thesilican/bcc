use anyhow::Result;
use bcc::{lex, parse, PrettyPrint};

fn main() -> Result<()> {
    let input = "struct my_struct *my_fn(struct my_struct *my_param) {}";
    let tokens = lex(input)?;
    for token in &tokens {
        println!("{}", token.pretty_print());
    }
    let program = parse(tokens);
    println!("{program:#?}");
    Ok(())
}
