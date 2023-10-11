# \WafApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_waf_ruleset**](WafApi.md#create_new_waf_ruleset) | **POST** /waf/rulesets | Create a new WAF Rule Set in an account.
[**delete_waf_ruleset**](WafApi.md#delete_waf_ruleset) | **DELETE** /waf/rulesets/{waf_rule_set_id} | Remove an WAF Rule Set from an account. Warning: this action cannot be undone.
[**get_waf_domains**](WafApi.md#get_waf_domains) | **GET** /waf/{waf_id}/domains | List all domains attached to a Web Application Firewall (WAF) in an account.
[**get_waf_events**](WafApi.md#get_waf_events) | **GET** /waf/{waf_id}/waf_events | Find WAF log events
[**get_waf_ruleset**](WafApi.md#get_waf_ruleset) | **GET** /waf/rulesets/{waf_rule_set_id} | List a specific Rule Set associated to a Web Application Firewall (WAF) in an account.
[**list_all_waf**](WafApi.md#list_all_waf) | **GET** /waf | List all Web Application Firewalls (WAFs) created in an account
[**list_all_waf_rulesets**](WafApi.md#list_all_waf_rulesets) | **GET** /waf/rulesets | list all Rule Sets associated to a Web Application Firewall (WAF) in an account.
[**update_waf_ruleset**](WafApi.md#update_waf_ruleset) | **PATCH** /waf/rulesets/{waf_rule_set_id} | Change only select settings of a WAF Rule Set



## create_new_waf_ruleset

> crate::models::SingleWaf create_new_waf_ruleset(create_new_waf_ruleset_request)
Create a new WAF Rule Set in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_new_waf_ruleset_request** | Option<[**CreateNewWafRulesetRequest**](CreateNewWafRulesetRequest.md)> |  |  |

### Return type

[**crate::models::SingleWaf**](SingleWAF.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_waf_ruleset

> delete_waf_ruleset(waf_rule_set_id)
Remove an WAF Rule Set from an account. Warning: this action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_set_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_waf_domains

> crate::models::WafDomains200 get_waf_domains(waf_id, name)
List all domains attached to a Web Application Firewall (WAF) in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_id** | **i64** | ID of WAF to return | [required] |
**name** | Option<**String**> | searches WAF for name |  |

### Return type

[**crate::models::WafDomains200**](WAFDomains200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_waf_events

> crate::models::WafEvents200 get_waf_events(waf_id, hour_range, domains_ids, network_list_id, sort, page, page_size)
Find WAF log events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_id** | **i64** | ID of WAF to return | [required] |
**hour_range** | **i64** | Last log hours since now (it must be a integer number ranging between 1 and 72) | [required] |
**domains_ids** | **String** | Multiple domain's id (they must be separated by comma like 1233,1234) | [required] |
**network_list_id** | Option<**i64**> | Id of a network list |  |
**sort** | Option<**String**> |  |  |[default to asc]
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 10]

### Return type

[**crate::models::WafEvents200**](WAFEvents200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_waf_ruleset

> crate::models::WafSingle200 get_waf_ruleset(waf_rule_set_id)
List a specific Rule Set associated to a Web Application Firewall (WAF) in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_set_id** | **i64** | ID of WAF Ruleset to return | [required] |

### Return type

[**crate::models::WafSingle200**](WAFSingle200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_waf

> crate::models::WafList200 list_all_waf(page, page_size)
List all Web Application Firewalls (WAFs) created in an account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | Identifies which page should be returned, if the return is paginated. |  |[default to 1]
**page_size** | Option<**i64**> | Identifies how many items should be returned per page. |  |[default to 10]

### Return type

[**crate::models::WafList200**](WAFList200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_waf_rulesets

> crate::models::WafList200 list_all_waf_rulesets(order_by, sort, page, page_size)
list all Rule Sets associated to a Web Application Firewall (WAF) in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Identifies which property the return should be sorted by. |  |[default to name]
**sort** | Option<**String**> | Defines whether objects are shown in ascending or descending order depending on the value set in order_by. |  |[default to asc]
**page** | Option<**i64**> | Identifies which page should be returned, if the return is paginated. |  |[default to 1]
**page_size** | Option<**i64**> | Identifies how many items should be returned per page. |  |[default to 10]

### Return type

[**crate::models::WafList200**](WAFList200.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_waf_ruleset

> crate::models::SingleWaf update_waf_ruleset(waf_rule_set_id, single_waf)
Change only select settings of a WAF Rule Set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**waf_rule_set_id** | **String** |  | [required] |
**single_waf** | Option<[**SingleWaf**](SingleWaf.md)> |  |  |

### Return type

[**crate::models::SingleWaf**](SingleWAF.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

