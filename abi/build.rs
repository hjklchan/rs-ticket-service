
fn main() -> std::io::Result<()> {
    let protos = &["protos/ticket.proto"];
    
    tonic_build::configure().out_dir("src/pb").compile(protos, &["."])?;

    Ok(())
}
