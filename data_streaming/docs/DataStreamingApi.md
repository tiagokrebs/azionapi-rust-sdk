# \DataStreamingApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_data_streaming**](DataStreamingApi.md#create_new_data_streaming) | **POST** /data_streaming/streamings | Create a new data streaming
[**delete_data_streaming_by_id**](DataStreamingApi.md#delete_data_streaming_by_id) | **DELETE** /data_streaming/streamings/{data_streaming_id} | Delete data streaming
[**edit_data_streaming_by_id**](DataStreamingApi.md#edit_data_streaming_by_id) | **PATCH** /data_streaming/streamings/{data_streaming_id} | Edit data streaming
[**list_data_streaming_by_id**](DataStreamingApi.md#list_data_streaming_by_id) | **GET** /data_streaming/streamings/{data_streaming_id} | Get expecific data streaming by Data Streaming ID
[**list_data_streamings**](DataStreamingApi.md#list_data_streamings) | **GET** /data_streaming/streamings | List all exist data streamings
[**overwrite_data_streaming_by_id**](DataStreamingApi.md#overwrite_data_streaming_by_id) | **PUT** /data_streaming/streamings/{data_streaming_id} | Overwrite data streaming



## create_new_data_streaming

> crate::models::CreateNewDataStreaming201Response create_new_data_streaming(create_new_data_streaming_request)
Create a new data streaming



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_new_data_streaming_request** | [**CreateNewDataStreamingRequest**](CreateNewDataStreamingRequest.md) |  | [required] |

### Return type

[**crate::models::CreateNewDataStreaming201Response**](CreateNewDataStreaming_201_response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_streaming_by_id

> delete_data_streaming_by_id(data_streaming_id)
Delete data streaming

Use the DELETE method to remove a data streaming from your account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_streaming_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_data_streaming_by_id

> crate::models::CreateNewDataStreaming201Response edit_data_streaming_by_id(data_streaming_id, create_new_data_streaming_request)
Edit data streaming

Use the PATCH method to change only select settings of your data streaming. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_streaming_id** | **i32** |  | [required] |
**create_new_data_streaming_request** | [**CreateNewDataStreamingRequest**](CreateNewDataStreamingRequest.md) |  | [required] |

### Return type

[**crate::models::CreateNewDataStreaming201Response**](CreateNewDataStreaming_201_response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_streaming_by_id

> crate::models::DataStreamingsById list_data_streaming_by_id(data_streaming_id)
Get expecific data streaming by Data Streaming ID

Use the GET method and add the data streaming's ID to the URI of the request to get more data on a specific data streaming.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_streaming_id** | **i32** |  | [required] |

### Return type

[**crate::models::DataStreamingsById**](DataStreamingsById.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_streamings

> crate::models::DataStreamingResponseWithResults list_data_streamings()
List all exist data streamings

Use the GET method to list all data streamings, both active and inactive, and its properties.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataStreamingResponseWithResults**](DataStreamingResponseWithResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## overwrite_data_streaming_by_id

> crate::models::CreateNewDataStreaming201Response overwrite_data_streaming_by_id(data_streaming_id, create_new_data_streaming_request)
Overwrite data streaming

Use the PUT method to overwrite the data streaming. Although  you can change a single property using the PUT method, you must pass all fields for the request to be completed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_streaming_id** | **i32** |  | [required] |
**create_new_data_streaming_request** | [**CreateNewDataStreamingRequest**](CreateNewDataStreamingRequest.md) |  | [required] |

### Return type

[**crate::models::CreateNewDataStreaming201Response**](CreateNewDataStreaming_201_response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

