fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "usage: lvbitfile2rust-cli <bitfile>");
    println!("{}", lvbitfile2rust::codegen(&args[1])?);
    Ok(())
}
