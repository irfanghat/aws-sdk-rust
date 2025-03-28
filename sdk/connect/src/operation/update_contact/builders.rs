// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact::_update_contact_output::UpdateContactOutputBuilder;

pub use crate::operation::update_contact::_update_contact_input::UpdateContactInputBuilder;

impl crate::operation::update_contact::builders::UpdateContactInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_contact::UpdateContactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_contact::UpdateContactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_contact();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateContact`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Adds or updates user-defined contact information associated with the specified contact. At least one field to be updated must be present in the request.</p><important>
/// <p>You can add or update user-defined contact information for both ongoing and completed contacts.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateContactFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_contact::builders::UpdateContactInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_contact::UpdateContactOutput,
        crate::operation::update_contact::UpdateContactError,
    > for UpdateContactFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_contact::UpdateContactOutput,
            crate::operation::update_contact::UpdateContactError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateContactFluentBuilder {
    /// Creates a new `UpdateContactFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateContact as a reference.
    pub fn as_input(&self) -> &crate::operation::update_contact::builders::UpdateContactInputBuilder {
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
        crate::operation::update_contact::UpdateContactOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_contact::UpdateContactError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_contact::UpdateContact::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_contact::UpdateContact::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_contact::UpdateContactOutput,
        crate::operation::update_contact::UpdateContactError,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with your contact center.</p>
    pub fn contact_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_id(input.into());
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with your contact center.</p>
    pub fn set_contact_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_id(input);
        self
    }
    /// <p>The identifier of the contact. This is the identifier of the contact associated with the first interaction with your contact center.</p>
    pub fn get_contact_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_id()
    }
    /// <p>The name of the contact.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the contact.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the contact.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description of the contact.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the contact.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the contact.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Adds a key-value pair to `References`.
    ///
    /// To override the contents of this collection use [`set_references`](Self::set_references).
    ///
    /// <p>Well-formed data on contact, shown to agents on Contact Control Panel (CCP).</p>
    pub fn references(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::Reference) -> Self {
        self.inner = self.inner.references(k.into(), v);
        self
    }
    /// <p>Well-formed data on contact, shown to agents on Contact Control Panel (CCP).</p>
    pub fn set_references(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::Reference>>,
    ) -> Self {
        self.inner = self.inner.set_references(input);
        self
    }
    /// <p>Well-formed data on contact, shown to agents on Contact Control Panel (CCP).</p>
    pub fn get_references(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::Reference>> {
        self.inner.get_references()
    }
    ///
    /// Adds a key-value pair to `SegmentAttributes`.
    ///
    /// To override the contents of this collection use [`set_segment_attributes`](Self::set_segment_attributes).
    ///
    /// <p>A set of system defined key-value pairs stored on individual contact segments (unique contact ID) using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows.</p>
    /// <p>Attribute keys can include only alphanumeric, -, and _.</p>
    /// <p>This field can be used to show channel subtype, such as <code>connect:Guide</code>.</p>
    /// <p>Currently Contact Expiry is the only segment attribute which can be updated by using the UpdateContact API.</p>
    pub fn segment_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::SegmentAttributeValue) -> Self {
        self.inner = self.inner.segment_attributes(k.into(), v);
        self
    }
    /// <p>A set of system defined key-value pairs stored on individual contact segments (unique contact ID) using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows.</p>
    /// <p>Attribute keys can include only alphanumeric, -, and _.</p>
    /// <p>This field can be used to show channel subtype, such as <code>connect:Guide</code>.</p>
    /// <p>Currently Contact Expiry is the only segment attribute which can be updated by using the UpdateContact API.</p>
    pub fn set_segment_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::SegmentAttributeValue>>,
    ) -> Self {
        self.inner = self.inner.set_segment_attributes(input);
        self
    }
    /// <p>A set of system defined key-value pairs stored on individual contact segments (unique contact ID) using an attribute map. The attributes are standard Amazon Connect attributes. They can be accessed in flows.</p>
    /// <p>Attribute keys can include only alphanumeric, -, and _.</p>
    /// <p>This field can be used to show channel subtype, such as <code>connect:Guide</code>.</p>
    /// <p>Currently Contact Expiry is the only segment attribute which can be updated by using the UpdateContact API.</p>
    pub fn get_segment_attributes(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::SegmentAttributeValue>> {
        self.inner.get_segment_attributes()
    }
    /// <p>Information about the queue associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn queue_info(mut self, input: crate::types::QueueInfoInput) -> Self {
        self.inner = self.inner.queue_info(input);
        self
    }
    /// <p>Information about the queue associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn set_queue_info(mut self, input: ::std::option::Option<crate::types::QueueInfoInput>) -> Self {
        self.inner = self.inner.set_queue_info(input);
        self
    }
    /// <p>Information about the queue associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn get_queue_info(&self) -> &::std::option::Option<crate::types::QueueInfoInput> {
        self.inner.get_queue_info()
    }
    /// <p>Information about the agent associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn user_info(mut self, input: crate::types::UserInfo) -> Self {
        self.inner = self.inner.user_info(input);
        self
    }
    /// <p>Information about the agent associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn set_user_info(mut self, input: ::std::option::Option<crate::types::UserInfo>) -> Self {
        self.inner = self.inner.set_user_info(input);
        self
    }
    /// <p>Information about the agent associated with a contact. This parameter can only be updated for external audio contacts. It is used when you integrate third-party systems with Contact Lens for analytics. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i> Amazon Connect Administrator Guide</i>.</p>
    pub fn get_user_info(&self) -> &::std::option::Option<crate::types::UserInfo> {
        self.inner.get_user_info()
    }
    /// <p>The endpoint of the customer for which the contact was initiated. For external audio contacts, this is usually the end customer's phone number. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn customer_endpoint(mut self, input: crate::types::Endpoint) -> Self {
        self.inner = self.inner.customer_endpoint(input);
        self
    }
    /// <p>The endpoint of the customer for which the contact was initiated. For external audio contacts, this is usually the end customer's phone number. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn set_customer_endpoint(mut self, input: ::std::option::Option<crate::types::Endpoint>) -> Self {
        self.inner = self.inner.set_customer_endpoint(input);
        self
    }
    /// <p>The endpoint of the customer for which the contact was initiated. For external audio contacts, this is usually the end customer's phone number. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn get_customer_endpoint(&self) -> &::std::option::Option<crate::types::Endpoint> {
        self.inner.get_customer_endpoint()
    }
    /// <p>External system endpoint for the contact was initiated. For external audio contacts, this is the phone number of the external system such as the contact center. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn system_endpoint(mut self, input: crate::types::Endpoint) -> Self {
        self.inner = self.inner.system_endpoint(input);
        self
    }
    /// <p>External system endpoint for the contact was initiated. For external audio contacts, this is the phone number of the external system such as the contact center. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn set_system_endpoint(mut self, input: ::std::option::Option<crate::types::Endpoint>) -> Self {
        self.inner = self.inner.set_system_endpoint(input);
        self
    }
    /// <p>External system endpoint for the contact was initiated. For external audio contacts, this is the phone number of the external system such as the contact center. This value can only be updated for external audio contacts. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/contact-lens-integration.html">Amazon Connect Contact Lens integration</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
    pub fn get_system_endpoint(&self) -> &::std::option::Option<crate::types::Endpoint> {
        self.inner.get_system_endpoint()
    }
}
