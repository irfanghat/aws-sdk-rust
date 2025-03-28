// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bucket_level(
    input: &crate::types::BucketLevel,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.activity_metrics {
        let inner_writer = scope.start_el("ActivityMetrics");
        crate::protocol_serde::shape_activity_metrics::ser_activity_metrics(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.prefix_level {
        let inner_writer = scope.start_el("PrefixLevel");
        crate::protocol_serde::shape_prefix_level::ser_prefix_level(var_2, inner_writer)?
    }
    if let Some(var_3) = &input.advanced_cost_optimization_metrics {
        let inner_writer = scope.start_el("AdvancedCostOptimizationMetrics");
        crate::protocol_serde::shape_advanced_cost_optimization_metrics::ser_advanced_cost_optimization_metrics(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.advanced_data_protection_metrics {
        let inner_writer = scope.start_el("AdvancedDataProtectionMetrics");
        crate::protocol_serde::shape_advanced_data_protection_metrics::ser_advanced_data_protection_metrics(var_4, inner_writer)?
    }
    if let Some(var_5) = &input.detailed_status_codes_metrics {
        let inner_writer = scope.start_el("DetailedStatusCodesMetrics");
        crate::protocol_serde::shape_detailed_status_codes_metrics::ser_detailed_status_codes_metrics(var_5, inner_writer)?
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_bucket_level(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::BucketLevel, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::BucketLevel::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ActivityMetrics") /* ActivityMetrics com.amazonaws.s3control#BucketLevel$ActivityMetrics */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_activity_metrics::de_activity_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_activity_metrics(var_6);
            }
            ,
            s if s.matches("PrefixLevel") /* PrefixLevel com.amazonaws.s3control#BucketLevel$PrefixLevel */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_prefix_level::de_prefix_level(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_prefix_level(var_7);
            }
            ,
            s if s.matches("AdvancedCostOptimizationMetrics") /* AdvancedCostOptimizationMetrics com.amazonaws.s3control#BucketLevel$AdvancedCostOptimizationMetrics */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_advanced_cost_optimization_metrics::de_advanced_cost_optimization_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_advanced_cost_optimization_metrics(var_8);
            }
            ,
            s if s.matches("AdvancedDataProtectionMetrics") /* AdvancedDataProtectionMetrics com.amazonaws.s3control#BucketLevel$AdvancedDataProtectionMetrics */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_advanced_data_protection_metrics::de_advanced_data_protection_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_advanced_data_protection_metrics(var_9);
            }
            ,
            s if s.matches("DetailedStatusCodesMetrics") /* DetailedStatusCodesMetrics com.amazonaws.s3control#BucketLevel$DetailedStatusCodesMetrics */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_detailed_status_codes_metrics::de_detailed_status_codes_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_detailed_status_codes_metrics(var_10);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
