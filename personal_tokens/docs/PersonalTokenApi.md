# \PersonalTokenApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_personal_token**](PersonalTokenApi.md#get_personal_token) | **GET** /iam/personal_tokens/{personalTokenId} | Get a personal token info
[**list_personal_token**](PersonalTokenApi.md#list_personal_token) | **GET** /iam/personal_tokens | List all existing personal token



## get_personal_token

> crate::models::PersonalTokenResponseGet get_personal_token(personal_token_id)
Get a personal token info

Get a personal token info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personal_token_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::PersonalTokenResponseGet**](PersonalTokenResponseGet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_personal_token

> crate::models::PersonalTokenResponseWithResults list_personal_token()
List all existing personal token

List all existing personal token

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PersonalTokenResponseWithResults**](PersonalTokenResponseWithResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

