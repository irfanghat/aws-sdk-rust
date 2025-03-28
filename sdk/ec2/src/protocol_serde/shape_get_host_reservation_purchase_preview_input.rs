// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_host_reservation_purchase_preview_input_input_input(
    input: &crate::operation::get_host_reservation_purchase_preview::GetHostReservationPurchasePreviewInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetHostReservationPurchasePreview", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("HostIdSet");
    if let Some(var_2) = &input.host_id_set {
        if !var_2.is_empty() {
            let mut list_4 = scope_1.start_list(true, Some("item"));
            for item_3 in var_2 {
                #[allow(unused_mut)]
                let mut entry_5 = list_4.entry();
                entry_5.string(item_3);
            }
            list_4.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("OfferingId");
    if let Some(var_7) = &input.offering_id {
        scope_6.string(var_7);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
