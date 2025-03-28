// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::grant_permissions::_grant_permissions_output::GrantPermissionsOutputBuilder;

pub use crate::operation::grant_permissions::_grant_permissions_input::GrantPermissionsInputBuilder;

impl crate::operation::grant_permissions::builders::GrantPermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::grant_permissions::GrantPermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::grant_permissions::GrantPermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.grant_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GrantPermissions`.
///
/// <p>Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3.</p>
/// <p>For information about permissions, see <a href="https://docs.aws.amazon.com/lake-formation/latest/dg/security-data-access.html">Security and Access Control to Metadata and Data</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GrantPermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::grant_permissions::builders::GrantPermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::grant_permissions::GrantPermissionsOutput,
        crate::operation::grant_permissions::GrantPermissionsError,
    > for GrantPermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::grant_permissions::GrantPermissionsOutput,
            crate::operation::grant_permissions::GrantPermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GrantPermissionsFluentBuilder {
    /// Creates a new `GrantPermissionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GrantPermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::grant_permissions::builders::GrantPermissionsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::grant_permissions::GrantPermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::grant_permissions::GrantPermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::grant_permissions::GrantPermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::grant_permissions::GrantPermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::grant_permissions::GrantPermissionsOutput,
        crate::operation::grant_permissions::GrantPermissionsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.</p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog_id()
    }
    /// <p>The principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles, and they are defined by their principal type and their ARN.</p>
    /// <p>Note that if you define a resource with a particular ARN, then later delete, and recreate a resource with that same ARN, the resource maintains the permissions already granted.</p>
    pub fn principal(mut self, input: crate::types::DataLakePrincipal) -> Self {
        self.inner = self.inner.principal(input);
        self
    }
    /// <p>The principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles, and they are defined by their principal type and their ARN.</p>
    /// <p>Note that if you define a resource with a particular ARN, then later delete, and recreate a resource with that same ARN, the resource maintains the permissions already granted.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<crate::types::DataLakePrincipal>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>The principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles, and they are defined by their principal type and their ARN.</p>
    /// <p>Note that if you define a resource with a particular ARN, then later delete, and recreate a resource with that same ARN, the resource maintains the permissions already granted.</p>
    pub fn get_principal(&self) -> &::std::option::Option<crate::types::DataLakePrincipal> {
        self.inner.get_principal()
    }
    /// <p>The resource to which permissions are to be granted. Resources in Lake Formation are the Data Catalog, databases, and tables.</p>
    pub fn resource(mut self, input: crate::types::Resource) -> Self {
        self.inner = self.inner.resource(input);
        self
    }
    /// <p>The resource to which permissions are to be granted. Resources in Lake Formation are the Data Catalog, databases, and tables.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<crate::types::Resource>) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>The resource to which permissions are to be granted. Resources in Lake Formation are the Data Catalog, databases, and tables.</p>
    pub fn get_resource(&self) -> &::std::option::Option<crate::types::Resource> {
        self.inner.get_resource()
    }
    ///
    /// Appends an item to `Permissions`.
    ///
    /// To override the contents of this collection use [`set_permissions`](Self::set_permissions).
    ///
    /// <p>The permissions granted to the principal on the resource. Lake Formation defines privileges to grant and revoke access to metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. Lake Formation requires that each principal be authorized to perform a specific task on Lake Formation resources.</p>
    pub fn permissions(mut self, input: crate::types::Permission) -> Self {
        self.inner = self.inner.permissions(input);
        self
    }
    /// <p>The permissions granted to the principal on the resource. Lake Formation defines privileges to grant and revoke access to metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. Lake Formation requires that each principal be authorized to perform a specific task on Lake Formation resources.</p>
    pub fn set_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Permission>>) -> Self {
        self.inner = self.inner.set_permissions(input);
        self
    }
    /// <p>The permissions granted to the principal on the resource. Lake Formation defines privileges to grant and revoke access to metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. Lake Formation requires that each principal be authorized to perform a specific task on Lake Formation resources.</p>
    pub fn get_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Permission>> {
        self.inner.get_permissions()
    }
    /// <p>A Lake Formation condition, which applies to permissions and opt-ins that contain an expression.</p>
    pub fn condition(mut self, input: crate::types::Condition) -> Self {
        self.inner = self.inner.condition(input);
        self
    }
    /// <p>A Lake Formation condition, which applies to permissions and opt-ins that contain an expression.</p>
    pub fn set_condition(mut self, input: ::std::option::Option<crate::types::Condition>) -> Self {
        self.inner = self.inner.set_condition(input);
        self
    }
    /// <p>A Lake Formation condition, which applies to permissions and opt-ins that contain an expression.</p>
    pub fn get_condition(&self) -> &::std::option::Option<crate::types::Condition> {
        self.inner.get_condition()
    }
    ///
    /// Appends an item to `PermissionsWithGrantOption`.
    ///
    /// To override the contents of this collection use [`set_permissions_with_grant_option`](Self::set_permissions_with_grant_option).
    ///
    /// <p>Indicates a list of the granted permissions that the principal may pass to other users. These permissions may only be a subset of the permissions granted in the <code>Privileges</code>.</p>
    pub fn permissions_with_grant_option(mut self, input: crate::types::Permission) -> Self {
        self.inner = self.inner.permissions_with_grant_option(input);
        self
    }
    /// <p>Indicates a list of the granted permissions that the principal may pass to other users. These permissions may only be a subset of the permissions granted in the <code>Privileges</code>.</p>
    pub fn set_permissions_with_grant_option(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Permission>>) -> Self {
        self.inner = self.inner.set_permissions_with_grant_option(input);
        self
    }
    /// <p>Indicates a list of the granted permissions that the principal may pass to other users. These permissions may only be a subset of the permissions granted in the <code>Privileges</code>.</p>
    pub fn get_permissions_with_grant_option(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Permission>> {
        self.inner.get_permissions_with_grant_option()
    }
}
