# \CreateCsrApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_csr**](CreateCsrApi.md#create_csr) | **POST** /digital_certificates/csr | Create an encrypted Certificate Request with Azion, which can then be sent for signing to a CA



## create_csr

> crate::models::Dc201 create_csr(create_csr_request)
Create an encrypted Certificate Request with Azion, which can then be sent for signing to a CA

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_csr_request** | [**CreateCsrRequest**](CreateCsrRequest.md) |  | [required] |

### Return type

[**crate::models::Dc201**](DC201.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

