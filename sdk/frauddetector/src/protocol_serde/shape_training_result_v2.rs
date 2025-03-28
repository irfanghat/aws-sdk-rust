// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_training_result_v2<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TrainingResultV2>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TrainingResultV2Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "dataValidationMetrics" => {
                            builder = builder.set_data_validation_metrics(
                                crate::protocol_serde::shape_data_validation_metrics::de_data_validation_metrics(tokens)?,
                            );
                        }
                        "trainingMetricsV2" => {
                            builder =
                                builder.set_training_metrics_v2(crate::protocol_serde::shape_training_metrics_v2::de_training_metrics_v2(tokens)?);
                        }
                        "variableImportanceMetrics" => {
                            builder = builder.set_variable_importance_metrics(
                                crate::protocol_serde::shape_variable_importance_metrics::de_variable_importance_metrics(tokens)?,
                            );
                        }
                        "aggregatedVariablesImportanceMetrics" => {
                            builder = builder.set_aggregated_variables_importance_metrics(
                                crate::protocol_serde::shape_aggregated_variables_importance_metrics::de_aggregated_variables_importance_metrics(
                                    tokens,
                                )?,
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
