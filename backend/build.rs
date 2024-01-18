fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/iot/types.proto")?;
    tonic_build::compile_protos("../proto/iot/registrationService.proto")?;
    tonic_build::compile_protos("../proto/iot/requestUpdateService.proto")?;
    tonic_build::compile_protos("../proto/iot/deviceControlService.proto")?;

    tonic_build::compile_protos("../proto/frontend/deviceControlService.proto")?;

    tonic_build::configure()
        .type_attribute("Device", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("RegistrationRequest", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("RegistrationResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ConnectedDevicesRequest", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("ConnectedDevicesResponse", "#[derive(serde::Deserialize, serde::Serialize)]") 
        .type_attribute("DeviceCapabilityStatus", "#[derive(serde::Deserialize, serde::Serialize)]") 
        .compile(
            &["../proto/frontend/registrationService.proto", "../proto/frontend/frontendTypes.proto", "../proto/iot/types.proto"],
            &["../proto/frontend", "../proto/iot"],
        )?;
    Ok(())
}
