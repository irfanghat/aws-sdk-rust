// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_security_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_security::UpdateSecurityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_authentication {
        #[allow(unused_mut)]
        let mut object_2 = object.key("clientAuthentication").start_object();
        crate::protocol_serde::shape_client_authentication::ser_client_authentication(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.current_version {
        object.key("currentVersion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.encryption_info {
        #[allow(unused_mut)]
        let mut object_5 = object.key("encryptionInfo").start_object();
        crate::protocol_serde::shape_encryption_info::ser_encryption_info(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
