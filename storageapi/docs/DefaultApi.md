# \DefaultApi

All URIs are relative to *https://storage-api.azion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_version**](DefaultApi.md#delete_version) | **DELETE** /storage/{version_id}/delete | /storage/:version_id/delete
[**storage_version_id_post**](DefaultApi.md#storage_version_id_post) | **POST** /storage/{version_id} | /storage/:version_id



## delete_version

> delete_version(version_id)
/storage/:version_id/delete

Delete a version. A version is just um path prefix/sub-namespace for a set of files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **String** | The version identifier | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_version_id_post

> serde_json::Value storage_version_id_post(x_azion_static_path, version_id, content_type, body)
/storage/:version_id

Upload file and transfer to remote storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_azion_static_path** | **String** | Required in order to get the path and file name. i.e.: assets/css/main.css | [required] |
**version_id** | **String** |  | [required] |
**content_type** | Option<**String**> | The content type of the file (Example: text/plain). |  |[default to b2/x-auto]
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

