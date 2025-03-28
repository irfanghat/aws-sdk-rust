// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_campaign_in_app_message(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CampaignInAppMessage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.body {
        object.key("Body").string(var_1.as_str());
    }
    if let Some(var_2) = &input.content {
        let mut array_3 = object.key("Content").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_in_app_message_content::ser_in_app_message_content(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.custom_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("CustomConfig").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.layout {
        object.key("Layout").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_campaign_in_app_message<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CampaignInAppMessage>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CampaignInAppMessageBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Body" => {
                            builder = builder.set_body(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Content" => {
                            builder = builder
                                .set_content(crate::protocol_serde::shape_list_of_in_app_message_content::de_list_of_in_app_message_content(tokens)?);
                        }
                        "CustomConfig" => {
                            builder = builder.set_custom_config(crate::protocol_serde::shape_map_of_string::de_map_of_string(tokens)?);
                        }
                        "Layout" => {
                            builder = builder.set_layout(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Layout::from(u.as_ref())))
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
