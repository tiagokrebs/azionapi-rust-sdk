# \OverwriteDigitalCertificateApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**overwrite_digital_certificate**](OverwriteDigitalCertificateApi.md#overwrite_digital_certificate) | **PUT** /digital_certificates/{digital_certificate_id} | Overwrite a digital certificate with another complete digital certificate



## overwrite_digital_certificate

> crate::models::Dc200 overwrite_digital_certificate(digital_certificate_id, create_certificate_request)
Overwrite a digital certificate with another complete digital certificate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digital_certificate_id** | **i32** | ID of certificate to overwrite | [required] |
**create_certificate_request** | [**CreateCertificateRequest**](CreateCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::Dc200**](DC200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

