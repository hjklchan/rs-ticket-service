fn main() -> std::io::Result<()> {
    let protos = &["protos/ticket.proto"];

    tonic_build::configure()
        .out_dir("src/pb")
        .build_client(false)
        .build_server(true)
        .compile(protos, &["."])?;

    Ok(())
}
