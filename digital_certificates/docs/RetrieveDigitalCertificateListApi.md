# \RetrieveDigitalCertificateListApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_digital_certificates**](RetrieveDigitalCertificateListApi.md#list_digital_certificates) | **GET** /digital_certificates | List all existing digital certificates and CSRs registered to your account



## list_digital_certificates

> crate::models::Dc200List list_digital_certificates(order_by, sort, name, search)
List all existing digital certificates and CSRs registered to your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Response field to order by, current options are \"name\" or \"id\" |  |[default to id]
**sort** | Option<**String**> | Sorting direction |  |[default to asc]
**name** | Option<**String**> | Searches certificate for name |  |
**search** | Option<**String**> | Searches for string in certificate name |  |

### Return type

[**crate::models::Dc200List**](DC200List.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

