fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/iot/types.proto")?;
    tonic_build::compile_protos("../proto/iot/registrationService.proto")?;
    tonic_build::compile_protos("../proto/iot/requestUpdateService.proto")?;
    tonic_build::compile_protos("../proto/iot/deviceControlService.proto")?;

    return Ok(());
}
