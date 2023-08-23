# \DataStreamingTemplatesApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data_straming_template_by_id**](DataStreamingTemplatesApi.md#get_data_straming_template_by_id) | **GET** /data_streaming/templates/{template_id} | Get an global Template info by template ID
[**list_data_streaming_templates**](DataStreamingTemplatesApi.md#list_data_streaming_templates) | **GET** /data_streaming/templates | List all global Templates that can be used on Data Streaming operations



## get_data_straming_template_by_id

> crate::models::TemplateResultById get_data_straming_template_by_id(template_id)
Get an global Template info by template ID

Use the GET method and add the data streaming's ID to the URI of the request to get more data on a specific data streaming global template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** |  | [required] |

### Return type

[**crate::models::TemplateResultById**](TemplateResultById.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_streaming_templates

> crate::models::TemplateResults list_data_streaming_templates()
List all global Templates that can be used on Data Streaming operations

Use the GET method to list all global templates that can be used on Data Streaming operations.  **Note:** Customized templates won't be listed. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TemplateResults**](TemplateResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

