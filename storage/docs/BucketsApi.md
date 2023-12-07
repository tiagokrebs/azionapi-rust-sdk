# \BucketsApi

All URIs are relative to *https://api.azion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_storage_buckets_create**](BucketsApi.md#api_v1_storage_buckets_create) | **POST** /v4/storage/buckets | /v4/storage/buckets
[**api_v1_storage_buckets_destroy**](BucketsApi.md#api_v1_storage_buckets_destroy) | **DELETE** /v4/storage/buckets/{name} | /v4/storage/buckets/:name
[**api_v1_storage_buckets_list**](BucketsApi.md#api_v1_storage_buckets_list) | **GET** /v4/storage/buckets | /v4/storage/buckets
[**api_v1_storage_buckets_partial_update**](BucketsApi.md#api_v1_storage_buckets_partial_update) | **PATCH** /v4/storage/buckets/{name} | /v4/storage/buckets/:name



## api_v1_storage_buckets_create

> crate::models::ResponseBucket api_v1_storage_buckets_create(bucket_create)
/v4/storage/buckets



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket_create** | [**BucketCreate**](BucketCreate.md) |  | [required] |

### Return type

[**crate::models::ResponseBucket**](ResponseBucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_storage_buckets_destroy

> crate::models::ResponseDeleteBucket api_v1_storage_buckets_destroy(name)
/v4/storage/buckets/:name



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::ResponseDeleteBucket**](ResponseDeleteBucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_storage_buckets_list

> crate::models::PaginatedBucketList api_v1_storage_buckets_list(page, page_size)
/v4/storage/buckets



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |

### Return type

[**crate::models::PaginatedBucketList**](PaginatedBucketList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_storage_buckets_partial_update

> crate::models::ResponseBucket api_v1_storage_buckets_partial_update(name, patched_bucket)
/v4/storage/buckets/:name



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**patched_bucket** | Option<[**PatchedBucket**](PatchedBucket.md)> |  |  |

### Return type

[**crate::models::ResponseBucket**](ResponseBucket.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

