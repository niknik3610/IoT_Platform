fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/iot/types.proto")?;
    tonic_build::compile_protos("../proto/iot/registrationService.proto")?;
    tonic_build::compile_protos("../proto/iot/requestUpdateService.proto")?;
    tonic_build::compile_protos("../proto/iot/deviceControlService.proto")?;

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(
            &[
                "../proto/frontend/registrationService.proto",
                "../proto/frontend/frontendTypes.proto",
                "../proto/iot/types.proto",
                "../proto/frontend/deviceControlService.proto",
            ],
            &["../proto/frontend", "../proto/iot"],
        )?;
    Ok(())
}
