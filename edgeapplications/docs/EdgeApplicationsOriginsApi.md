# \EdgeApplicationsOriginsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_edge_application_id_origins_get**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_get) | **GET** /edge_applications/{edge_application_id}/origins | /edge_applications/{edge_application_id}/origins
[**edge_applications_edge_application_id_origins_origin_key_delete**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_delete) | **DELETE** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_id}
[**edge_applications_edge_application_id_origins_origin_key_get**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_get) | **GET** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_key}
[**edge_applications_edge_application_id_origins_origin_key_patch**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_patch) | **PATCH** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/:edge_application_id:/origins/:origin_id:
[**edge_applications_edge_application_id_origins_origin_key_put**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_put) | **PUT** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_id}
[**edge_applications_edge_application_id_origins_post**](EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_post) | **POST** /edge_applications/{edge_application_id}/origins | /edge_applications/{edge_application_id}/origins



## edge_applications_edge_application_id_origins_get

> models::OriginsResponse edge_applications_edge_application_id_origins_get(edge_application_id, page, page_size, filter, order_by, sort, accept)
/edge_applications/{edge_application_id}/origins

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

[**models::OriginsResponse**](OriginsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_origins_origin_key_delete

> edge_applications_edge_application_id_origins_origin_key_delete(edge_application_id, origin_key, accept)
/edge_applications/{edge_application_id}/origins/{origin_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**origin_key** | **String** |  | [required] |
**accept** | Option<**String**> | The id of the Origin that you plan to delete. |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_origins_origin_key_get

> models::OriginsIdResponse edge_applications_edge_application_id_origins_origin_key_get(edge_application_id, origin_key, accept)
/edge_applications/{edge_application_id}/origins/{origin_key}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**origin_key** | **String** |  | [required] |
**accept** | Option<**String**> | The id of the Origin that you plan to query. |  |

### Return type

[**models::OriginsIdResponse**](OriginsIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_origins_origin_key_patch

> models::OriginsIdResponse edge_applications_edge_application_id_origins_origin_key_patch(edge_application_id, origin_key, accept, content_type, patch_origins_request)
/edge_applications/:edge_application_id:/origins/:origin_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**origin_key** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**patch_origins_request** | Option<[**PatchOriginsRequest**](PatchOriginsRequest.md)> |  |  |

### Return type

[**models::OriginsIdResponse**](OriginsIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_origins_origin_key_put

> models::OriginsIdResponse edge_applications_edge_application_id_origins_origin_key_put(edge_application_id, origin_key, accept, content_type, update_origins_request)
/edge_applications/{edge_application_id}/origins/{origin_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**origin_key** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**update_origins_request** | Option<[**UpdateOriginsRequest**](UpdateOriginsRequest.md)> |  |  |

### Return type

[**models::OriginsIdResponse**](OriginsIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_origins_post

> models::OriginsIdResponse edge_applications_edge_application_id_origins_post(edge_application_id, accept, content_type, create_origins_request)
/edge_applications/{edge_application_id}/origins

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**create_origins_request** | Option<[**CreateOriginsRequest**](CreateOriginsRequest.md)> |  |  |

### Return type

[**models::OriginsIdResponse**](OriginsIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

