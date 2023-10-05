# \ZonesApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_zone**](ZonesApi.md#delete_zone) | **DELETE** /intelligent_dns/{zone_id} | Remove an Intelligent DNS hosted zone
[**get_zone**](ZonesApi.md#get_zone) | **GET** /intelligent_dns/{zone_id} | Get an Intelligent DNS hosted zone
[**get_zones**](ZonesApi.md#get_zones) | **GET** /intelligent_dns | Get a collection of Intelligent DNS zones
[**post_zone**](ZonesApi.md#post_zone) | **POST** /intelligent_dns | Add a new Intelligent DNS zone
[**put_zone**](ZonesApi.md#put_zone) | **PUT** /intelligent_dns/{zone_id} | Update an Intelligent DNS hosted zone



## delete_zone

> String delete_zone(zone_id)
Remove an Intelligent DNS hosted zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |

### Return type

**String**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zone

> crate::models::GetZoneResponse get_zone(zone_id)
Get an Intelligent DNS hosted zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |

### Return type

[**crate::models::GetZoneResponse**](GetZoneResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zones

> crate::models::GetZonesResponse get_zones(order_by, sort, page, page_size)
Get a collection of Intelligent DNS zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Identifies which property the return should be sorted by. |  |[default to name]
**sort** | Option<**String**> | Defines whether objects are shown in ascending or descending order depending on the value set in order_by. |  |[default to asc]
**page** | Option<**i64**> | Identifies which page should be returned, if the return is paginated. |  |[default to 1]
**page_size** | Option<**i64**> | Identifies how many items should be returned per page. |  |[default to 10]

### Return type

[**crate::models::GetZonesResponse**](GetZonesResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_zone

> crate::models::PostOrPutZoneResponse post_zone(zone)
Add a new Intelligent DNS zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | Option<[**Zone**](Zone.md)> |  |  |

### Return type

[**crate::models::PostOrPutZoneResponse**](PostOrPutZoneResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_zone

> crate::models::PostOrPutZoneResponse put_zone(zone_id, zone)
Update an Intelligent DNS hosted zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |
**zone** | Option<[**Zone**](Zone.md)> |  |  |

### Return type

[**crate::models::PostOrPutZoneResponse**](PostOrPutZoneResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

