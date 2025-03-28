// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_stack_set_input_input_input(
    input: &crate::operation::describe_stack_set::DescribeStackSetInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeStackSet", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StackSetName");
    if let Some(var_2) = &input.stack_set_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CallAs");
    if let Some(var_4) = &input.call_as {
        scope_3.string(var_4.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
