// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_scaling_policy(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ScalingPolicy, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ScalingPolicy::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AutoScalingGroupName") /* AutoScalingGroupName com.amazonaws.autoscaling#ScalingPolicy$AutoScalingGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_scaling_group_name(var_1);
            }
            ,
            s if s.matches("PolicyName") /* PolicyName com.amazonaws.autoscaling#ScalingPolicy$PolicyName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_policy_name(var_2);
            }
            ,
            s if s.matches("PolicyARN") /* PolicyARN com.amazonaws.autoscaling#ScalingPolicy$PolicyARN */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_policy_arn(var_3);
            }
            ,
            s if s.matches("PolicyType") /* PolicyType com.amazonaws.autoscaling#ScalingPolicy$PolicyType */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_policy_type(var_4);
            }
            ,
            s if s.matches("AdjustmentType") /* AdjustmentType com.amazonaws.autoscaling#ScalingPolicy$AdjustmentType */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_adjustment_type(var_5);
            }
            ,
            s if s.matches("MinAdjustmentStep") /* MinAdjustmentStep com.amazonaws.autoscaling#ScalingPolicy$MinAdjustmentStep */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#MinAdjustmentStep`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_adjustment_step(var_6);
            }
            ,
            s if s.matches("MinAdjustmentMagnitude") /* MinAdjustmentMagnitude com.amazonaws.autoscaling#ScalingPolicy$MinAdjustmentMagnitude */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#MinAdjustmentMagnitude`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_adjustment_magnitude(var_7);
            }
            ,
            s if s.matches("ScalingAdjustment") /* ScalingAdjustment com.amazonaws.autoscaling#ScalingPolicy$ScalingAdjustment */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#PolicyIncrement`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_scaling_adjustment(var_8);
            }
            ,
            s if s.matches("Cooldown") /* Cooldown com.amazonaws.autoscaling#ScalingPolicy$Cooldown */ =>  {
                let var_9 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#Cooldown`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_cooldown(var_9);
            }
            ,
            s if s.matches("StepAdjustments") /* StepAdjustments com.amazonaws.autoscaling#ScalingPolicy$StepAdjustments */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_step_adjustments::de_step_adjustments(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_step_adjustments(var_10);
            }
            ,
            s if s.matches("MetricAggregationType") /* MetricAggregationType com.amazonaws.autoscaling#ScalingPolicy$MetricAggregationType */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_metric_aggregation_type(var_11);
            }
            ,
            s if s.matches("EstimatedInstanceWarmup") /* EstimatedInstanceWarmup com.amazonaws.autoscaling#ScalingPolicy$EstimatedInstanceWarmup */ =>  {
                let var_12 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#EstimatedInstanceWarmup`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_estimated_instance_warmup(var_12);
            }
            ,
            s if s.matches("Alarms") /* Alarms com.amazonaws.autoscaling#ScalingPolicy$Alarms */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_alarms::de_alarms(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_alarms(var_13);
            }
            ,
            s if s.matches("TargetTrackingConfiguration") /* TargetTrackingConfiguration com.amazonaws.autoscaling#ScalingPolicy$TargetTrackingConfiguration */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_target_tracking_configuration::de_target_tracking_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_tracking_configuration(var_14);
            }
            ,
            s if s.matches("Enabled") /* Enabled com.amazonaws.autoscaling#ScalingPolicy$Enabled */ =>  {
                let var_15 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.autoscaling#ScalingPolicyEnabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enabled(var_15);
            }
            ,
            s if s.matches("PredictiveScalingConfiguration") /* PredictiveScalingConfiguration com.amazonaws.autoscaling#ScalingPolicy$PredictiveScalingConfiguration */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_predictive_scaling_configuration::de_predictive_scaling_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_predictive_scaling_configuration(var_16);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
