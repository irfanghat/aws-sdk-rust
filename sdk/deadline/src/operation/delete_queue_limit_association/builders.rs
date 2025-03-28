// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_queue_limit_association::_delete_queue_limit_association_output::DeleteQueueLimitAssociationOutputBuilder;

pub use crate::operation::delete_queue_limit_association::_delete_queue_limit_association_input::DeleteQueueLimitAssociationInputBuilder;

impl crate::operation::delete_queue_limit_association::builders::DeleteQueueLimitAssociationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_queue_limit_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteQueueLimitAssociation`.
///
/// <p>Removes the association between a queue and a limit. You must use the <code>UpdateQueueLimitAssociation</code> operation to set the status to <code>STOP_LIMIT_USAGE_AND_COMPLETE_TASKS</code> or <code>STOP_LIMIT_USAGE_AND_CANCEL_TASKS</code>. The status does not change immediately. Use the <code>GetQueueLimitAssociation</code> operation to see if the status changed to <code>STOPPED</code> before deleting the association.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteQueueLimitAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_queue_limit_association::builders::DeleteQueueLimitAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationOutput,
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationError,
    > for DeleteQueueLimitAssociationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationOutput,
            crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteQueueLimitAssociationFluentBuilder {
    /// Creates a new `DeleteQueueLimitAssociationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteQueueLimitAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_queue_limit_association::builders::DeleteQueueLimitAssociationInputBuilder {
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
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationOutput,
        crate::operation::delete_queue_limit_association::DeleteQueueLimitAssociationError,
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
    /// <p>The unique identifier of the farm that contains the queue and limit to disassociate.</p>
    pub fn farm_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.farm_id(input.into());
        self
    }
    /// <p>The unique identifier of the farm that contains the queue and limit to disassociate.</p>
    pub fn set_farm_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_farm_id(input);
        self
    }
    /// <p>The unique identifier of the farm that contains the queue and limit to disassociate.</p>
    pub fn get_farm_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_farm_id()
    }
    /// <p>The unique identifier of the queue to disassociate.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_id(input.into());
        self
    }
    /// <p>The unique identifier of the queue to disassociate.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_id(input);
        self
    }
    /// <p>The unique identifier of the queue to disassociate.</p>
    pub fn get_queue_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_id()
    }
    /// <p>The unique identifier of the limit to disassociate.</p>
    pub fn limit_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.limit_id(input.into());
        self
    }
    /// <p>The unique identifier of the limit to disassociate.</p>
    pub fn set_limit_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_limit_id(input);
        self
    }
    /// <p>The unique identifier of the limit to disassociate.</p>
    pub fn get_limit_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_limit_id()
    }
}
