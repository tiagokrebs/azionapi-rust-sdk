# \CreateDigitalCertificateApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_certificate**](CreateDigitalCertificateApi.md#create_certificate) | **POST** /digital_certificates | Create a new digital certificate



## create_certificate

> crate::models::Dc201 create_certificate(create_certificate_request)
Create a new digital certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_certificate_request** | [**CreateCertificateRequest**](CreateCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::Dc201**](DC201.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

