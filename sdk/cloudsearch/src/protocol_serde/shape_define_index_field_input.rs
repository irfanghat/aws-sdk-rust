// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_define_index_field_input_input_input(
    input: &crate::operation::define_index_field::DefineIndexFieldInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DefineIndexField", "2013-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DomainName");
    if let Some(var_2) = &input.domain_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IndexField");
    if let Some(var_4) = &input.index_field {
        crate::protocol_serde::shape_index_field::ser_index_field(scope_3, var_4)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
