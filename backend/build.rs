fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/iot/types/Types.proto")?;
    tonic_build::compile_protos("proto/iot/registration/RegistrationService.proto")?;
    Ok(())
}
