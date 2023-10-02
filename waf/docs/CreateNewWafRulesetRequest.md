# CreateNewWafRulesetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**name** | **String** | Identification name for WAF Rule Set. | 
**mode** | **String** |  | 
**active** | **bool** |  | 
**sql_injection** | **bool** |  | 
**sql_injection_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**remote_file_inclusion** | **bool** |  | 
**remote_file_inclusion_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**directory_traversal** | **bool** |  | 
**directory_traversal_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**cross_site_scripting** | **bool** |  | 
**cross_site_scripting_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**evading_tricks** | **bool** |  | 
**evading_tricks_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**file_upload** | **bool** |  | 
**file_upload_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**unwanted_access** | **bool** |  | 
**unwanted_access_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**identified_attack** | **bool** |  | 
**identified_attack_sensitivity** | [**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md) |  | 
**bypass_addresses** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


