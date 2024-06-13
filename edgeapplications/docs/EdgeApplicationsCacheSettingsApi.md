# \EdgeApplicationsCacheSettingsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_edge_application_id_cache_settings_cache_settings_id_delete**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_delete) | **DELETE** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:
[**edge_applications_edge_application_id_cache_settings_cache_settings_id_get**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_get) | **GET** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:
[**edge_applications_edge_application_id_cache_settings_cache_settings_id_patch**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_patch) | **PATCH** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:
[**edge_applications_edge_application_id_cache_settings_cache_settings_id_put**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_put) | **PUT** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/ca
[**edge_applications_edge_application_id_cache_settings_get**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_get) | **GET** /edge_applications/{edge_application_id}/cache_settings | /edge_applications/{edge_application_id}/cache_settings
[**edge_applications_edge_application_id_cache_settings_post**](EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_post) | **POST** /edge_applications/{edge_application_id}/cache_settings | /edge_applications/:edge_application_id:/cache_settings



## edge_applications_edge_application_id_cache_settings_cache_settings_id_delete

> edge_applications_edge_application_id_cache_settings_cache_settings_id_delete(edge_application_id, cache_settings_id, accept, content_type)
/edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**cache_settings_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_cache_settings_cache_settings_id_get

> models::ApplicationCacheGetOneResponse edge_applications_edge_application_id_cache_settings_cache_settings_id_get(edge_application_id, cache_settings_id, accept)
/edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**cache_settings_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

[**models::ApplicationCacheGetOneResponse**](ApplicationCacheGetOneResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_cache_settings_cache_settings_id_patch

> models::ApplicationCachePatchResponse edge_applications_edge_application_id_cache_settings_cache_settings_id_patch(edge_application_id, cache_settings_id, accept, application_cache_patch_request)
/edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**cache_settings_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**application_cache_patch_request** | Option<[**ApplicationCachePatchRequest**](ApplicationCachePatchRequest.md)> |  |  |

### Return type

[**models::ApplicationCachePatchResponse**](ApplicationCachePatchResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_cache_settings_cache_settings_id_put

> models::ApplicationCachePutResponse edge_applications_edge_application_id_cache_settings_cache_settings_id_put(edge_application_id, cache_settings_id, accept, content_type, application_cache_put_request)
/edge_applications/:edge_application_id:/cache_settings/ca

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**cache_settings_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_cache_put_request** | Option<[**ApplicationCachePutRequest**](ApplicationCachePutRequest.md)> |  |  |

### Return type

[**models::ApplicationCachePutResponse**](ApplicationCachePutResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_cache_settings_get

> models::ApplicationCacheGetResponse edge_applications_edge_application_id_cache_settings_get(edge_application_id, page, page_size, filter, order_by, sort, accept)
/edge_applications/{edge_application_id}/cache_settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**accept** | Option<**String**> |  |  |

### Return type

[**models::ApplicationCacheGetResponse**](ApplicationCacheGetResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_cache_settings_post

> models::ApplicationCacheCreateResponse edge_applications_edge_application_id_cache_settings_post(edge_application_id, accept, content_type, application_cache_create_request)
/edge_applications/:edge_application_id:/cache_settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_cache_create_request** | Option<[**ApplicationCacheCreateRequest**](ApplicationCacheCreateRequest.md)> |  |  |

### Return type

[**models::ApplicationCacheCreateResponse**](ApplicationCacheCreateResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

