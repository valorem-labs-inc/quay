fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/types.proto")?;
    tonic_build::compile_protos("proto/quote.proto")?;
    tonic_build::compile_protos("proto/seaport.proto")?;

    Ok(())
}
