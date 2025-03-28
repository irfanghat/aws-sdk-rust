// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_usage_amount(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UsageAmount,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("startHour")
            .date_time(&input.start_hour, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    {
        object.key("amount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.amount).into()),
        );
    }
    Ok(())
}
