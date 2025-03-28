// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_node_association_status_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_node_association_status::DescribeNodeAssociationStatusInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.node_association_status_token {
        object.key("NodeAssociationStatusToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.server_name {
        object.key("ServerName").string(var_2.as_str());
    }
    Ok(())
}
