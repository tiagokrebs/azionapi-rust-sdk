# \DefaultApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edge_firewall_edge_firewall_id_rules_engine_get**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_get) | **GET** /edge_firewall/{edge_firewall_id}/rules_engine | List all rule sets.
[**edge_firewall_edge_firewall_id_rules_engine_post**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_post) | **POST** /edge_firewall/{edge_firewall_id}/rules_engine | Create rule set.
[**edge_firewall_edge_firewall_id_rules_engine_rule_set_id_delete**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_rule_set_id_delete) | **DELETE** /edge_firewall/{edge_firewall_id}/rules_engine/{rule_set_id} | Delete rule set.
[**edge_firewall_edge_firewall_id_rules_engine_rule_set_id_get**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_rule_set_id_get) | **GET** /edge_firewall/{edge_firewall_id}/rules_engine/{rule_set_id} | Retrieve rule set by ID.
[**edge_firewall_edge_firewall_id_rules_engine_rule_set_id_patch**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_rule_set_id_patch) | **PATCH** /edge_firewall/{edge_firewall_id}/rules_engine/{rule_set_id} | Edit rule set.
[**edge_firewall_edge_firewall_id_rules_engine_rule_set_id_put**](DefaultApi.md#edge_firewall_edge_firewall_id_rules_engine_rule_set_id_put) | **PUT** /edge_firewall/{edge_firewall_id}/rules_engine/{rule_set_id} | Overwrite rule set
[**edge_firewall_get**](DefaultApi.md#edge_firewall_get) | **GET** /edge_firewall | List all user edge firewall
[**edge_firewall_post**](DefaultApi.md#edge_firewall_post) | **POST** /edge_firewall | Create a edge firewall
[**edge_firewall_uuid_delete**](DefaultApi.md#edge_firewall_uuid_delete) | **DELETE** /edge_firewall/{uuid} | Delete an edge firewall by uuid
[**edge_firewall_uuid_get**](DefaultApi.md#edge_firewall_uuid_get) | **GET** /edge_firewall/{uuid} | Retrieve an edge firewall set by uuid
[**edge_firewall_uuid_patch**](DefaultApi.md#edge_firewall_uuid_patch) | **PATCH** /edge_firewall/{uuid} | Update some edge firewall attributes, like \"active\"
[**edge_firewall_uuid_put**](DefaultApi.md#edge_firewall_uuid_put) | **PUT** /edge_firewall/{uuid} | Overwrite some edge firewall attributes, like \"active\"



## edge_firewall_edge_firewall_id_rules_engine_get

> crate::models::RuleSetResponseAll edge_firewall_edge_firewall_id_rules_engine_get(edge_firewall_id)
List all rule sets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |

### Return type

[**crate::models::RuleSetResponseAll**](RuleSetResponseAll.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_rules_engine_post

> crate::models::RuleSetResponse edge_firewall_edge_firewall_id_rules_engine_post(edge_firewall_id, create_rule_set_request)
Create rule set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**create_rule_set_request** | Option<[**CreateRuleSetRequest**](CreateRuleSetRequest.md)> |  |  |

### Return type

[**crate::models::RuleSetResponse**](RuleSetResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_rules_engine_rule_set_id_delete

> edge_firewall_edge_firewall_id_rules_engine_rule_set_id_delete(edge_firewall_id, rule_set_id)
Delete rule set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**rule_set_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_rules_engine_rule_set_id_get

> crate::models::RuleSetResult edge_firewall_edge_firewall_id_rules_engine_rule_set_id_get(edge_firewall_id, rule_set_id, order_by, sort, page, page_size)
Retrieve rule set by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**rule_set_id** | **i64** |  | [required] |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 10]

### Return type

[**crate::models::RuleSetResult**](RuleSetResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_rules_engine_rule_set_id_patch

> crate::models::RuleSetResult edge_firewall_edge_firewall_id_rules_engine_rule_set_id_patch(edge_firewall_id, rule_set_id, create_rule_set_request)
Edit rule set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**rule_set_id** | **i64** |  | [required] |
**create_rule_set_request** | Option<[**CreateRuleSetRequest**](CreateRuleSetRequest.md)> |  |  |

### Return type

[**crate::models::RuleSetResult**](RuleSetResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_edge_firewall_id_rules_engine_rule_set_id_put

> crate::models::RuleSetResult edge_firewall_edge_firewall_id_rules_engine_rule_set_id_put(edge_firewall_id, rule_set_id, create_rule_set_request)
Overwrite rule set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_firewall_id** | **i64** |  | [required] |
**rule_set_id** | **i64** |  | [required] |
**create_rule_set_request** | Option<[**CreateRuleSetRequest**](CreateRuleSetRequest.md)> |  |  |

### Return type

[**crate::models::RuleSetResult**](RuleSetResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_get

> crate::models::ListEdgeFirewallResponse edge_firewall_get(page, page_size, sort, order_by)
List all user edge firewall

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**sort** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |

### Return type

[**crate::models::ListEdgeFirewallResponse**](ListEdgeFirewallResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_post

> crate::models::EdgeFirewallResponse edge_firewall_post(create_edge_firewall_request)
Create a edge firewall

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_edge_firewall_request** | [**CreateEdgeFirewallRequest**](CreateEdgeFirewallRequest.md) |  | [required] |

### Return type

[**crate::models::EdgeFirewallResponse**](EdgeFirewallResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_uuid_delete

> edge_firewall_uuid_delete(uuid)
Delete an edge firewall by uuid

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


## edge_firewall_uuid_get

> crate::models::EdgeFirewallResponse edge_firewall_uuid_get(uuid)
Retrieve an edge firewall set by uuid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**crate::models::EdgeFirewallResponse**](EdgeFirewallResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_uuid_patch

> crate::models::EdgeFirewallResponse edge_firewall_uuid_patch(uuid, update_edge_firewall_request)
Update some edge firewall attributes, like \"active\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**update_edge_firewall_request** | [**UpdateEdgeFirewallRequest**](UpdateEdgeFirewallRequest.md) |  | [required] |

### Return type

[**crate::models::EdgeFirewallResponse**](EdgeFirewallResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edge_firewall_uuid_put

> crate::models::EdgeFirewallResponse edge_firewall_uuid_put(uuid, update_edge_firewall_request)
Overwrite some edge firewall attributes, like \"active\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**update_edge_firewall_request** | [**UpdateEdgeFirewallRequest**](UpdateEdgeFirewallRequest.md) |  | [required] |

### Return type

[**crate::models::EdgeFirewallResponse**](EdgeFirewallResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

