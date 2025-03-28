// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateBasePathMapping`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::set_domain_name):<br>required: **true**<br><p>The domain name of the BasePathMapping resource to create.</p><br>
    ///   - [`domain_name_id(impl Into<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::domain_name_id) / [`set_domain_name_id(Option<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::set_domain_name_id):<br>required: **false**<br><p>The identifier for the domain name resource. Required for private custom domain names.</p><br>
    ///   - [`base_path(impl Into<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::base_path) / [`set_base_path(Option<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::set_base_path):<br>required: **false**<br><p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Specify '(none)' if you do not want callers to specify a base path name after the domain name.</p><br>
    ///   - [`rest_api_id(impl Into<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::set_rest_api_id):<br>required: **true**<br><p>The string identifier of the associated RestApi.</p><br>
    ///   - [`stage(impl Into<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::stage) / [`set_stage(Option<String>)`](crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::set_stage):<br>required: **false**<br><p>The name of the API's stage that you want to use for this mapping. Specify '(none)' if you want callers to explicitly specify the stage name after any base path name.</p><br>
    /// - On success, responds with [`CreateBasePathMappingOutput`](crate::operation::create_base_path_mapping::CreateBasePathMappingOutput) with field(s):
    ///   - [`base_path(Option<String>)`](crate::operation::create_base_path_mapping::CreateBasePathMappingOutput::base_path): <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p>
    ///   - [`rest_api_id(Option<String>)`](crate::operation::create_base_path_mapping::CreateBasePathMappingOutput::rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`stage(Option<String>)`](crate::operation::create_base_path_mapping::CreateBasePathMappingOutput::stage): <p>The name of the associated stage.</p>
    /// - On failure, responds with [`SdkError<CreateBasePathMappingError>`](crate::operation::create_base_path_mapping::CreateBasePathMappingError)
    pub fn create_base_path_mapping(&self) -> crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder {
        crate::operation::create_base_path_mapping::builders::CreateBasePathMappingFluentBuilder::new(self.handle.clone())
    }
}
