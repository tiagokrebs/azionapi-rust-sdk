# \RecordsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_zone_record**](RecordsApi.md#delete_zone_record) | **DELETE** /intelligent_dns/{zone_id}/records/{record_id} | Remove an Intelligent DNS zone record
[**get_zone_records**](RecordsApi.md#get_zone_records) | **GET** /intelligent_dns/{zone_id}/records | Get a collection of Intelligent DNS zone records
[**post_zone_record**](RecordsApi.md#post_zone_record) | **POST** /intelligent_dns/{zone_id}/records | Create a new Intelligent DNS zone record
[**put_zone_record**](RecordsApi.md#put_zone_record) | **PUT** /intelligent_dns/{zone_id}/records/{record_id} | Update an Intelligent DNS zone record



## delete_zone_record

> String delete_zone_record(zone_id, record_id)
Remove an Intelligent DNS zone record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |
**record_id** | **i32** | The zone record id | [required] |

### Return type

**String**

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zone_records

> crate::models::GetRecordsResponse get_zone_records(zone_id)
Get a collection of Intelligent DNS zone records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |

### Return type

[**crate::models::GetRecordsResponse**](GetRecordsResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_zone_record

> crate::models::PostOrPutRecordResponse post_zone_record(zone_id, record_post_or_put)
Create a new Intelligent DNS zone record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |
**record_post_or_put** | Option<[**RecordPostOrPut**](RecordPostOrPut.md)> |  |  |

### Return type

[**crate::models::PostOrPutRecordResponse**](PostOrPutRecordResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_zone_record

> crate::models::PostOrPutRecordResponse put_zone_record(zone_id, record_id, record_post_or_put)
Update an Intelligent DNS zone record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **i32** | The hosted zone id | [required] |
**record_id** | **i32** | The zone record id | [required] |
**record_post_or_put** | Option<[**RecordPostOrPut**](RecordPostOrPut.md)> |  |  |

### Return type

[**crate::models::PostOrPutRecordResponse**](PostOrPutRecordResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

