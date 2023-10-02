# SingleWaf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> |  | [optional]
**name** | Option<**String**> | Identification name for WAF Rule Set. | [optional]
**mode** | Option<**String**> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**sql_injection** | Option<**bool**> |  | [optional]
**sql_injection_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**remote_file_inclusion** | Option<**bool**> |  | [optional]
**remote_file_inclusion_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**directory_traversal** | Option<**bool**> |  | [optional]
**directory_traversal_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**cross_site_scripting** | Option<**bool**> |  | [optional]
**cross_site_scripting_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**evading_tricks** | Option<**bool**> |  | [optional]
**evading_tricks_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**file_upload** | Option<**bool**> |  | [optional]
**file_upload_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**unwanted_access** | Option<**bool**> |  | [optional]
**unwanted_access_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**identified_attack** | Option<**bool**> |  | [optional]
**identified_attack_sensitivity** | Option<[**crate::models::WafSensitivityChoices**](WAFSensitivityChoices.md)> |  | [optional]
**bypass_addresses** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


