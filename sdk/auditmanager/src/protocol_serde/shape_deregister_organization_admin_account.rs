// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_organization_admin_account_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountOutput,
    crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?
                };
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?
                };
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?
                };
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::unhandled)?
                };
                tmp
            })
        }
        _ => crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_organization_admin_account_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountOutput,
    crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::deregister_organization_admin_account::builders::DeregisterOrganizationAdminAccountOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_deregister_organization_admin_account_input(
    input: &crate::operation::deregister_organization_admin_account::DeregisterOrganizationAdminAccountInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deregister_organization_admin_account_input::ser_deregister_organization_admin_account_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
