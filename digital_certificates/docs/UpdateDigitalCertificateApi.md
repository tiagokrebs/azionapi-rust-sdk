# \UpdateDigitalCertificateApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_digital_certificate**](UpdateDigitalCertificateApi.md#update_digital_certificate) | **PATCH** /digital_certificates/{digital_certificate_id} | Change only select settings of your digital certificate or CSR.



## update_digital_certificate

> crate::models::Dc200 update_digital_certificate(digital_certificate_id, update_digital_certificate_request)
Change only select settings of your digital certificate or CSR.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digital_certificate_id** | **i32** | ID of certificate to update | [required] |
**update_digital_certificate_request** | [**UpdateDigitalCertificateRequest**](UpdateDigitalCertificateRequest.md) |  | [required] |

### Return type

[**crate::models::Dc200**](DC200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

