# \WafApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_waf_domains**](WafApi.md#get_waf_domains) | **GET** /waf/{wafId}/domains | Find domains attached to a WAF
[**get_waf_events**](WafApi.md#get_waf_events) | **GET** /waf/{wafId}/waf_events | Find WAF log events



## get_waf_domains

> crate::models::WafDomains200 get_waf_domains(waf_id, name)
Find domains attached to a WAF

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_id** | **i64** | ID of WAF to return | [required] |
**name** | Option<**String**> | searches WAF for name |  |

### Return type

[**crate::models::WafDomains200**](WAFDomains200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_waf_events

> crate::models::WafEvents200 get_waf_events(waf_id, hour_range, domains_ids, network_list_id)
Find WAF log events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_id** | **i64** | ID of WAF to return | [required] |
**hour_range** | **i64** | Last log hours since now (it must be a integer number ranging between 1 and 72) | [required] |
**domains_ids** | **String** | Multiple domain's id (they must be separated by comma like 1233,1234) | [required] |
**network_list_id** | Option<**i64**> | Id of a network list |  |

### Return type

[**crate::models::WafEvents200**](WAFEvents200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

