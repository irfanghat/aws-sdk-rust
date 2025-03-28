// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_instance_access_details::_get_instance_access_details_output::GetInstanceAccessDetailsOutputBuilder;

pub use crate::operation::get_instance_access_details::_get_instance_access_details_input::GetInstanceAccessDetailsInputBuilder;

impl crate::operation::get_instance_access_details::builders::GetInstanceAccessDetailsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_instance_access_details::GetInstanceAccessDetailsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_instance_access_details();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetInstanceAccessDetails`.
///
/// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p>
/// <p>The <code>get instance access details</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>instance name</code>. For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetInstanceAccessDetailsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_instance_access_details::builders::GetInstanceAccessDetailsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsOutput,
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsError,
    > for GetInstanceAccessDetailsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_instance_access_details::GetInstanceAccessDetailsOutput,
            crate::operation::get_instance_access_details::GetInstanceAccessDetailsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetInstanceAccessDetailsFluentBuilder {
    /// Creates a new `GetInstanceAccessDetailsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetInstanceAccessDetails as a reference.
    pub fn as_input(&self) -> &crate::operation::get_instance_access_details::builders::GetInstanceAccessDetailsInputBuilder {
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
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_instance_access_details::GetInstanceAccessDetailsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_instance_access_details::GetInstanceAccessDetails::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_instance_access_details::GetInstanceAccessDetails::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsOutput,
        crate::operation::get_instance_access_details::GetInstanceAccessDetailsError,
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
    /// <p>The name of the instance to access.</p>
    pub fn instance_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_name(input.into());
        self
    }
    /// <p>The name of the instance to access.</p>
    pub fn set_instance_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_name(input);
        self
    }
    /// <p>The name of the instance to access.</p>
    pub fn get_instance_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_name()
    }
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    pub fn protocol(mut self, input: crate::types::InstanceAccessProtocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::InstanceAccessProtocol>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<crate::types::InstanceAccessProtocol> {
        self.inner.get_protocol()
    }
}
