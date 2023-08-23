# \DataStreamingDomainApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_data_streaming**](DataStreamingDomainApi.md#list_data_streaming) | **GET** /data_streaming/domains | List all domains used on data streaming



## list_data_streaming

> crate::models::DataStreamingsDomainResponse list_data_streaming(name, streaming_id, selected)
List all domains used on data streaming

Use the GET method to list all available domains that can be used on Data Streaming operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Domain's name in data streaming |  |
**streaming_id** | Option<**i64**> |  |  |
**selected** | Option<**bool**> |  |  |

### Return type

[**crate::models::DataStreamingsDomainResponse**](DataStreamingsDomainResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

