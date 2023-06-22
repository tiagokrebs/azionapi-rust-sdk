# \RealTimePurgeApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purge_cache_key**](RealTimePurgeApi.md#purge_cache_key) | **POST** /purge/cachekey | /purge/cachekey
[**purge_url**](RealTimePurgeApi.md#purge_url) | **POST** /purge/url | /purge/url
[**purge_wildcard**](RealTimePurgeApi.md#purge_wildcard) | **POST** /purge/wildcard | /purge/wildcard



## purge_cache_key

> purge_cache_key(accept, content_type, purge_cache_key_request)
/purge/cachekey

List of Cache Keys you want to remove from the Azion Edge Servers cache. urls (array): list of up to 50 Cache Keys to be expired from the cache, per request. method (choice): purge method, use the “delete” value for removal. Layer (choice): layer where the purge will be done. Use the value “edge_caching” (default value if not informed) to purge on the Edge Caching layer and the value “l2_caching” to purge on L2 Caching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**purge_cache_key_request** | Option<[**PurgeCacheKeyRequest**](PurgeCacheKeyRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_url

> purge_url(accept, content_type, purge_url_request)
/purge/url

List of URLs you want to remove from the Azion Edge Servers cache. urls (array): list of up to 50 URLs to be expired from the cache, per request. method (choice): purge method, use the “delete” value for removal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**purge_url_request** | Option<[**PurgeUrlRequest**](PurgeUrlRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_wildcard

> purge_wildcard(accept, content_type, purge_wildcard_request)
/purge/wildcard

The Wildcard expression that represents the list of objects that you want to remove from the Azion Edge Servers cache. urls (array):the Wildcard URL or Wildcard Cache Key that represents the list of objects you want to expire. You can only use one Wildcard expression per request. method (choice): purge method, use the “delete” value for removal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**purge_wildcard_request** | Option<[**PurgeWildcardRequest**](PurgeWildcardRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

