// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_dominant_language_detection_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_dominant_language_detection_job::StartDominantLanguageDetectionJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("InputDataConfig").start_object();
        crate::protocol_serde::shape_input_data_config::ser_input_data_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("OutputDataConfig").start_object();
        crate::protocol_serde::shape_output_data_config::ser_output_data_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.job_name {
        object.key("JobName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.volume_kms_key_id {
        object.key("VolumeKmsKeyId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_10 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}
