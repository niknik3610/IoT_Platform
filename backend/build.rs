fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[
                "../proto/iot/registrationService.proto",
                "../proto/iot/requestUpdateService.proto",
                "../proto/iot/types.proto",
                "../proto/iot/deviceControlService.proto",
            ],
            &["../proto/iot"],
        )?;

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
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
