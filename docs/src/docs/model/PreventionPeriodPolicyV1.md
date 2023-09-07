# PreventionPeriodPolicyV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **String** | The customer id associated with the policy |
**created_by** | **String** | The email of the user which created the policy |
**created_timestamp** | **String** | The time at which the policy was created |
**description** | **String** | The description of a policy. Use this field to provide a high level summary of what this policy enforces |
**enabled** | **bool** | If a policy is enabled it will be used during the course of policy evaluation |
**groups** | [**Vec<crate::models::HostGroupsPeriodHostGroupV1>**](host_groups.HostGroupV1.md) | The groups that are currently attached to the policy |
**id** | **String** | The unique id of the policy |
**ioa_rule_groups** | [**Vec<crate::models::IoaRuleGroupsPeriodRuleGroupV1>**](ioa_rule_groups.RuleGroupV1.md) | The IOA rule groups that are currently attached to the policy |
**modified_by** | **String** | The email of the user which last modified the policy |
**modified_timestamp** | **String** | The time at which the policy was last modified |
**name** | **String** | The human readable name of the policy |
**platform_name** | **String** | The name of the platform |
**prevention_settings** | [**Vec<crate::models::PreventionPeriodCategoryRespV1>**](prevention.CategoryRespV1.md) | A category of settings in a prevention policy |

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
