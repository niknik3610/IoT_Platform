fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/iot/types.proto")?;
    tonic_build::compile_protos("../proto/iot/registrationService.proto")?;
    tonic_build::compile_protos("../proto/iot/requestUpdateService.proto")?;
    tonic_build::compile_protos("../proto/iot/deviceControlService.proto")?;

    tonic_build::compile_protos("../proto/frontend/frontend_types.proto")?;
    tonic_build::compile_protos("../proto/frontend/frontendDeviceControlService.proto")?;

    tonic_build::configure()
        .type_attribute("RegistrationRequest", "#[derive(Deserialize, Serialize)]")
        .type_attribute("RegistrationResponse", "#[derive(Deserialize, Serialize)]")
        .type_attribute("ConnectedDevicesRequest", "#[derive(Deserialize, Serialize)]")
        .type_attribute("ConnectedDevicesResponse", "#[derive(Deserialize, Serialize)]") 
        .type_attribute("DeviceCapabilityStatus", "#[derive(Deserialize, Serialize)]") 
        .compile(
            &["../proto/frontend/registrationService.proto", "../proto/frontend/frontend_types.proto"],
            &["../proto/frontend"],
        )?;
    Ok(())
}
