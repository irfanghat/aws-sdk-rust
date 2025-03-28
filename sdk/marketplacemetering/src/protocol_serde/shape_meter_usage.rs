// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_meter_usage_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::meter_usage::MeterUsageOutput, crate::operation::meter_usage::MeterUsageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::meter_usage::MeterUsageError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CustomerNotEntitledException" => crate::operation::meter_usage::MeterUsageError::CustomerNotEntitledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CustomerNotEntitledExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_customer_not_entitled_exception::de_customer_not_entitled_exception_json_err(_response_body, output)
                        .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DuplicateRequestException" => crate::operation::meter_usage::MeterUsageError::DuplicateRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DuplicateRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_duplicate_request_exception::de_duplicate_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceErrorException" => crate::operation::meter_usage::MeterUsageError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEndpointRegionException" => crate::operation::meter_usage::MeterUsageError::InvalidEndpointRegionException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEndpointRegionExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_endpoint_region_exception::de_invalid_endpoint_region_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidProductCodeException" => crate::operation::meter_usage::MeterUsageError::InvalidProductCodeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidProductCodeExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_product_code_exception::de_invalid_product_code_exception_json_err(_response_body, output)
                        .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTagException" => crate::operation::meter_usage::MeterUsageError::InvalidTagException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTagExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_tag_exception::de_invalid_tag_exception_json_err(_response_body, output)
                    .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidUsageAllocationsException" => crate::operation::meter_usage::MeterUsageError::InvalidUsageAllocationsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidUsageAllocationsExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_usage_allocations_exception::de_invalid_usage_allocations_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidUsageDimensionException" => crate::operation::meter_usage::MeterUsageError::InvalidUsageDimensionException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidUsageDimensionExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_usage_dimension_exception::de_invalid_usage_dimension_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::meter_usage::MeterUsageError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TimestampOutOfBoundsException" => crate::operation::meter_usage::MeterUsageError::TimestampOutOfBoundsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TimestampOutOfBoundsExceptionBuilder::default();
                output = crate::protocol_serde::shape_timestamp_out_of_bounds_exception::de_timestamp_out_of_bounds_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::meter_usage::MeterUsageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_meter_usage_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::meter_usage::MeterUsageOutput, crate::operation::meter_usage::MeterUsageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::meter_usage::builders::MeterUsageOutputBuilder::default();
        output = crate::protocol_serde::shape_meter_usage::de_meter_usage(_response_body, output)
            .map_err(crate::operation::meter_usage::MeterUsageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_meter_usage_input(
    input: &crate::operation::meter_usage::MeterUsageInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_meter_usage_input::ser_meter_usage_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_meter_usage(
    value: &[u8],
    mut builder: crate::operation::meter_usage::builders::MeterUsageOutputBuilder,
) -> ::std::result::Result<crate::operation::meter_usage::builders::MeterUsageOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
{
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "MeteringRecordId" => {
                    builder = builder.set_metering_record_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
