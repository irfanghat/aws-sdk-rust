// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_batch_load_task;

pub(crate) mod shape_create_database;

pub(crate) mod shape_create_table;

pub(crate) mod shape_delete_database;

pub(crate) mod shape_delete_table;

pub(crate) mod shape_describe_batch_load_task;

pub(crate) mod shape_describe_database;

pub(crate) mod shape_describe_endpoints;

pub(crate) mod shape_describe_table;

pub(crate) mod shape_list_batch_load_tasks;

pub(crate) mod shape_list_databases;

pub(crate) mod shape_list_tables;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_resume_batch_load_task;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_database;

pub(crate) mod shape_update_table;

pub(crate) mod shape_write_records;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_batch_load_task_input;

pub(crate) mod shape_create_database_input;

pub(crate) mod shape_create_table_input;

pub(crate) mod shape_delete_database_input;

pub(crate) mod shape_delete_table_input;

pub(crate) mod shape_describe_batch_load_task_input;

pub(crate) mod shape_describe_database_input;

pub(crate) mod shape_describe_table_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_invalid_endpoint_exception;

pub(crate) mod shape_list_batch_load_tasks_input;

pub(crate) mod shape_list_databases_input;

pub(crate) mod shape_list_tables_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_rejected_records_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_resume_batch_load_task_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_database_input;

pub(crate) mod shape_update_table_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_write_records_input;

pub(crate) mod shape_batch_load_task_description;

pub(crate) mod shape_batch_load_task_list;

pub(crate) mod shape_data_model_configuration;

pub(crate) mod shape_data_source_configuration;

pub(crate) mod shape_database;

pub(crate) mod shape_database_list;

pub(crate) mod shape_endpoints;

pub(crate) mod shape_magnetic_store_write_properties;

pub(crate) mod shape_record;

pub(crate) mod shape_records_ingested;

pub(crate) mod shape_rejected_records;

pub(crate) mod shape_report_configuration;

pub(crate) mod shape_retention_properties;

pub(crate) mod shape_schema;

pub(crate) mod shape_table;

pub(crate) mod shape_table_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_batch_load_progress_report;

pub(crate) mod shape_batch_load_task;

pub(crate) mod shape_csv_configuration;

pub(crate) mod shape_data_model;

pub(crate) mod shape_data_model_s3_configuration;

pub(crate) mod shape_data_source_s3_configuration;

pub(crate) mod shape_dimension;

pub(crate) mod shape_endpoint;

pub(crate) mod shape_magnetic_store_rejected_data_location;

pub(crate) mod shape_measure_value;

pub(crate) mod shape_partition_key;

pub(crate) mod shape_rejected_record;

pub(crate) mod shape_report_s3_configuration;

pub(crate) mod shape_dimension_mapping;

pub(crate) mod shape_mixed_measure_mapping;

pub(crate) mod shape_multi_measure_mappings;

pub(crate) mod shape_partition_key_list;

pub(crate) mod shape_s3_configuration;

pub(crate) mod shape_dimension_mappings;

pub(crate) mod shape_mixed_measure_mapping_list;

pub(crate) mod shape_multi_measure_attribute_mapping;

pub(crate) mod shape_multi_measure_attribute_mapping_list;
