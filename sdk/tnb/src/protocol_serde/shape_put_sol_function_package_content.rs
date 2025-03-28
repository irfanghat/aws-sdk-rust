// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_sol_function_package_content_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput,
    crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => {
            crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
                };
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
                };
                tmp
            })
        }
        "ThrottlingException" => crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_sol_function_package_content_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput,
    crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentOutputBuilder::default();
        output = crate::protocol_serde::shape_put_sol_function_package_content::de_put_sol_function_package_content(_response_body, output)
            .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::put_sol_function_package_content_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError::unhandled)?
    })
}

pub fn ser_put_sol_function_package_content_headers(
    input: &crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.content_type {
        let formatted_2 = inner_1.as_str();
        let header_value = formatted_2;
        let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "content_type",
                format!("`{}` cannot be used as a header value: {}", &header_value, err),
            )
        })?;
        builder = builder.header("Content-Type", header_value);
    }
    Ok(builder)
}

pub(crate) fn de_put_sol_function_package_content(
    value: &[u8],
    mut builder: crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentOutputBuilder,
) -> ::std::result::Result<
    crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "id" => {
                    builder = builder.set_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "metadata" => {
                    builder = builder.set_metadata(
                        crate::protocol_serde::shape_put_sol_function_package_content_metadata::de_put_sol_function_package_content_metadata(tokens)?,
                    );
                }
                "vnfProductName" => {
                    builder = builder.set_vnf_product_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "vnfProvider" => {
                    builder = builder.set_vnf_provider(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "vnfdId" => {
                    builder = builder.set_vnfd_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "vnfdVersion" => {
                    builder = builder.set_vnfd_version(
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
