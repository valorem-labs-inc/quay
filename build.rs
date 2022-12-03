fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/requestforquote.proto")?;
    tonic_build::compile_protos("proto/quote.proto")?;
    Ok(())
}
