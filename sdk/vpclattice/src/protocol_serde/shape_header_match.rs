// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_header_match(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::HeaderMatch,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.r#match {
        #[allow(unused_mut)]
        let mut object_2 = object.key("match").start_object();
        crate::protocol_serde::shape_header_match_type::ser_header_match_type(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.case_sensitive {
        object.key("caseSensitive").boolean(*var_3);
    }
    Ok(())
}

pub(crate) fn de_header_match<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HeaderMatch>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HeaderMatchBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "match" => {
                            builder = builder.set_match(crate::protocol_serde::shape_header_match_type::de_header_match_type(tokens)?);
                        }
                        "caseSensitive" => {
                            builder = builder.set_case_sensitive(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
            Ok(Some(crate::serde_util::header_match_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
