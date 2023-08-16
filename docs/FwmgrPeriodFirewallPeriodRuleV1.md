# FwmgrPeriodFirewallPeriodRuleV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**address_family** | **String** |  | 
**created_by** | **String** |  | 
**created_on** | **String** |  | 
**customer_id** | Option<**String**> |  | [optional]
**deleted** | **bool** |  | 
**description** | **String** |  | 
**direction** | **String** |  | 
**enabled** | **bool** |  | 
**family** | **String** |  | 
**fields** | [**Vec<crate::models::FwmgrPeriodFirewallPeriodFieldValue>**](fwmgr.firewall.FieldValue.md) |  | 
**fqdn** | **String** |  | 
**fqdn_enabled** | **bool** |  | 
**icmp** | [**crate::models::FwmgrPeriodFirewallPeriodIcmp**](fwmgr.firewall.ICMP.md) |  | 
**id** | **String** |  | 
**local_address** | [**Vec<crate::models::FwmgrPeriodFirewallPeriodAddressRange>**](fwmgr.firewall.AddressRange.md) |  | 
**local_port** | [**Vec<crate::models::FwmgrPeriodFirewallPeriodPortRange>**](fwmgr.firewall.PortRange.md) |  | 
**modified_by** | Option<**String**> |  | [optional]
**modified_on** | Option<**String**> |  | [optional]
**monitor** | [**crate::models::FwmgrPeriodFirewallPeriodMonitoring**](fwmgr.firewall.Monitoring.md) |  | 
**name** | **String** |  | 
**platform_ids** | **Vec<String>** |  | 
**protocol** | **String** |  | 
**remote_address** | [**Vec<crate::models::FwmgrPeriodFirewallPeriodAddressRange>**](fwmgr.firewall.AddressRange.md) |  | 
**remote_port** | [**Vec<crate::models::FwmgrPeriodFirewallPeriodPortRange>**](fwmgr.firewall.PortRange.md) |  | 
**rule_group** | [**crate::models::FwmgrPeriodFirewallPeriodRuleGroupSummaryV1**](fwmgr.firewall.RuleGroupSummaryV1.md) |  | 
**version** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


