// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_shard_group_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_db_shard_group::ModifyDbShardGroupOutput,
    crate::operation::modify_db_shard_group::ModifyDBShardGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBShardGroupAlreadyExists" => crate::operation::modify_db_shard_group::ModifyDBShardGroupError::DbShardGroupAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DbShardGroupAlreadyExistsFaultBuilder::default();
                output = crate::protocol_serde::shape_db_shard_group_already_exists_fault::de_db_shard_group_already_exists_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DBShardGroupNotFound" => crate::operation::modify_db_shard_group::ModifyDBShardGroupError::DbShardGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DbShardGroupNotFoundFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_db_shard_group_not_found_fault::de_db_shard_group_not_found_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDBClusterStateFault" => crate::operation::modify_db_shard_group::ModifyDBShardGroupError::InvalidDbClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDbClusterStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_db_cluster_state_fault::de_invalid_db_cluster_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::modify_db_shard_group::ModifyDBShardGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_shard_group_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_db_shard_group::ModifyDbShardGroupOutput,
    crate::operation::modify_db_shard_group::ModifyDBShardGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_db_shard_group::builders::ModifyDbShardGroupOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_db_shard_group::de_modify_db_shard_group(_response_body, output)
            .map_err(crate::operation::modify_db_shard_group::ModifyDBShardGroupError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_db_shard_group(
    inp: &[u8],
    mut builder: crate::operation::modify_db_shard_group::builders::ModifyDbShardGroupOutputBuilder,
) -> std::result::Result<crate::operation::modify_db_shard_group::builders::ModifyDbShardGroupOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyDBShardGroupResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyDBShardGroupResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifyDBShardGroupResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifyDBShardGroupResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DBShardGroupResourceId") /* DBShardGroupResourceId com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$DBShardGroupResourceId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_shard_group_resource_id(var_1);
            }
            ,
            s if s.matches("DBShardGroupIdentifier") /* DBShardGroupIdentifier com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$DBShardGroupIdentifier */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_shard_group_identifier(var_2);
            }
            ,
            s if s.matches("DBClusterIdentifier") /* DBClusterIdentifier com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$DBClusterIdentifier */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_identifier(var_3);
            }
            ,
            s if s.matches("MaxACU") /* MaxACU com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$MaxACU */ =>  {
                let var_4 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.rds#DoubleOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_acu(var_4);
            }
            ,
            s if s.matches("MinACU") /* MinACU com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$MinACU */ =>  {
                let var_5 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.rds#DoubleOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_acu(var_5);
            }
            ,
            s if s.matches("ComputeRedundancy") /* ComputeRedundancy com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$ComputeRedundancy */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.rds#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_compute_redundancy(var_6);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$Status */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_7);
            }
            ,
            s if s.matches("PubliclyAccessible") /* PubliclyAccessible com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$PubliclyAccessible */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_publicly_accessible(var_8);
            }
            ,
            s if s.matches("Endpoint") /* Endpoint com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$Endpoint */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_endpoint(var_9);
            }
            ,
            s if s.matches("DBShardGroupArn") /* DBShardGroupArn com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$DBShardGroupArn */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_shard_group_arn(var_10);
            }
            ,
            s if s.matches("TagList") /* TagList com.amazonaws.rds.synthetic#ModifyDBShardGroupOutput$TagList */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tag_list(var_11);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyDBShardGroupResult tag"));
    };
    Ok(builder)
}
