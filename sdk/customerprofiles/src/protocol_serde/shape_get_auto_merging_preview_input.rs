// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_auto_merging_preview_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_auto_merging_preview::GetAutoMergingPreviewInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.conflict_resolution {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ConflictResolution").start_object();
        crate::protocol_serde::shape_conflict_resolution::ser_conflict_resolution(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.consolidation {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Consolidation").start_object();
        crate::protocol_serde::shape_consolidation::ser_consolidation(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.min_allowed_confidence_score_for_merging {
        object.key("MinAllowedConfidenceScoreForMerging").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_5).into()),
        );
    }
    Ok(())
}
