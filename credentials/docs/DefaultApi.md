# \DefaultApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_credential**](DefaultApi.md#create_credential) | **POST** /credentials | Create credential
[**delete_credential**](DefaultApi.md#delete_credential) | **DELETE** /credentials/{credentialId} | Delete the Credential
[**find_all**](DefaultApi.md#find_all) | **GET** /credentials | Return all credentials
[**load_credential**](DefaultApi.md#load_credential) | **GET** /credentials/{credentialId} | Load the Credential
[**update_credential**](DefaultApi.md#update_credential) | **PATCH** /credentials/{credentialId} | Update the Credential



## create_credential

> crate::models::AuthToken create_credential(create_credential_request)
Create credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_credential_request** | [**CreateCredentialRequest**](CreateCredentialRequest.md) |  | [required] |

### Return type

[**crate::models::AuthToken**](AuthToken.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_credential

> delete_credential(credential_id)
Delete the Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all

> crate::models::ResponseWithTotal find_all(filter, page, page_size, sort, order_by)
Return all credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**sort** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |

### Return type

[**crate::models::ResponseWithTotal**](ResponseWithTotal.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_credential

> crate::models::Response load_credential(credential_id)
Load the Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** |  | [required] |

### Return type

[**crate::models::Response**](Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_credential

> crate::models::Response update_credential(credential_id, update_credential_request)
Update the Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** |  | [required] |
**update_credential_request** | [**UpdateCredentialRequest**](UpdateCredentialRequest.md) |  | [required] |

### Return type

[**crate::models::Response**](Response.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

