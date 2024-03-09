fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(
            &[
                "../../proto/frontend/registrationService.proto",
                "../../proto/frontend/frontendTypes.proto",
                "../../proto/iot/types.proto",
                "../../proto/frontend/deviceControlService.proto",
            ],
            &["../../proto/frontend", "../../proto/iot"],
        )?;
    Ok(())
}
