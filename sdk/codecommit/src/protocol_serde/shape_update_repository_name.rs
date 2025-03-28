// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_repository_name_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_repository_name::UpdateRepositoryNameOutput,
    crate::operation::update_repository_name::UpdateRepositoryNameError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidRepositoryNameException" => crate::operation::update_repository_name::UpdateRepositoryNameError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRepositoryNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::operation::update_repository_name::UpdateRepositoryNameError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryNameExistsException" => crate::operation::update_repository_name::UpdateRepositoryNameError::RepositoryNameExistsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNameExistsExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_name_exists_exception::de_repository_name_exists_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::operation::update_repository_name::UpdateRepositoryNameError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNameRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_repository_name::UpdateRepositoryNameError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_repository_name::UpdateRepositoryNameError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_repository_name_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_repository_name::UpdateRepositoryNameOutput,
    crate::operation::update_repository_name::UpdateRepositoryNameError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_repository_name::builders::UpdateRepositoryNameOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_repository_name_input(
    input: &crate::operation::update_repository_name::UpdateRepositoryNameInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_repository_name_input::ser_update_repository_name_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
