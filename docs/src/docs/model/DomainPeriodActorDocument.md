# DomainPeriodActorDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **bool** | Boolean field marking if actor is active |
**actor_type** | Option<**String**> | Actor type, one of: targeted, ecrime | [optional]
**capabilities** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | actor's capabilities, some examples: RAT,Ransomware,Spearphishing,Downloader,Backdoor,InformationStealer,exploit,CredentialHarvesting,dropper,DenialOfService,Loader,Phishing |
**capability** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**created_date** | **i64** | Actor's document creation date when it was added to the Falcon portal in unix timestamp format |
**description** | Option<**String**> | Actor's text description, partially containing structured data from other fields | [optional]
**ecrime_kill_chain** | Option<[**crate::models::DomainPeriodECrimeKillChain**](domain.ECrimeKillChain.md)> |  | [optional]
**entitlements** | Option<[**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md)> | Field used to filter user's access to actor documents | [optional]
**first_activity_date** | **i64** | Actor's first activity observed date in unix timestamp format |
**group** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**id** | **i64** | Numerical ID for the Actor |
**image** | Option<[**crate::models::DomainPeriodImage**](domain.Image.md)> |  | [optional]
**kill_chain** | Option<[**crate::models::DomainPeriodKillChain**](domain.KillChain.md)> |  | [optional]
**known_as** | **String** | Alternative names and community identifiers of an actor |
**last_activity_date** | **i64** | Actor's last (most recent) activity observed date in unix timestamp format |
**last_modified_date** | **i64** | Actor's document last modified date in unix timestamp format |
**motivations** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | Actor's activity motivation, one of: State-Sponsored, Criminal, Hacktivism |
**name** | Option<**String**> | Actor's name, composed of 2 words | [optional]
**notify_users** | **bool** | internal field |
**objectives** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | Actor's activity objectives, one of: IntelligenceGathering, FinancialGain, IntellectualPropertyTheft, defacement, Destruction, DenialOfService |
**origins** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | represents origin of actor's activity and/or members, some examples: China,Russian Federation,Eastern Europe,Iran,East Asia, South Asia |
**recent_alerting** | Option<**i64**> | Recent CrowdStrike's finished intelligence alerting date in unix timestamp format | [optional]
**region** | Option<[**crate::models::DomainPeriodEntity**](domain.Entity.md)> |  | [optional]
**rich_text_description** | Option<**String**> | Rich text version of the description field | [optional]
**short_description** | **String** | Short version of the description field |
**slug** | Option<**String**> | Name in url friendly format, lowercased and spaces replaced with dash | [optional]
**status** | **String** | Status of an actor, one of: Active, Inactive, Retired |
**target_countries** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | Target countries of actor's activity and attacks, slug value is a 2 characters code for the country value, some examples: United States,United Kingdom,Germany,India,Japan,France,Australia,Canada,China |
**target_industries** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | Target economical industries of actor's activity and attacks. List of available values: Government, Financial Services, Technology, Telecommunications, Healthcare, Energy, Academic, Media, Aerospace, NGO, Manufacturing, Industrials and Engineering, Retail, Hospitality, Consulting and Professional Services, Opportunistic, Aviation, Defense, Transportation, Oil and Gas, Legal, Pharmaceutical, Logistics, Military, Automotive, Food and Beverage, Consumer Goods, Real Estate, Insurance, Agriculture, Chemicals, Utilities, Maritime, Extractive, Travel, Dissident, Cryptocurrency, Entertainment, National Government, Law Enforcement, Think Tanks, Local Government, Sports Organizations, Computer Gaming, Biomedical, Nonprofit, Financial Management & Hedge Funds, Political Parties, Architectural and Engineering, Emergency Services, Social Media, International Government, Nuclear, Research Entities, Vocational and Higher-Level Education, eCommerce |
**target_regions** | [**Vec<crate::models::DomainPeriodEntity>**](domain.Entity.md) | Target geographic regions of actor's activity and attacks. List of available values: North America, Western Europe, Southeast Asia, Middle East, Eastern Europe, South Asia, South America, Oceania, East Asia, Central Africa, Northern Europe, Southern Europe, North Africa, Southern Africa, Central America, Central Asia, East Africa, West Africa, Caribbean |
**thumbnail** | Option<[**crate::models::DomainPeriodImage**](domain.Image.md)> |  | [optional]
**url** | Option<**String**> | URL at which actor profile can be accessed | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
