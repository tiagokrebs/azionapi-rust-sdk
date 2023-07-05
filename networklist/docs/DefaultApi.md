# \DefaultApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_lists_get**](DefaultApi.md#network_lists_get) | **GET** /network_lists | List all user Network Lists
[**network_lists_post**](DefaultApi.md#network_lists_post) | **POST** /network_lists | Create a Network Lists
[**network_lists_uuid_get**](DefaultApi.md#network_lists_uuid_get) | **GET** /network_lists/{uuid} | Retrieve a Network Lists set by uuid
[**network_lists_uuid_put**](DefaultApi.md#network_lists_uuid_put) | **PUT** /network_lists/{uuid} | Overwrite some Network Lists attributes



## network_lists_get

> crate::models::ListNetworkListsResponse network_lists_get(page)
List all user Network Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |

### Return type

[**crate::models::ListNetworkListsResponse**](ListNetworkListsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_lists_post

> network_lists_post(create_network_lists_request)
Create a Network Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_network_lists_request** | [**CreateNetworkListsRequest**](CreateNetworkListsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_lists_uuid_get

> crate::models::NetworkListsResponse network_lists_uuid_get(uuid)
Retrieve a Network Lists set by uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**crate::models::NetworkListsResponse**](NetworkListsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_lists_uuid_put

> crate::models::ListNetworkListsResponse network_lists_uuid_put(uuid, update_network_lists_request)
Overwrite some Network Lists attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**update_network_lists_request** | [**UpdateNetworkListsRequest**](UpdateNetworkListsRequest.md) |  | [required] |

### Return type

[**crate::models::ListNetworkListsResponse**](ListNetworkListsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

