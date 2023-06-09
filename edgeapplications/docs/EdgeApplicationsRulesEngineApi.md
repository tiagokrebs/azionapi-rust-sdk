# \EdgeApplicationsRulesEngineApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_applications_edge_application_id_rules_engine_phase_rules_get**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_get) | **GET** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
[**edge_applications_edge_application_id_rules_engine_phase_rules_post**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_post) | **POST** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
[**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete) | **DELETE** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
[**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get) | **GET** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
[**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch) | **PATCH** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:
[**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put**](EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put) | **PUT** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:



## edge_applications_edge_application_id_rules_engine_phase_rules_get

> crate::models::RulesEngineResponse edge_applications_edge_application_id_rules_engine_phase_rules_get(edge_application_id, phase, page, page_size, filter, order_by, sort, accept)
/edge_applications/{edge_application_id}/rules_engine/{phase}/rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**phase** | **String** |  | [required] |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**accept** | Option<**String**> |  |  |

### Return type

[**crate::models::RulesEngineResponse**](RulesEngineResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_rules_engine_phase_rules_post

> crate::models::RulesEngineIdResponse edge_applications_edge_application_id_rules_engine_phase_rules_post(edge_application_id, phase, accept, content_type, create_rules_engine_request)
/edge_applications/{edge_application_id}/rules_engine/{phase}/rules

Check below the list of behaviors that can be applied:  | Name                                | Behavior               | | ----------------------------------- | ---------------------- | | Add Request Cookie                  | add_request_cookie     | | Add Request Header                  | add_request_header     | | Add Response Cookie                 | set_cookie             | | Add Response Header                 | add_response_header    | | Bypass Cache                        | bypass_cache_phase     | | Capture Match Groups                | capture_match_groups   | | Deliver                             | deliver                | | Deny (403 Forbidden)                | deny                   | | Enable Gzip                         | enable_gzip            | | Filter Request Cookie               | filter_request_cookie  | | Filter Request Header               | filter_request_header  | | Filter Response Cookie              | filter_response_cookie | | Filter Response Header              | filter_response_header | | Finish Request Phase                | finish_request_phase   | | Forward Cookies                     | forward_cookies        | | Optimize Images                     | optimize_images        | | Redirect HTTP to HTTPS              | redirect_http_to_https | | Redirect To (301 Moved Permanently) | redirect_to_301        | | Redirect To (302 Found)             | redirect_to_302        | | Rewrite Request                     | rewrite_request        | | Run Function                        | run_function           | | Set Cache Policy                    | set_cache_policy       | | Set Origin                          | set_origin             |

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**phase** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**create_rules_engine_request** | Option<[**CreateRulesEngineRequest**](CreateRulesEngineRequest.md)> |  |  |

### Return type

[**crate::models::RulesEngineIdResponse**](RulesEngineIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete

> edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete(edge_application_id, phase, rule_id, accept)
/edge_applications/{edge_application_id}/rules_engine/{phase}/rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** | The id of the edge application you plan to delete.  | [required] |
**phase** | **String** |  | [required] |
**rule_id** | **i64** | The id of the rule you plan to delete.  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get

> crate::models::RulesEngineIdResponse edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get(edge_application_id, phase, rule_id, accept)
/edge_applications/{edge_application_id}/rules_engine/{phase}/rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** | The id of the edge application you want to get.  | [required] |
**phase** | **String** |  | [required] |
**rule_id** | **i64** | The id of the rule you plan to delete.  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

[**crate::models::RulesEngineIdResponse**](RulesEngineIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch

> crate::models::RulesEngineIdResponse edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch(edge_application_id, phase, rule_id, accept, content_type, patch_rules_engine_request)
/edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**phase** | **String** |  | [required] |
**rule_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**patch_rules_engine_request** | Option<[**PatchRulesEngineRequest**](PatchRulesEngineRequest.md)> |  |  |

### Return type

[**crate::models::RulesEngineIdResponse**](RulesEngineIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put

> crate::models::RulesEngineIdResponse edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put(edge_application_id, phase, rule_id, accept, content_type, update_rules_engine_request)
/edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_application_id** | **i64** |  | [required] |
**phase** | **String** |  | [required] |
**rule_id** | **i64** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> | The type of coding used in the Body (application/json). <br>  Example: Content-Type: application/json |  |
**update_rules_engine_request** | Option<[**UpdateRulesEngineRequest**](UpdateRulesEngineRequest.md)> |  |  |

### Return type

[**crate::models::RulesEngineIdResponse**](RulesEngineIdResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

