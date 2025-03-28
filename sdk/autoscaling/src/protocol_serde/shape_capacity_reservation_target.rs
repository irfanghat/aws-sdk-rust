// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_capacity_reservation_target(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::CapacityReservationTarget,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CapacityReservationIds");
    if let Some(var_2) = &input.capacity_reservation_ids {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("CapacityReservationResourceGroupArns");
    if let Some(var_7) = &input.capacity_reservation_resource_group_arns {
        let mut list_9 = scope_6.start_list(false, None);
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_capacity_reservation_target(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::CapacityReservationTarget, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CapacityReservationTarget::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CapacityReservationIds") /* CapacityReservationIds com.amazonaws.autoscaling#CapacityReservationTarget$CapacityReservationIds */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation_ids::de_capacity_reservation_ids(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_ids(var_11);
            }
            ,
            s if s.matches("CapacityReservationResourceGroupArns") /* CapacityReservationResourceGroupArns com.amazonaws.autoscaling#CapacityReservationTarget$CapacityReservationResourceGroupArns */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation_resource_group_arns::de_capacity_reservation_resource_group_arns(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_resource_group_arns(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
