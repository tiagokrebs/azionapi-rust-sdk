# \EdgeApplicationsDeviceGroupsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_edge_application_id_device_groups_device_group_id_delete**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_delete) | **DELETE** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
[**edge_applications_edge_application_id_device_groups_device_group_id_get**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_get) | **GET** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
[**edge_applications_edge_application_id_device_groups_device_group_id_patch**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_patch) | **PATCH** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
[**edge_applications_edge_application_id_device_groups_device_group_id_put**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_put) | **PUT** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
[**edge_applications_edge_application_id_device_groups_get**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_get) | **GET** /edge_applications/{edge_application_id}/device_groups | /edge_applications/{edge_application_id}/device_groups
[**edge_applications_edge_application_id_device_groups_post**](EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_post) | **POST** /edge_applications/{edge_application_id}/device_groups | /edge_applications/{edge_application_id}/device_groups



## edge_applications_edge_application_id_device_groups_device_group_id_delete

> edge_applications_edge_application_id_device_groups_device_group_id_delete(edge_application_id, device_group_id, accept)
/edge_applications/{edge_application_id}/device_groups/{device_group_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**device_group_id** | **i64** |  | [required] |
**accept** | Option<**String**> | The id of the Device Groups that you plan to delete. |  |

### Return type

 (empty response body)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_device_groups_device_group_id_get

> crate::models::DeviceGroupsIdResponse edge_applications_edge_application_id_device_groups_device_group_id_get(edge_application_id, device_group_id, accept)
/edge_applications/{edge_application_id}/device_groups/{device_group_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**device_group_id** | **i64** |  | [required] |
**accept** | Option<**String**> | The id of the Device Groups that you plan to query. |  |

### Return type

[**crate::models::DeviceGroupsIdResponse**](DeviceGroupsIdResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_device_groups_device_group_id_patch

> crate::models::DeviceGroupsIdResponse edge_applications_edge_application_id_device_groups_device_group_id_patch(edge_application_id, device_group_id, accept, content_type, patch_device_groups_request)
/edge_applications/{edge_application_id}/device_groups/{device_group_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**device_group_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**patch_device_groups_request** | Option<[**PatchDeviceGroupsRequest**](PatchDeviceGroupsRequest.md)> |  |  |

### Return type

[**crate::models::DeviceGroupsIdResponse**](DeviceGroupsIdResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_device_groups_device_group_id_put

> crate::models::DeviceGroupsIdResponse edge_applications_edge_application_id_device_groups_device_group_id_put(edge_application_id, device_group_id, accept, content_type, update_device_groups_request)
/edge_applications/{edge_application_id}/device_groups/{device_group_id}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**device_group_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**update_device_groups_request** | Option<[**UpdateDeviceGroupsRequest**](UpdateDeviceGroupsRequest.md)> |  |  |

### Return type

[**crate::models::DeviceGroupsIdResponse**](DeviceGroupsIdResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_device_groups_get

> crate::models::DeviceGroupsResponse edge_applications_edge_application_id_device_groups_get(edge_application_id, page, page_size, filter, order_by, sort, accept)
/edge_applications/{edge_application_id}/device_groups

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

[**crate::models::DeviceGroupsResponse**](DeviceGroupsResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_device_groups_post

> crate::models::DeviceGroupsIdResponse edge_applications_edge_application_id_device_groups_post(edge_application_id, accept, content_type, create_device_groups_request)
/edge_applications/{edge_application_id}/device_groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**create_device_groups_request** | Option<[**CreateDeviceGroupsRequest**](CreateDeviceGroupsRequest.md)> |  |  |

### Return type

[**crate::models::DeviceGroupsIdResponse**](DeviceGroupsIdResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

