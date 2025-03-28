// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_addresses_input_input_input(
    input: &crate::operation::describe_addresses::DescribeAddressesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeAddresses", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PublicIp");
    if let Some(var_2) = &input.public_ips {
        if !var_2.is_empty() {
            let mut list_4 = scope_1.start_list(true, Some("PublicIp"));
            for item_3 in var_2 {
                #[allow(unused_mut)]
                let mut entry_5 = list_4.entry();
                entry_5.string(item_3);
            }
            list_4.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DryRun");
    if let Some(var_7) = &input.dry_run {
        scope_6.boolean(*var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Filter");
    if let Some(var_9) = &input.filters {
        if !var_9.is_empty() {
            let mut list_11 = scope_8.start_list(true, Some("Filter"));
            for item_10 in var_9 {
                #[allow(unused_mut)]
                let mut entry_12 = list_11.entry();
                crate::protocol_serde::shape_filter::ser_filter(entry_12, item_10)?;
            }
            list_11.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("AllocationId");
    if let Some(var_14) = &input.allocation_ids {
        if !var_14.is_empty() {
            let mut list_16 = scope_13.start_list(true, Some("AllocationId"));
            for item_15 in var_14 {
                #[allow(unused_mut)]
                let mut entry_17 = list_16.entry();
                entry_17.string(item_15);
            }
            list_16.finish();
        }
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
