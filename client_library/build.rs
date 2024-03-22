fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[
            "proto/iot/types.proto",
            "proto/iot/registrationService.proto",
            "proto/iot/requestUpdateService.proto",
            "proto/iot/deviceControlService.proto",
            ],
            &["proto/iot"],
            )?;

    return Ok(());
}
