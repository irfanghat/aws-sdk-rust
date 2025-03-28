// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cdma_local_id(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CdmaLocalId,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("PnOffset").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.pn_offset).into()),
        );
    }
    {
        object.key("CdmaChannel").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.cdma_channel).into()),
        );
    }
    Ok(())
}
