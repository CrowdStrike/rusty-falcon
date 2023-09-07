# DomainPeriodNewsDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | legacy field, not used | [optional]
**actors** | [**Vec<crate::models::DomainPeriodSimpleActor>**](domain.SimpleActor.md) | Actors mentioned, related or referenced in the news/report |
**attachments** | Option<[**Vec<crate::models::DomainPeriodFile>**](domain.File.md)> | News attachment, containing either pdf url or feeds zip and/or gzip archive | [optional]
**created_date** | **i64** | Date of the news document creation, unix timestampt |
**description** | Option<**String**> | Full report description, extracted from the document | [optional]
**entitlements** | Option<[**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md)> | internal property used for permissions check of access, not returned or explicitly filterable | [optional]
**id** | **i64** | Integer ID of the News document |
**image** | Option<[**crate::models::DomainPeriodImage**](domain.Image.md)> |  | [optional]
**last_modified_date** | **i64** | Date of the news document last modification, unix timestampt |
**motivations** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | News mentioned motivation or motivation of related actors and malware families |
**name** | **String** | News title |
**notify_users** | Option<**bool**> | internal field, not used | [optional]
**rich_text_description** | Option<**String**> | Rich text description with markup | [optional]
**short_description** | Option<**String**> | Short description of the report content | [optional]
**slug** | **String** | News title in a url friendly way, which is title in lowercase and special characters including space replaced with dash |
**sub_type** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**tags** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | News tags, which contains MITRE, Vulnerability community identifiers, capabilities, malware family name, customer target, activity cluster, notable event, geopolitical issue |
**target_countries** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | News mentioned target countries or related actor's target countries |
**target_industries** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | News mentioned target industries or related actor's target industries |
**thumbnail** | [**crate::models::DomainPeriodImage**](domain.Image.md) |  |
**topic** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**r#type** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**url** | Option<**String**> | URL of the news document where it can be accessed in the Falcon Portal | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
