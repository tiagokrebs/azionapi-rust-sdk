# \RetrieveDigitalCertificateByIdApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_certificate**](RetrieveDigitalCertificateByIdApi.md#get_certificate) | **GET** /digital_certificates/{digital_certificate_id} | Get more data on a specific digital certificate or CSR.



## get_certificate

> crate::models::Dc200 get_certificate(digital_certificate_id)
Get more data on a specific digital certificate or CSR.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digital_certificate_id** | **i64** | ID of certificate | [required] |

### Return type

[**crate::models::Dc200**](DC200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

