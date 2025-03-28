// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_lifecycle_configuration::_put_lifecycle_configuration_output::PutLifecycleConfigurationOutputBuilder;

pub use crate::operation::put_lifecycle_configuration::_put_lifecycle_configuration_input::PutLifecycleConfigurationInputBuilder;

impl crate::operation::put_lifecycle_configuration::builders::PutLifecycleConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_lifecycle_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutLifecycleConfiguration`.
///
/// <p>Use this action to manage storage for your file system. A <code>LifecycleConfiguration</code> consists of one or more <code>LifecyclePolicy</code> objects that define the following:</p>
/// <ul>
/// <li>
/// <p><b> <code>TransitionToIA</code> </b> – When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access (IA) storage.</p></li>
/// <li>
/// <p><b> <code>TransitionToArchive</code> </b> – When to move files in the file system from their current storage class (either IA or Standard storage) into the Archive storage.</p>
/// <p>File systems cannot transition into Archive storage before transitioning into IA storage. Therefore, TransitionToArchive must either not be set or must be later than TransitionToIA.</p><note>
/// <p>The Archive storage class is available only for file systems that use the Elastic throughput mode and the General Purpose performance mode.</p>
/// </note></li>
/// </ul>
/// <ul>
/// <li>
/// <p><b> <code>TransitionToPrimaryStorageClass</code> </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA or Archive storage.</p></li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/lifecycle-management-efs.html"> Managing file system storage</a>.</p>
/// <p>Each Amazon EFS file system supports one lifecycle configuration, which applies to all files in the file system. If a <code>LifecycleConfiguration</code> object already exists for the specified file system, a <code>PutLifecycleConfiguration</code> call modifies the existing configuration. A <code>PutLifecycleConfiguration</code> call with an empty <code>LifecyclePolicies</code> array in the request body deletes any existing <code>LifecycleConfiguration</code>. In the request, specify the following:</p>
/// <ul>
/// <li>
/// <p>The ID for the file system for which you are enabling, disabling, or modifying lifecycle management.</p></li>
/// <li>
/// <p>A <code>LifecyclePolicies</code> array of <code>LifecyclePolicy</code> objects that define when to move files to IA storage, to Archive storage, and back to primary storage.</p><note>
/// <p>Amazon EFS requires that each <code>LifecyclePolicy</code> object have only have a single transition, so the <code>LifecyclePolicies</code> array needs to be structured with separate <code>LifecyclePolicy</code> objects. See the example requests in the following section for more information.</p>
/// </note></li>
/// </ul>
/// <p>This operation requires permissions for the <code>elasticfilesystem:PutLifecycleConfiguration</code> operation.</p>
/// <p>To apply a <code>LifecycleConfiguration</code> object to an encrypted file system, you need the same Key Management Service permissions as when you created the encrypted file system.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutLifecycleConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_lifecycle_configuration::builders::PutLifecycleConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationOutput,
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationError,
    > for PutLifecycleConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationOutput,
            crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutLifecycleConfigurationFluentBuilder {
    /// Creates a new `PutLifecycleConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutLifecycleConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_lifecycle_configuration::builders::PutLifecycleConfigurationInputBuilder {
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
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_lifecycle_configuration::PutLifecycleConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_lifecycle_configuration::PutLifecycleConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationOutput,
        crate::operation::put_lifecycle_configuration::PutLifecycleConfigurationError,
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
    /// <p>The ID of the file system for which you are creating the <code>LifecycleConfiguration</code> object (String).</p>
    pub fn file_system_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_system_id(input.into());
        self
    }
    /// <p>The ID of the file system for which you are creating the <code>LifecycleConfiguration</code> object (String).</p>
    pub fn set_file_system_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_file_system_id(input);
        self
    }
    /// <p>The ID of the file system for which you are creating the <code>LifecycleConfiguration</code> object (String).</p>
    pub fn get_file_system_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_file_system_id()
    }
    ///
    /// Appends an item to `LifecyclePolicies`.
    ///
    /// To override the contents of this collection use [`set_lifecycle_policies`](Self::set_lifecycle_policies).
    ///
    /// <p>An array of <code>LifecyclePolicy</code> objects that define the file system's <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object informs lifecycle management of the following:</p>
    /// <ul>
    /// <li>
    /// <p><b> <code>TransitionToIA</code> </b> – When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access (IA) storage.</p></li>
    /// <li>
    /// <p><b> <code>TransitionToArchive</code> </b> – When to move files in the file system from their current storage class (either IA or Standard storage) into the Archive storage.</p>
    /// <p>File systems cannot transition into Archive storage before transitioning into IA storage. Therefore, TransitionToArchive must either not be set or must be later than TransitionToIA.</p><note>
    /// <p>The Archive storage class is available only for file systems that use the Elastic throughput mode and the General Purpose performance mode.</p>
    /// </note></li>
    /// <li>
    /// <p><b> <code>TransitionToPrimaryStorageClass</code> </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA or Archive storage.</p></li>
    /// </ul><note>
    /// <p>When using the <code>put-lifecycle-configuration</code> CLI command or the <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each <code>LifecyclePolicy</code> object have only a single transition. This means that in a request body, <code>LifecyclePolicies</code> must be structured as an array of <code>LifecyclePolicy</code> objects, one object for each storage transition. See the example requests in the following section for more information.</p>
    /// </note>
    pub fn lifecycle_policies(mut self, input: crate::types::LifecyclePolicy) -> Self {
        self.inner = self.inner.lifecycle_policies(input);
        self
    }
    /// <p>An array of <code>LifecyclePolicy</code> objects that define the file system's <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object informs lifecycle management of the following:</p>
    /// <ul>
    /// <li>
    /// <p><b> <code>TransitionToIA</code> </b> – When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access (IA) storage.</p></li>
    /// <li>
    /// <p><b> <code>TransitionToArchive</code> </b> – When to move files in the file system from their current storage class (either IA or Standard storage) into the Archive storage.</p>
    /// <p>File systems cannot transition into Archive storage before transitioning into IA storage. Therefore, TransitionToArchive must either not be set or must be later than TransitionToIA.</p><note>
    /// <p>The Archive storage class is available only for file systems that use the Elastic throughput mode and the General Purpose performance mode.</p>
    /// </note></li>
    /// <li>
    /// <p><b> <code>TransitionToPrimaryStorageClass</code> </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA or Archive storage.</p></li>
    /// </ul><note>
    /// <p>When using the <code>put-lifecycle-configuration</code> CLI command or the <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each <code>LifecyclePolicy</code> object have only a single transition. This means that in a request body, <code>LifecyclePolicies</code> must be structured as an array of <code>LifecyclePolicy</code> objects, one object for each storage transition. See the example requests in the following section for more information.</p>
    /// </note>
    pub fn set_lifecycle_policies(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LifecyclePolicy>>) -> Self {
        self.inner = self.inner.set_lifecycle_policies(input);
        self
    }
    /// <p>An array of <code>LifecyclePolicy</code> objects that define the file system's <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object informs lifecycle management of the following:</p>
    /// <ul>
    /// <li>
    /// <p><b> <code>TransitionToIA</code> </b> – When to move files in the file system from primary storage (Standard storage class) into the Infrequent Access (IA) storage.</p></li>
    /// <li>
    /// <p><b> <code>TransitionToArchive</code> </b> – When to move files in the file system from their current storage class (either IA or Standard storage) into the Archive storage.</p>
    /// <p>File systems cannot transition into Archive storage before transitioning into IA storage. Therefore, TransitionToArchive must either not be set or must be later than TransitionToIA.</p><note>
    /// <p>The Archive storage class is available only for file systems that use the Elastic throughput mode and the General Purpose performance mode.</p>
    /// </note></li>
    /// <li>
    /// <p><b> <code>TransitionToPrimaryStorageClass</code> </b> – Whether to move files in the file system back to primary storage (Standard storage class) after they are accessed in IA or Archive storage.</p></li>
    /// </ul><note>
    /// <p>When using the <code>put-lifecycle-configuration</code> CLI command or the <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each <code>LifecyclePolicy</code> object have only a single transition. This means that in a request body, <code>LifecyclePolicies</code> must be structured as an array of <code>LifecyclePolicy</code> objects, one object for each storage transition. See the example requests in the following section for more information.</p>
    /// </note>
    pub fn get_lifecycle_policies(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LifecyclePolicy>> {
        self.inner.get_lifecycle_policies()
    }
}
