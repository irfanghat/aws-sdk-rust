// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_recovery_instance_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.recovery_instance_id {
        object.key("recoveryInstanceID").string(var_1.as_str());
    }
    Ok(())
}
