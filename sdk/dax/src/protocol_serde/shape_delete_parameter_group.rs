// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_parameter_group_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_parameter_group::DeleteParameterGroupOutput,
    crate::operation::delete_parameter_group::DeleteParameterGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterCombinationException" => {
            crate::operation::delete_parameter_group::DeleteParameterGroupError::InvalidParameterCombinationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterGroupStateFault" => crate::operation::delete_parameter_group::DeleteParameterGroupError::InvalidParameterGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterGroupStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_group_state_fault::de_invalid_parameter_group_state_fault_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::delete_parameter_group::DeleteParameterGroupError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ParameterGroupNotFoundFault" => crate::operation::delete_parameter_group::DeleteParameterGroupError::ParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ParameterGroupNotFoundFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_parameter_group_not_found_fault::de_parameter_group_not_found_fault_json_err(_response_body, output)
                        .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceLinkedRoleNotFoundFault" => crate::operation::delete_parameter_group::DeleteParameterGroupError::ServiceLinkedRoleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceLinkedRoleNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_service_linked_role_not_found_fault::de_service_linked_role_not_found_fault_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_parameter_group::DeleteParameterGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_parameter_group_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_parameter_group::DeleteParameterGroupOutput,
    crate::operation::delete_parameter_group::DeleteParameterGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_parameter_group::builders::DeleteParameterGroupOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_parameter_group::de_delete_parameter_group(_response_body, output)
            .map_err(crate::operation::delete_parameter_group::DeleteParameterGroupError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_parameter_group_input(
    input: &crate::operation::delete_parameter_group::DeleteParameterGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_parameter_group_input::ser_delete_parameter_group_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_delete_parameter_group(
    value: &[u8],
    mut builder: crate::operation::delete_parameter_group::builders::DeleteParameterGroupOutputBuilder,
) -> ::std::result::Result<
    crate::operation::delete_parameter_group::builders::DeleteParameterGroupOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DeletionMessage" => {
                    builder = builder.set_deletion_message(
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
