// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_inference_component::_update_inference_component_output::UpdateInferenceComponentOutputBuilder;

pub use crate::operation::update_inference_component::_update_inference_component_input::UpdateInferenceComponentInputBuilder;

impl crate::operation::update_inference_component::builders::UpdateInferenceComponentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_inference_component::UpdateInferenceComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_inference_component::UpdateInferenceComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_inference_component();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateInferenceComponent`.
///
/// <p>Updates an inference component.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateInferenceComponentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_inference_component::builders::UpdateInferenceComponentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_inference_component::UpdateInferenceComponentOutput,
        crate::operation::update_inference_component::UpdateInferenceComponentError,
    > for UpdateInferenceComponentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_inference_component::UpdateInferenceComponentOutput,
            crate::operation::update_inference_component::UpdateInferenceComponentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateInferenceComponentFluentBuilder {
    /// Creates a new `UpdateInferenceComponentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateInferenceComponent as a reference.
    pub fn as_input(&self) -> &crate::operation::update_inference_component::builders::UpdateInferenceComponentInputBuilder {
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
        crate::operation::update_inference_component::UpdateInferenceComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_inference_component::UpdateInferenceComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_inference_component::UpdateInferenceComponent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_inference_component::UpdateInferenceComponent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_inference_component::UpdateInferenceComponentOutput,
        crate::operation::update_inference_component::UpdateInferenceComponentError,
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
    /// <p>The name of the inference component.</p>
    pub fn inference_component_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.inference_component_name(input.into());
        self
    }
    /// <p>The name of the inference component.</p>
    pub fn set_inference_component_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_inference_component_name(input);
        self
    }
    /// <p>The name of the inference component.</p>
    pub fn get_inference_component_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_inference_component_name()
    }
    /// <p>Details about the resources to deploy with this inference component, including the model, container, and compute resources.</p>
    pub fn specification(mut self, input: crate::types::InferenceComponentSpecification) -> Self {
        self.inner = self.inner.specification(input);
        self
    }
    /// <p>Details about the resources to deploy with this inference component, including the model, container, and compute resources.</p>
    pub fn set_specification(mut self, input: ::std::option::Option<crate::types::InferenceComponentSpecification>) -> Self {
        self.inner = self.inner.set_specification(input);
        self
    }
    /// <p>Details about the resources to deploy with this inference component, including the model, container, and compute resources.</p>
    pub fn get_specification(&self) -> &::std::option::Option<crate::types::InferenceComponentSpecification> {
        self.inner.get_specification()
    }
    /// <p>Runtime settings for a model that is deployed with an inference component.</p>
    pub fn runtime_config(mut self, input: crate::types::InferenceComponentRuntimeConfig) -> Self {
        self.inner = self.inner.runtime_config(input);
        self
    }
    /// <p>Runtime settings for a model that is deployed with an inference component.</p>
    pub fn set_runtime_config(mut self, input: ::std::option::Option<crate::types::InferenceComponentRuntimeConfig>) -> Self {
        self.inner = self.inner.set_runtime_config(input);
        self
    }
    /// <p>Runtime settings for a model that is deployed with an inference component.</p>
    pub fn get_runtime_config(&self) -> &::std::option::Option<crate::types::InferenceComponentRuntimeConfig> {
        self.inner.get_runtime_config()
    }
    /// <p>The deployment configuration for the inference component. The configuration contains the desired deployment strategy and rollback settings.</p>
    pub fn deployment_config(mut self, input: crate::types::InferenceComponentDeploymentConfig) -> Self {
        self.inner = self.inner.deployment_config(input);
        self
    }
    /// <p>The deployment configuration for the inference component. The configuration contains the desired deployment strategy and rollback settings.</p>
    pub fn set_deployment_config(mut self, input: ::std::option::Option<crate::types::InferenceComponentDeploymentConfig>) -> Self {
        self.inner = self.inner.set_deployment_config(input);
        self
    }
    /// <p>The deployment configuration for the inference component. The configuration contains the desired deployment strategy and rollback settings.</p>
    pub fn get_deployment_config(&self) -> &::std::option::Option<crate::types::InferenceComponentDeploymentConfig> {
        self.inner.get_deployment_config()
    }
}
