# \DomainsApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_domain**](DomainsApi.md#create_domain) | **POST** /domains | /domains
[**del_domain**](DomainsApi.md#del_domain) | **DELETE** /domains/{id} | /domains/:id
[**get_domain**](DomainsApi.md#get_domain) | **GET** /domains/{id} | /domains/:id
[**get_domains**](DomainsApi.md#get_domains) | **GET** /domains | /domains
[**put_domain**](DomainsApi.md#put_domain) | **PUT** /domains/{id} | /domains:/:id
[**update_domain**](DomainsApi.md#update_domain) | **PATCH** /domains/{id} | /domains/:id



## create_domain

> models::DomainResponseWithResult create_domain(accept, content_type, create_domain_request)
/domains

It enables you to include a new domain into an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**create_domain_request** | Option<[**CreateDomainRequest**](CreateDomainRequest.md)> |  |  |

### Return type

[**models::DomainResponseWithResult**](DomainResponseWithResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## del_domain

> del_domain(id, accept)
/domains/:id

It enables you to delete a domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the domain to be deleted.  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain

> models::DomainResponseWithResult get_domain(id, accept)
/domains/:id

It returns details of a domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the domain to be consulted.  | [required] |
**accept** | Option<**String**> |  |  |

### Return type

[**models::DomainResponseWithResult**](DomainResponseWithResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domains

> models::DomainResponseWithResults get_domains(page, page_size, sort, order_by, accept)
/domains

It returns the list of domains of an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |
**sort** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**accept** | Option<**String**> |  |  |

### Return type

[**models::DomainResponseWithResults**](DomainResponseWithResults.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_domain

> models::DomainResponseWithResult put_domain(id, accept, content_type, put_domain_request)
/domains:/:id

It overwrites all fields of a domain, while preserving the id. Optional fields not filled in will be replaced by the default values.  To update only some fields in a domain, consider using the PATCH method instead of PUT.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**put_domain_request** | Option<[**PutDomainRequest**](PutDomainRequest.md)> |  |  |

### Return type

[**models::DomainResponseWithResult**](DomainResponseWithResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_domain

> models::DomainResponseWithResult update_domain(id, accept, content_type, update_domain_request)
/domains/:id

It updates one or more fields in a Domain, preserving the value of the fields not informed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**accept** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**update_domain_request** | Option<[**UpdateDomainRequest**](UpdateDomainRequest.md)> |  |  |

### Return type

[**models::DomainResponseWithResult**](DomainResponseWithResult.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

