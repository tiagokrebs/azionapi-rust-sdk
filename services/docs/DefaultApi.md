# \DefaultApi

All URIs are relative to *http://bff.azion.net:3002*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_resource**](DefaultApi.md#delete_resource) | **DELETE** /edge_services/{serviceId}/resources/{resourceId} | Delete Service Resource by ID
[**delete_service**](DefaultApi.md#delete_service) | **DELETE** /edge_services/{id} | Delete Service by ID
[**get_resource**](DefaultApi.md#get_resource) | **GET** /edge_services/{serviceId}/resources/{resourceId} | Return Service Resource by ID
[**get_resources**](DefaultApi.md#get_resources) | **GET** /edge_services/{serviceId}/resources | Return Service Resources by page
[**get_service**](DefaultApi.md#get_service) | **GET** /edge_services/{id} | Return Service by ID
[**get_services**](DefaultApi.md#get_services) | **GET** /edge_services | Return Services by page
[**new_service**](DefaultApi.md#new_service) | **POST** /edge_services | Create Service
[**patch_service**](DefaultApi.md#patch_service) | **PATCH** /edge_services/{id} | Update Service by ID
[**patch_service_resource**](DefaultApi.md#patch_service_resource) | **PATCH** /edge_services/{serviceId}/resources/{resourceId} | Update Service Resource by ID
[**post_resource**](DefaultApi.md#post_resource) | **POST** /edge_services/{serviceId}/resources | Create Service Resource



## delete_resource

> delete_resource(service_id, resource_id)
Delete Service Resource by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **i64** |  | [required] |
**resource_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service

> delete_service(id)
Delete Service by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resource

> crate::models::ResourceDetail get_resource(service_id, resource_id)
Return Service Resource by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **i64** |  | [required] |
**resource_id** | **i64** |  | [required] |

### Return type

[**crate::models::ResourceDetail**](ResourceDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resources

> serde_json::Value get_resources(service_id, page, page_size, filter, order_by, sort)
Return Service Resources by page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **i64** |  | [required] |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service

> crate::models::ServiceResponse get_service(id, with_vars)
Return Service by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**with_vars** | Option<**bool**> |  |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> crate::models::ServiceResponseWithTotals get_services(page, page_size, filter, order_by, sort)
Return Services by page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ServiceResponseWithTotals**](ServiceResponseWithTotals.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_service

> crate::models::ServiceResponse new_service(create_service_request)
Create Service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_request** | [**CreateServiceRequest**](CreateServiceRequest.md) |  | [required] |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_service

> crate::models::ServiceResponse patch_service(id, update_service_request)
Update Service by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**update_service_request** | [**UpdateServiceRequest**](UpdateServiceRequest.md) |  | [required] |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_service_resource

> crate::models::ResourceDetail patch_service_resource(service_id, resource_id, update_resource_request)
Update Service Resource by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **i64** |  | [required] |
**resource_id** | **i64** |  | [required] |
**update_resource_request** | [**UpdateResourceRequest**](UpdateResourceRequest.md) |  | [required] |

### Return type

[**crate::models::ResourceDetail**](ResourceDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_resource

> crate::models::ResourceDetail post_resource(service_id, create_resource_request)
Create Service Resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **i64** |  | [required] |
**create_resource_request** | [**CreateResourceRequest**](CreateResourceRequest.md) |  | [required] |

### Return type

[**crate::models::ResourceDetail**](ResourceDetail.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

