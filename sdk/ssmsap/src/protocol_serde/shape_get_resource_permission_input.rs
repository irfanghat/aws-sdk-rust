// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_resource_permission_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_resource_permission::GetResourcePermissionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action_type {
        object.key("ActionType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_arn {
        object.key("ResourceArn").string(var_2.as_str());
    }
    Ok(())
}
