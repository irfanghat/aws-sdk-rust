// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_encryption(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateEncryption,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.algorithm {
        object.key("algorithm").string(var_1.as_str());
    }
    if let Some(var_2) = &input.constant_initialization_vector {
        object.key("constantInitializationVector").string(var_2.as_str());
    }
    if let Some(var_3) = &input.device_id {
        object.key("deviceId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.key_type {
        object.key("keyType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.region {
        object.key("region").string(var_5.as_str());
    }
    if let Some(var_6) = &input.resource_id {
        object.key("resourceId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.role_arn {
        object.key("roleArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.secret_arn {
        object.key("secretArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.url {
        object.key("url").string(var_9.as_str());
    }
    Ok(())
}
