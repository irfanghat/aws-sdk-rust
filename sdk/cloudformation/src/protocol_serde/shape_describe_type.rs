// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_type_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_type::DescribeTypeOutput, crate::operation::describe_type::DescribeTypeError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_type::DescribeTypeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_type::DescribeTypeError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CFNRegistryException" => crate::operation::describe_type::DescribeTypeError::CfnRegistryException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CfnRegistryExceptionBuilder::default();
                output = crate::protocol_serde::shape_cfn_registry_exception::de_cfn_registry_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_type::DescribeTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TypeNotFoundException" => crate::operation::describe_type::DescribeTypeError::TypeNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TypeNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_type_not_found_exception::de_type_not_found_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_type::DescribeTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_type::DescribeTypeError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_type_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_type::DescribeTypeOutput, crate::operation::describe_type::DescribeTypeError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_type::builders::DescribeTypeOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_type::de_describe_type(_response_body, output)
            .map_err(crate::operation::describe_type::DescribeTypeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_type(
    inp: &[u8],
    mut builder: crate::operation::describe_type::builders::DescribeTypeOutputBuilder,
) -> std::result::Result<crate::operation::describe_type::builders::DescribeTypeOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeTypeResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeTypeResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeTypeResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeTypeResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Arn") /* Arn com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$Arn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_1);
            }
            ,
            s if s.matches("Type") /* Type com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$Type */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::RegistryType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RegistryType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_2);
            }
            ,
            s if s.matches("TypeName") /* TypeName com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$TypeName */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_name(var_3);
            }
            ,
            s if s.matches("DefaultVersionId") /* DefaultVersionId com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$DefaultVersionId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_default_version_id(var_4);
            }
            ,
            s if s.matches("IsDefaultVersion") /* IsDefaultVersion com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$IsDefaultVersion */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#IsDefaultVersion`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_default_version(var_5);
            }
            ,
            s if s.matches("TypeTestsStatus") /* TypeTestsStatus com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$TypeTestsStatus */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::TypeTestsStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TypeTestsStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type_tests_status(var_6);
            }
            ,
            s if s.matches("TypeTestsStatusDescription") /* TypeTestsStatusDescription com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$TypeTestsStatusDescription */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type_tests_status_description(var_7);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$Description */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_8);
            }
            ,
            s if s.matches("Schema") /* Schema com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$Schema */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_schema(var_9);
            }
            ,
            s if s.matches("ProvisioningType") /* ProvisioningType com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$ProvisioningType */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::ProvisioningType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ProvisioningType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_provisioning_type(var_10);
            }
            ,
            s if s.matches("DeprecatedStatus") /* DeprecatedStatus com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$DeprecatedStatus */ =>  {
                let var_11 =
                    Some(
                        Result::<crate::types::DeprecatedStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DeprecatedStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_deprecated_status(var_11);
            }
            ,
            s if s.matches("LoggingConfig") /* LoggingConfig com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$LoggingConfig */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_logging_config::de_logging_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_logging_config(var_12);
            }
            ,
            s if s.matches("RequiredActivatedTypes") /* RequiredActivatedTypes com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$RequiredActivatedTypes */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_required_activated_types::de_required_activated_types(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_required_activated_types(var_13);
            }
            ,
            s if s.matches("ExecutionRoleArn") /* ExecutionRoleArn com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$ExecutionRoleArn */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_execution_role_arn(var_14);
            }
            ,
            s if s.matches("Visibility") /* Visibility com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$Visibility */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::types::Visibility, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Visibility::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_visibility(var_15);
            }
            ,
            s if s.matches("SourceUrl") /* SourceUrl com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$SourceUrl */ =>  {
                let var_16 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_url(var_16);
            }
            ,
            s if s.matches("DocumentationUrl") /* DocumentationUrl com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$DocumentationUrl */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_documentation_url(var_17);
            }
            ,
            s if s.matches("LastUpdated") /* LastUpdated com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$LastUpdated */ =>  {
                let var_18 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_updated(var_18);
            }
            ,
            s if s.matches("TimeCreated") /* TimeCreated com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$TimeCreated */ =>  {
                let var_19 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudformation#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_time_created(var_19);
            }
            ,
            s if s.matches("ConfigurationSchema") /* ConfigurationSchema com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$ConfigurationSchema */ =>  {
                let var_20 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_configuration_schema(var_20);
            }
            ,
            s if s.matches("PublisherId") /* PublisherId com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$PublisherId */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_publisher_id(var_21);
            }
            ,
            s if s.matches("OriginalTypeName") /* OriginalTypeName com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$OriginalTypeName */ =>  {
                let var_22 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_original_type_name(var_22);
            }
            ,
            s if s.matches("OriginalTypeArn") /* OriginalTypeArn com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$OriginalTypeArn */ =>  {
                let var_23 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_original_type_arn(var_23);
            }
            ,
            s if s.matches("PublicVersionNumber") /* PublicVersionNumber com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$PublicVersionNumber */ =>  {
                let var_24 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_version_number(var_24);
            }
            ,
            s if s.matches("LatestPublicVersion") /* LatestPublicVersion com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$LatestPublicVersion */ =>  {
                let var_25 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_latest_public_version(var_25);
            }
            ,
            s if s.matches("IsActivated") /* IsActivated com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$IsActivated */ =>  {
                let var_26 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#IsActivated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_activated(var_26);
            }
            ,
            s if s.matches("AutoUpdate") /* AutoUpdate com.amazonaws.cloudformation.synthetic#DescribeTypeOutput$AutoUpdate */ =>  {
                let var_27 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.cloudformation#AutoUpdate`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_auto_update(var_27);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeTypeResult tag"));
    };
    Ok(builder)
}
