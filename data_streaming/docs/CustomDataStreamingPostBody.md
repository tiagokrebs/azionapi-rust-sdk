# CustomDataStreamingPostBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**data_source** | Option<**String**> | Options:  * `http` - Edge Applications (default)  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History    | [optional]
**template_model** | Option<**String**> | Note:  * Add all variables and values that will be used to stream according to the data source you choose to use.    * All data streaming [variables can be found on the reference documentation](https://www.azion.com/en/documentation/products/data-streaming/#selecting-data-sources).    | [optional]
**active** | Option<**bool**> |  | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


