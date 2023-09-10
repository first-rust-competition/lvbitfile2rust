use anyhow::Result;

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "usage: lvbitfile2rust-cli <bitfile>");
    println!("{}", lvbitfile2rust::codegen(args.remove(1), false)?);
    Ok(())
}
