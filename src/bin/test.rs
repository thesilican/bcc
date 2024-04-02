use anyhow::Result;
use bcc::lex;

fn main() -> Result<()> {
    let input = "
    int main() {
        return 0;
    }
    ";
    let output = lex(input)?;
    for token in output {
        println!("{token}");
    }
    Ok(())
}
