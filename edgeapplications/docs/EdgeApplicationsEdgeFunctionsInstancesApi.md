# \EdgeApplicationsEdgeFunctionsInstancesApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_edge_application_id_functions_instances_functions_instances_id_delete**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_delete) | **DELETE** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
[**edge_applications_edge_application_id_functions_instances_functions_instances_id_get**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_get) | **GET** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
[**edge_applications_edge_application_id_functions_instances_functions_instances_id_patch**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_patch) | **PATCH** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
[**edge_applications_edge_application_id_functions_instances_functions_instances_id_put**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_put) | **PUT** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
[**edge_applications_edge_application_id_functions_instances_get**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_get) | **GET** /edge_applications/{edge_application_id}/functions_instances | /edge_applications/:edge_application_id:/functions_instances
[**edge_applications_edge_application_id_functions_instances_post**](EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_post) | **POST** /edge_applications/{edge_application_id}/functions_instances | edge_application/:edge_application_id:/functions_instances



## edge_applications_edge_application_id_functions_instances_functions_instances_id_delete

> edge_applications_edge_application_id_functions_instances_functions_instances_id_delete(edge_application_id, functions_instances_id, accept, content_type)
/edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **String** |  | [required] |
**functions_instances_id** | **String** |  | [required] |
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


## edge_applications_edge_application_id_functions_instances_functions_instances_id_get

> crate::models::ApplicationInstancesGetOneResponse edge_applications_edge_application_id_functions_instances_functions_instances_id_get(edge_application_id, functions_instances_id, accept)
/edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**functions_instances_id** | **i64** |  | [required] |
**accept** | Option<**String**> | The id of the edge function instance you plan to query.  |  |

### Return type

[**crate::models::ApplicationInstancesGetOneResponse**](ApplicationInstancesGetOneResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_functions_instances_functions_instances_id_patch

> crate::models::ApplicationInstanceResults edge_applications_edge_application_id_functions_instances_functions_instances_id_patch(edge_application_id, functions_instances_id, accept, content_type, application_update_instance_request)
/edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **String** | The id of the edge application you plan to overwrite  | [required] |
**functions_instances_id** | **String** | The id of the edge function instance you plan to overwrite. | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_update_instance_request** | Option<[**ApplicationUpdateInstanceRequest**](ApplicationUpdateInstanceRequest.md)> |  |  |

### Return type

[**crate::models::ApplicationInstanceResults**](ApplicationInstanceResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_functions_instances_functions_instances_id_put

> crate::models::ApplicationInstanceResults edge_applications_edge_application_id_functions_instances_functions_instances_id_put(edge_application_id, functions_instances_id, accept, content_type, application_put_instance_request)
/edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **String** | The id of the edge application you plan to overwrite  | [required] |
**functions_instances_id** | **String** | The id of the edge function instance you plan to overwrite. | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_put_instance_request** | Option<[**ApplicationPutInstanceRequest**](ApplicationPutInstanceRequest.md)> |  |  |

### Return type

[**crate::models::ApplicationInstanceResults**](ApplicationInstanceResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_functions_instances_get

> crate::models::ApplicationInstancesGetResponse edge_applications_edge_application_id_functions_instances_get(edge_application_id, page, page_size, filter, order_by, sort, accept)
/edge_applications/:edge_application_id:/functions_instances

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

[**crate::models::ApplicationInstancesGetResponse**](ApplicationInstancesGetResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_functions_instances_post

> crate::models::ApplicationInstanceResults edge_applications_edge_application_id_functions_instances_post(edge_application_id, accept, content_type, application_create_instance_request)
edge_application/:edge_application_id:/functions_instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**application_create_instance_request** | Option<[**ApplicationCreateInstanceRequest**](ApplicationCreateInstanceRequest.md)> |  |  |

### Return type

[**crate::models::ApplicationInstanceResults**](ApplicationInstanceResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

