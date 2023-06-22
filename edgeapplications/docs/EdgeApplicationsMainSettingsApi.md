# \EdgeApplicationsMainSettingsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_get**](EdgeApplicationsMainSettingsApi.md#edge_applications_get) | **GET** /edge_applications | /edge_applications
[**edge_applications_id_delete**](EdgeApplicationsMainSettingsApi.md#edge_applications_id_delete) | **DELETE** /edge_applications/{id} | /edge_applications/:id
[**edge_applications_id_get**](EdgeApplicationsMainSettingsApi.md#edge_applications_id_get) | **GET** /edge_applications/{id} | /edge_applications/:id
[**edge_applications_id_patch**](EdgeApplicationsMainSettingsApi.md#edge_applications_id_patch) | **PATCH** /edge_applications/{id} | /edge_applications/:id
[**edge_applications_id_put**](EdgeApplicationsMainSettingsApi.md#edge_applications_id_put) | **PUT** /edge_applications/{id} | /edge_applications/:id
[**edge_applications_post**](EdgeApplicationsMainSettingsApi.md#edge_applications_post) | **POST** /edge_applications | /edge_applications



## edge_applications_get

> serde_json::Value edge_applications_get(page, page_size, filter, order_by, sort, accept)
/edge_applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**accept** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_id_delete

> edge_applications_id_delete(id, accept)
/edge_applications/:id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the edge application that you plan to delete. | [required] |
**accept** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_id_get

> crate::models::GetApplicationResponse edge_applications_id_get(id, accept)
/edge_applications/:id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

[**crate::models::GetApplicationResponse**](GetApplicationResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_id_patch

> crate::models::ApplicationUpdateResponse edge_applications_id_patch(id, accept, content_type, application_update_request)
/edge_applications/:id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_update_request** | Option<[**ApplicationUpdateRequest**](ApplicationUpdateRequest.md)> |  |  |

### Return type

[**crate::models::ApplicationUpdateResponse**](ApplicationUpdateResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_id_put

> crate::models::ApplicationPutResult edge_applications_id_put(id, accept, content_type, application_put_request)
/edge_applications/:id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The Id of the edge application to be overwritten.  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_put_request** | Option<[**ApplicationPutRequest**](ApplicationPutRequest.md)> |  |  |

### Return type

[**crate::models::ApplicationPutResult**](ApplicationPutResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_post

> crate::models::CreateApplicationResult edge_applications_post(accept, content_type, create_application_request)
/edge_applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**create_application_request** | Option<[**CreateApplicationRequest**](CreateApplicationRequest.md)> |  |  |

### Return type

[**crate::models::CreateApplicationResult**](CreateApplicationResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

