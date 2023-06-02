# Rust API client for edgeapplications

## Welcome to the Azion API!

With the following APIs you can check, remove or update existing settings, besides creating new ones.

* * *

## Overview

The Azion API is a RESTful API, based on HTTPS requests, that allows you to integrate your systems with our platform, simply, quickly, and securely.

Here you will find instructions on how our API works and what functionality is available.

This documentation is being constantly updated. Make sure you verify if there are any updates or changes before you perform any development in your application, even if you are familiar with our API.

* * *

Both HTTPS requests and responses must be in JavaScript Object Notation (JSON) format. All HTTPS requests and responses must follow these conventions.

**Base URL**

The base URL of the API, which must be used, is:

[**https://api.azionapi.net/_**](https://api.azionapi.net/)

**HTTP Methods**

Each HTTP method defines the type of operation that will be run.

| HTTP Method | CRUD | Whole Collection (e.g. /items) | Specific Item (e.g. /items/:item_id) |
| --- | --- | --- | --- |
| GET | Read | It retrieves the list of items in a Collection. | It retrieves a specific item in a Collection. |
| POST | Create | It creates a new item in the Collection. | \\- |
| PUT | Update/Replace | It replaces a whole Collection with a new one. | It replaces an item in a Collection with a new one. |
| PATCH | Update/Modify | It partially updates a Collection. | It partially updates an item in a Collection |
| DELETE | Delete | It deletes a whole Collection. | It deletes an item in a Collection. |

* * *

**Status Codes**

The HTTP return code defines the status – successful or not – after the requested operation is run.

| Status Code | Meaning |
| --- | --- |
| 200 OK | General Status for a successful operation. |
| 201 CREATED | Successfully created a collection or item, by means of POST or PUT. |
| 204 NO CONTENT | Successful operation, but without any content to be return to the Body. Generally used for DELETE or PUT operations. |
| 207 MULTI-STATUS | A batch of operations were triggered by a single request, which resulted in different return codes when it was run, for some of the operations. The codes are displayed in the “status” field, in the body of the response, for each sub-batch of operations for whichever are applicable. |
| 400 BAD REQUEST | Request error, such as invalid parameters, missing mandatory parameters or others. |
| 401 UNAUTHORIZED | Error caused by a missing HTTP Authentication header. |
| 403 FORBIDDEN | User does not have the permissions to run the requested operation. |
| 404 NOT FOUND | The requested resource does not exist. |
| 405 METHOD NOT ALLOWED | The requested method is not applicable with the URL. |
| 406 NOT ACCEPTABLE | Accept header missing from the HTTP request or the header contains formatting or the version is not supported by the API. |
| 409 CONFLICT | Conflict generated in running the request, such as a request to create an already existing record. |
| 429 TOO MANY REQUESTS | The request was temporarily rejected, due to exceeding the limit on operations. Wait for the time defined in the X-Ratelimit-Reset header and try again. |
| 500 INTERNAL SERVER ERROR | Unintentional error, due to an unidentified failure in the request process. |
| \\--- |  |
| **HTTP Headers** |  |

All requests must be generated with the HTTP header specifying the desired code format for responses and the API version used. The current version of the API is 3 and the format is application/json.

```
Accept: application/json; version=3

```

* * *

**Rate Limit**

The limit of operations that can be run via the API is defined by 3 HTTP response headers:

*   **X-ratelimit-limit:** number of operations allowed in one time-frame;
*   **X-ratelimit-remaining:** number of operations still available in one time-frame;
*   **X-ratelimit-reset:** is the date and time, in IOS 8601 format, which represents the point in the future when the time-frame will be closed and when the limits will, therefore, be reset.
    

Example of HTTP response headers and a request:

```
Date: Thu, 02 Nov 2017 12:30:28 GMT
X-ratelimit-remaining: 199
X-ratelimit-limit: 200
X-ratelimit-reset: 2017-11-02T12:31:28.675446

```

In the example provided, 200 operations are allowed within a 1-minute time frame. Of those, there are 199 still available, because one has already been run. The total limit will be reset after 1 minute.

When the X-ratelimit-limit is reached, or in other words, when the X-ratelimit-remaining reaches zero, the API will give the error, HTTP 429 TOO MANY REQUESTS. If your application receives this error, you will need to wait until the time defined in X-ratelimit-reset has passed, to make another request.

*   **X-ratelimit-limit by product**
    

The *X-ratelimit limit* variations by product are the following:

*   **Real-Time Metrics:** 20 requests per minute.
*   **Real-Time Purge:** 200 requests per minute; except for the Wildcard, which is 2000 a day.
    

> The rate-limit values are based on the client_id.

* * *

**Optional Parameters**

In this version, it is possible to include some optional parameters as part of the GET method, which can help and modify the form in which your data will be returned.

You can combine these parameters to get the result you want.

They are:

| Parameter | Description | Accepted values: |
| --- | --- | --- |
| order_by | Identifies which field the return should be sorted by. The default ordering is ascending. | Depends on the fields available from the endpoint structure |
| sort | Defines which ordering to be used: ascending or descending. | asc  <br>  <br>desc |
| page | Identifies which page should be returned, if the return is paginated. The default value is 1. | Page number. |
| page_size | Identifies how many items should be returned per page. The default value is 10. | Desired number of items. |

* * *

**Request Exemple**

You can use one parameter, or a combination. See the example below, which uses all of them in the same request.

```
GET /domains?order_by=name&page_size=20&sort=desc&page=3
Accept: application/json; version=3
Authorization: token 2909f3932069047f4736dc87e72baaddd19c9f75

```

* * *

# Authentication

Authentication and authorization of operations via Azion API is done through Tokens.

The first step is to create the Token through authenticating a user registered in [Real-Time Manager](https://sso.azion.com/login).

* * *

## Encoding Username and Password in Base64

Only token creation and cancelling operations are performed through Basic Authentication, that is, with a username and password. You can create and cancel the token through the API itself, but for that you need to encode your username and password in base64.

Base64 encoding can be done from the command line on a Unix terminal:

```
$ echo 'user@domain:password'|base64
dXNlckBkb21haW46cGFzc3dvcmQK

```

If you do not have a Unix terminal available, you can use the free online service at [https://www.base64encode.org/](https://www.base64encode.org/)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `edgeapplications` and add the following to `Cargo.toml` under `[dependencies]`:

```
edgeapplications = { path = "./edgeapplications" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.azionapi.net*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_cache_settings_delete**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_delete) | **DELETE** /edge_applications/{edge_application_id}/cache_settings/{cache_settings} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings:
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_cache_settings_id_get**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_get) | **GET** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_cache_settings_id_put**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_id_put) | **PUT** /edge_applications/{edge_application_id}/cache_settings/{cache_settings_id} | /edge_applications/:edge_application_id:/cache_settings/ca
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_cache_settings_patch**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_cache_settings_patch) | **PATCH** /edge_applications/{edge_application_id}/cache_settings/{cache_settings} | /edge_applications/:edge_application_id:/cache_settings/:cache_settings_id:
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_get**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_get) | **GET** /edge_applications/{edge_application_id}/cache_settings | /edge_applications/{edge_application_id}/cache_settings
*EdgeApplicationsCacheSettingsApi* | [**edge_applications_edge_application_id_cache_settings_post**](docs/EdgeApplicationsCacheSettingsApi.md#edge_applications_edge_application_id_cache_settings_post) | **POST** /edge_applications/{edge_application_id}/cache_settings | /edge_applications/:edge_application_id:/cache_settings
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_device_group_id_delete**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_delete) | **DELETE** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_device_group_id_get**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_get) | **GET** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_device_group_id_patch**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_patch) | **PATCH** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_device_group_id_put**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_device_group_id_put) | **PUT** /edge_applications/{edge_application_id}/device_groups/{device_group_id} | /edge_applications/{edge_application_id}/device_groups/{device_group_id}
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_get**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_get) | **GET** /edge_applications/{edge_application_id}/device_groups | /edge_applications/{edge_application_id}/device_groups
*EdgeApplicationsDeviceGroupsApi* | [**edge_applications_edge_application_id_device_groups_post**](docs/EdgeApplicationsDeviceGroupsApi.md#edge_applications_edge_application_id_device_groups_post) | **POST** /edge_applications/{edge_application_id}/device_groups | /edge_applications/{edge_application_id}/device_groups
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_functions_instances_id_delete**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_delete) | **DELETE** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_functions_instances_id_get**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_get) | **GET** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_functions_instances_id_patch**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_patch) | **PATCH** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_functions_instances_id_put**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_functions_instances_id_put) | **PUT** /edge_applications/{edge_application_id}/functions_instances/{functions_instances_id} | /edge_applications/:edge_application_id:/functions_instances/:functions_instances_id:
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_get**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_get) | **GET** /edge_applications/{edge_application_id}/functions_instances | /edge_applications/:edge_application_id:/functions_instances
*EdgeApplicationsEdgeFunctionsInstancesApi* | [**edge_applications_edge_application_id_functions_instances_post**](docs/EdgeApplicationsEdgeFunctionsInstancesApi.md#edge_applications_edge_application_id_functions_instances_post) | **POST** /edge_applications/{edge_application_id}/functions_instances | edge_application/:edge_application_id:/functions_instances
*EdgeApplicationsMainSettingsApi* | [**edge_applications_get**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_get) | **GET** /edge_applications | /edge_applications
*EdgeApplicationsMainSettingsApi* | [**edge_applications_id_delete**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_id_delete) | **DELETE** /edge_applications/{id} | /edge_applications/:id
*EdgeApplicationsMainSettingsApi* | [**edge_applications_id_get**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_id_get) | **GET** /edge_applications/{id} | /edge_applications/:id
*EdgeApplicationsMainSettingsApi* | [**edge_applications_id_patch**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_id_patch) | **PATCH** /edge_applications/{id} | /edge_applications/:id
*EdgeApplicationsMainSettingsApi* | [**edge_applications_id_put**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_id_put) | **PUT** /edge_applications/{id} | /edge_applications/:id
*EdgeApplicationsMainSettingsApi* | [**edge_applications_post**](docs/EdgeApplicationsMainSettingsApi.md#edge_applications_post) | **POST** /edge_applications | /edge_applications
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_get**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_get) | **GET** /edge_applications/{edge_application_id}/origins | /edge_applications/{edge_application_id}/origins
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_origin_key_delete**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_delete) | **DELETE** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_id}
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_origin_key_get**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_get) | **GET** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_key}
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_origin_key_patch**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_patch) | **PATCH** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/:edge_application_id:/origins/:origin_id:
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_origin_key_put**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_origin_key_put) | **PUT** /edge_applications/{edge_application_id}/origins/{origin_key} | /edge_applications/{edge_application_id}/origins/{origin_id}
*EdgeApplicationsOriginsApi* | [**edge_applications_edge_application_id_origins_post**](docs/EdgeApplicationsOriginsApi.md#edge_applications_edge_application_id_origins_post) | **POST** /edge_applications/{edge_application_id}/origins | /edge_applications/{edge_application_id}/origins
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_get**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_get) | **GET** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_post**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_post) | **POST** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_delete) | **DELETE** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_get) | **GET** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/{edge_application_id}/rules_engine/{phase}/rules
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_patch) | **PATCH** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:
*EdgeApplicationsRulesEngineApi* | [**edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put**](docs/EdgeApplicationsRulesEngineApi.md#edge_applications_edge_application_id_rules_engine_phase_rules_rule_id_put) | **PUT** /edge_applications/{edge_application_id}/rules_engine/{phase}/rules/{rule_id} | /edge_applications/:edge_application_id:/rules_engine/:phase:/rules/:rule_id:


## Documentation For Models

 - [ApplicationCacheCreateRequest](docs/ApplicationCacheCreateRequest.md)
 - [ApplicationCacheCreateResponse](docs/ApplicationCacheCreateResponse.md)
 - [ApplicationCacheCreateResults](docs/ApplicationCacheCreateResults.md)
 - [ApplicationCacheGetOneResponse](docs/ApplicationCacheGetOneResponse.md)
 - [ApplicationCacheGetResponse](docs/ApplicationCacheGetResponse.md)
 - [ApplicationCachePatchRequest](docs/ApplicationCachePatchRequest.md)
 - [ApplicationCachePatchResponse](docs/ApplicationCachePatchResponse.md)
 - [ApplicationCachePutRequest](docs/ApplicationCachePutRequest.md)
 - [ApplicationCachePutResponse](docs/ApplicationCachePutResponse.md)
 - [ApplicationCacheResponseDetails](docs/ApplicationCacheResponseDetails.md)
 - [ApplicationCacheResults](docs/ApplicationCacheResults.md)
 - [ApplicationCreateInstanceRequest](docs/ApplicationCreateInstanceRequest.md)
 - [ApplicationInstanceResults](docs/ApplicationInstanceResults.md)
 - [ApplicationInstancesGetOneResponse](docs/ApplicationInstancesGetOneResponse.md)
 - [ApplicationInstancesGetResponse](docs/ApplicationInstancesGetResponse.md)
 - [ApplicationInstancesResults](docs/ApplicationInstancesResults.md)
 - [ApplicationLinks](docs/ApplicationLinks.md)
 - [ApplicationOrigins](docs/ApplicationOrigins.md)
 - [ApplicationPutInstanceRequest](docs/ApplicationPutInstanceRequest.md)
 - [ApplicationPutRequest](docs/ApplicationPutRequest.md)
 - [ApplicationPutResult](docs/ApplicationPutResult.md)
 - [ApplicationResults](docs/ApplicationResults.md)
 - [ApplicationResultsCreate](docs/ApplicationResultsCreate.md)
 - [ApplicationUpdateInstanceRequest](docs/ApplicationUpdateInstanceRequest.md)
 - [ApplicationUpdateRequest](docs/ApplicationUpdateRequest.md)
 - [ApplicationUpdateResponse](docs/ApplicationUpdateResponse.md)
 - [ApplicationUpdateResults](docs/ApplicationUpdateResults.md)
 - [ApplicationsResults](docs/ApplicationsResults.md)
 - [CreateApplicationRequest](docs/CreateApplicationRequest.md)
 - [CreateApplicationResult](docs/CreateApplicationResult.md)
 - [CreateDeviceGroupsRequest](docs/CreateDeviceGroupsRequest.md)
 - [CreateOriginsRequest](docs/CreateOriginsRequest.md)
 - [CreateOriginsRequestAddresses](docs/CreateOriginsRequestAddresses.md)
 - [CreateRulesEngineRequest](docs/CreateRulesEngineRequest.md)
 - [DeviceGroupsIdResponse](docs/DeviceGroupsIdResponse.md)
 - [DeviceGroupsResponse](docs/DeviceGroupsResponse.md)
 - [DeviceGroupsResponseLinks](docs/DeviceGroupsResponseLinks.md)
 - [DeviceGroupsResultResponse](docs/DeviceGroupsResultResponse.md)
 - [GetApplicationResponse](docs/GetApplicationResponse.md)
 - [GetApplicationsResponse](docs/GetApplicationsResponse.md)
 - [OriginsIdResponse](docs/OriginsIdResponse.md)
 - [OriginsResponse](docs/OriginsResponse.md)
 - [OriginsResponseLinks](docs/OriginsResponseLinks.md)
 - [OriginsResultResponse](docs/OriginsResultResponse.md)
 - [OriginsResultResponseAddresses](docs/OriginsResultResponseAddresses.md)
 - [PatchDeviceGroupsRequest](docs/PatchDeviceGroupsRequest.md)
 - [PatchOriginsRequest](docs/PatchOriginsRequest.md)
 - [PatchRulesEngineRequest](docs/PatchRulesEngineRequest.md)
 - [RulesEngineBehavior](docs/RulesEngineBehavior.md)
 - [RulesEngineCriteria](docs/RulesEngineCriteria.md)
 - [RulesEngineIdResponse](docs/RulesEngineIdResponse.md)
 - [RulesEngineResponse](docs/RulesEngineResponse.md)
 - [RulesEngineResultResponse](docs/RulesEngineResultResponse.md)
 - [RulesEngineResultResponseBehaviors](docs/RulesEngineResultResponseBehaviors.md)
 - [UpdateDeviceGroupsRequest](docs/UpdateDeviceGroupsRequest.md)
 - [UpdateOriginsRequest](docs/UpdateOriginsRequest.md)
 - [UpdateRulesEngineRequest](docs/UpdateRulesEngineRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



