# \DefaultApi

All URIs are relative to *https://api.azionapi.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize_edge_node**](DefaultApi.md#authorize_edge_node) | **PATCH** /edge_nodes/authorize | Authorize edge-node
[**create_edge_node_svcs**](DefaultApi.md#create_edge_node_svcs) | **POST** /edge_nodes/{edgenodeId}/services | Create an edge-node Service association
[**del_edge_node**](DefaultApi.md#del_edge_node) | **DELETE** /edge_nodes/{edgenodeId} | Delete edge-node by ID
[**del_edge_node_svc**](DefaultApi.md#del_edge_node_svc) | **DELETE** /edge_nodes/{edgenodeId}/services/{bindId} | Delete an edge-node Service association
[**get_edge_node**](DefaultApi.md#get_edge_node) | **GET** /edge_nodes/{edgenodeId} | Return edge-node by ID
[**get_edge_node_groups**](DefaultApi.md#get_edge_node_groups) | **GET** /edge_nodes/groups | Return edge-node groups
[**get_edge_node_svc_detail**](DefaultApi.md#get_edge_node_svc_detail) | **GET** /edge_nodes/{edgenodeId}/services/{bindId} | Return edge-node Service association by ID
[**get_edge_node_svcs**](DefaultApi.md#get_edge_node_svcs) | **GET** /edge_nodes/{edgenodeId}/services | Return edge-node Services association
[**get_edge_nodes**](DefaultApi.md#get_edge_nodes) | **GET** /edge_nodes | Return edge-nodes
[**update_edge_node**](DefaultApi.md#update_edge_node) | **PATCH** /edge_nodes/{edgenodeId} | Update edge-node
[**update_edge_node_svc_detail**](DefaultApi.md#update_edge_node_svc_detail) | **PATCH** /edge_nodes/{edgenodeId}/services/{bindId} | Update edge-node Service association by ID



## authorize_edge_node

> crate::models::AuthorizeEdgeNodesResponse authorize_edge_node(authorize_edge_nodes_request)
Authorize edge-node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorize_edge_nodes_request** | [**AuthorizeEdgeNodesRequest**](AuthorizeEdgeNodesRequest.md) |  | [required] |

### Return type

[**crate::models::AuthorizeEdgeNodesResponse**](AuthorizeEdgeNodesResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_edge_node_svcs

> crate::models::ServiceBindDetailResponse create_edge_node_svcs(edgenode_id, service_bind_request)
Create an edge-node Service association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |
**service_bind_request** | [**ServiceBindRequest**](ServiceBindRequest.md) |  | [required] |

### Return type

[**crate::models::ServiceBindDetailResponse**](ServiceBindDetailResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## del_edge_node

> del_edge_node(edgenode_id)
Delete edge-node by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## del_edge_node_svc

> del_edge_node_svc(edgenode_id, bind_id)
Delete an edge-node Service association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |
**bind_id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edge_node

> crate::models::EdgeNodeDetailResponse get_edge_node(edgenode_id)
Return edge-node by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |

### Return type

[**crate::models::EdgeNodeDetailResponse**](EdgeNodeDetailResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edge_node_groups

> Vec<crate::models::NodeGroupResponse> get_edge_node_groups()
Return edge-node groups

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NodeGroupResponse>**](NodeGroupResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edge_node_svc_detail

> crate::models::ServiceBindDetailResponse get_edge_node_svc_detail(edgenode_id, bind_id)
Return edge-node Service association by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |
**bind_id** | **i64** |  | [required] |

### Return type

[**crate::models::ServiceBindDetailResponse**](ServiceBindDetailResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edge_node_svcs

> crate::models::ServiceResponseWithTotal get_edge_node_svcs(edgenode_id, is_bound, filter, order_by, sort, page, page_size)
Return edge-node Services association

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |
**is_bound** | Option<**bool**> |  |  |
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |
**page_size** | Option<**i64**> |  |  |

### Return type

[**crate::models::ServiceResponseWithTotal**](ServiceResponseWithTotal.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edge_nodes

> crate::models::EdgeNodeResponseWithTotal get_edge_nodes(filter, order_by, sort, only_ids, page_size)
Return edge-nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> |  |  |
**order_by** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**only_ids** | Option<**bool**> |  |  |
**page_size** | Option<**i64**> |  |  |

### Return type

[**crate::models::EdgeNodeResponseWithTotal**](EdgeNodeResponseWithTotal.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_edge_node

> crate::models::UpdateEdgeNodeResponse update_edge_node(edgenode_id)
Update edge-node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |

### Return type

[**crate::models::UpdateEdgeNodeResponse**](UpdateEdgeNodeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_edge_node_svc_detail

> crate::models::ServiceBindDetailResponse update_edge_node_svc_detail(edgenode_id, bind_id, update_service_bind_request)
Update edge-node Service association by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgenode_id** | **i64** |  | [required] |
**bind_id** | **i64** |  | [required] |
**update_service_bind_request** | [**UpdateServiceBindRequest**](UpdateServiceBindRequest.md) |  | [required] |

### Return type

[**crate::models::ServiceBindDetailResponse**](ServiceBindDetailResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json; version=3
- **Accept**: application/json; version=3

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

