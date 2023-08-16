# DomainPeriodDdosAttackSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attack_type** | **String** | The type of attack. One of `Amplification`, `Botnet`, `Other` | 
**confidence** | **String** | The confidence level. One of `Low`, `Medium`, `High` | 
**duration** | **i64** | The duration of the attack in seconds | 
**key** | **String** | The hash over target and date | 
**network_protocol** | **String** | The network protocol used. One of `TCP`, `UDP`, `ICMP`, `Other` | 
**protocols** | **Vec<String>** | The protocols used in the attack | 
**requests** | **i64** | The number of requests against the target | 
**start_time** | **String** | The ISO-8601 date for the attack start time | 
**target** | **String** | The target's IPv4/6 address or hostname | 
**target_details** | [**crate::models::DomainPeriodDdosTargetDetails**](domain.DDOSTargetDetails.md) |  | 
**target_domain** | **String** | The target's domain | 
**target_ip** | **String** | The target's IPv4/6 address | 
**target_ports** | **Vec<i32>** | List of ports where the attack took place | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


