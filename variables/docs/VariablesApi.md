# \VariablesApi

All URIs are relative to *https://stage-variables.azion.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_schema_retrieve**](VariablesApi.md#api_schema_retrieve) | **GET** /api/schema | 
[**api_variables_create**](VariablesApi.md#api_variables_create) | **POST** /api/variables | 
[**api_variables_destroy**](VariablesApi.md#api_variables_destroy) | **DELETE** /api/variables/{uuid} | 
[**api_variables_list**](VariablesApi.md#api_variables_list) | **GET** /api/variables | 
[**api_variables_retrieve**](VariablesApi.md#api_variables_retrieve) | **GET** /api/variables/{uuid} | 
[**api_variables_update**](VariablesApi.md#api_variables_update) | **PUT** /api/variables/{uuid} | 



## api_schema_retrieve

> ::std::collections::HashMap<String, serde_json::Value> api_schema_retrieve(format, lang)


OpenApi3 schema for this API. Format can be selected via content negotiation.  - YAML: application/vnd.oai.openapi - JSON: application/vnd.oai.openapi+json

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**lang** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.oai.openapi, application/yaml, application/vnd.oai.openapi+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variables_create

> crate::models::VariableGet api_variables_create(variable_create)


Create a new Variable. <br><ul><li>If the attribute \"secret\" is informed with value \"true\" in request payload the Variable value will be secret and no longer viewable after creation.</li><li>If the attribute \"secret\" is not informed the Variable value will be considered as not secret by default.</li></ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_create** | [**VariableCreate**](VariableCreate.md) |  | [required] |

### Return type

[**crate::models::VariableGet**](VariableGet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variables_destroy

> api_variables_destroy(uuid)


Delete a Variable by it's UUID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variables_list

> Vec<crate::models::Variable> api_variables_list()


List all user's Variables.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Variable>**](Variable.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variables_retrieve

> crate::models::Variable api_variables_retrieve(uuid)


Retrieve all data for a Variable by it's UUID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**crate::models::Variable**](Variable.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variables_update

> crate::models::VariableGet api_variables_update(uuid, variable_create)


Update variable attributes by it's UUID. Keep the Variable UUID but overwrite all editable attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**variable_create** | [**VariableCreate**](VariableCreate.md) |  | [required] |

### Return type

[**crate::models::VariableGet**](VariableGet.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

