# ApplicationCacheResponseDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**name** | **String** |  | 
**browser_cache_settings** | **String** |  | 
**browser_cache_settings_maximum_ttl** | **i64** |  | 
**cdn_cache_settings** | **String** |  | 
**cdn_cache_settings_maximum_ttl** | **i64** |  | 
**cache_by_query_string** | **String** |  | 
**query_string_fields** | Option<**Vec<String>**> |  | 
**enable_query_string_sort** | **bool** |  | 
**cache_by_cookies** | **String** |  | 
**cookie_names** | Option<**Vec<String>**> |  | 
**adaptive_delivery_action** | Option<**String**> |  | [optional]
**device_group** | Option<**Vec<i32>**> |  | [optional]
**enable_caching_for_post** | **bool** |  | 
**enable_caching_for_options** | Option<**bool**> |  | [optional]
**l2_caching_enabled** | **bool** |  | 
**is_slice_configuration_enabled** | Option<**bool**> |  | [optional]
**is_slice_edge_caching_enabled** | Option<**bool**> |  | [optional]
**is_slice_l2_caching_enabled** | Option<**bool**> |  | [optional]
**slice_configuration_range** | Option<**i64**> |  | [optional]
**enable_stale_cache** | Option<**bool**> |  | [optional]
**l2_region** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


