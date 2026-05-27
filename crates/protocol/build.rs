fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .out_dir("src/proto")
        .compile_protos(&["proto/protocol.proto"], &["proto"])?;
    Ok(())
}
