fn main() -> Result<(), Box<dyn std::error::Error>> {
    //All .proto files that need to be compiled should look like this
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
