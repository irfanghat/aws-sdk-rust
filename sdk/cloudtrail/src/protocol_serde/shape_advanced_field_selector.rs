// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_advanced_field_selector(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AdvancedFieldSelector,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Field").string(input.field.as_str());
    }
    if let Some(var_1) = &input.equals {
        let mut array_2 = object.key("Equals").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.starts_with {
        let mut array_5 = object.key("StartsWith").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.ends_with {
        let mut array_8 = object.key("EndsWith").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.not_equals {
        let mut array_11 = object.key("NotEquals").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.not_starts_with {
        let mut array_14 = object.key("NotStartsWith").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.not_ends_with {
        let mut array_17 = object.key("NotEndsWith").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub(crate) fn de_advanced_field_selector<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AdvancedFieldSelector>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AdvancedFieldSelectorBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Field" => {
                            builder = builder.set_field(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Equals" => {
                            builder = builder.set_equals(crate::protocol_serde::shape_operator::de_operator(tokens)?);
                        }
                        "StartsWith" => {
                            builder = builder.set_starts_with(crate::protocol_serde::shape_operator::de_operator(tokens)?);
                        }
                        "EndsWith" => {
                            builder = builder.set_ends_with(crate::protocol_serde::shape_operator::de_operator(tokens)?);
                        }
                        "NotEquals" => {
                            builder = builder.set_not_equals(crate::protocol_serde::shape_operator::de_operator(tokens)?);
                        }
                        "NotStartsWith" => {
                            builder = builder.set_not_starts_with(crate::protocol_serde::shape_operator::de_operator(tokens)?);
                        }
                        "NotEndsWith" => {
                            builder = builder.set_not_ends_with(crate::protocol_serde::shape_operator::de_operator(tokens)?);
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
            Ok(Some(crate::serde_util::advanced_field_selector_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
