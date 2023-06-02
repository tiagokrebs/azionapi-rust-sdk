# \DnssecApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_zone_dns_sec**](DnssecApi.md#get_zone_dns_sec) | **GET** /intelligent_dns/{zone_id}/dnssec | Retrieve the DNSSEC zone status
[**put_zone_dns_sec**](DnssecApi.md#put_zone_dns_sec) | **PATCH** /intelligent_dns/{zone_id}/dnssec | Update the DNSSEC zone



## get_zone_dns_sec

> crate::models::GetOrPatchDnsSecResponse get_zone_dns_sec(zone_id)
Retrieve the DNSSEC zone status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |

### Return type

[**crate::models::GetOrPatchDnsSecResponse**](GetOrPatchDnsSecResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_zone_dns_sec

> crate::models::GetOrPatchDnsSecResponse put_zone_dns_sec(zone_id, dns_sec)
Update the DNSSEC zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |
**dns_sec** | Option<[**DnsSec**](DnsSec.md)> |  |  |

### Return type

[**crate::models::GetOrPatchDnsSecResponse**](GetOrPatchDnsSecResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

