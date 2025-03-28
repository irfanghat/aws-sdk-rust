// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_schema_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_schema::CreateSchemaInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("RegistryId").start_object();
        crate::protocol_serde::shape_registry_id::ser_registry_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.schema_name {
        object.key("SchemaName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.data_format {
        object.key("DataFormat").string(var_4.as_str());
    }
    if let Some(var_5) = &input.compatibility {
        object.key("Compatibility").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.schema_definition {
        object.key("SchemaDefinition").string(var_11.as_str());
    }
    Ok(())
}
