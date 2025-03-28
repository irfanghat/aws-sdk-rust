// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_distribution_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_distribution::UpdateDistributionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.distribution_name {
        object.key("distributionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.origin {
        #[allow(unused_mut)]
        let mut object_3 = object.key("origin").start_object();
        crate::protocol_serde::shape_input_origin::ser_input_origin(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.default_cache_behavior {
        #[allow(unused_mut)]
        let mut object_5 = object.key("defaultCacheBehavior").start_object();
        crate::protocol_serde::shape_cache_behavior::ser_cache_behavior(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.cache_behavior_settings {
        #[allow(unused_mut)]
        let mut object_7 = object.key("cacheBehaviorSettings").start_object();
        crate::protocol_serde::shape_cache_settings::ser_cache_settings(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.cache_behaviors {
        let mut array_9 = object.key("cacheBehaviors").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_cache_behavior_per_path::ser_cache_behavior_per_path(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.is_enabled {
        object.key("isEnabled").boolean(*var_12);
    }
    if let Some(var_13) = &input.viewer_minimum_tls_protocol_version {
        object.key("viewerMinimumTlsProtocolVersion").string(var_13.as_str());
    }
    if let Some(var_14) = &input.certificate_name {
        object.key("certificateName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.use_default_certificate {
        object.key("useDefaultCertificate").boolean(*var_15);
    }
    Ok(())
}
