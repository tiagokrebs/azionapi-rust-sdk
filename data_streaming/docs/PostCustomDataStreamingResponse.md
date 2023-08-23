# PostCustomDataStreamingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**data_source** | Option<**String**> | Options:  * `http` - Edge Applications  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History    | [optional]
**template_model** | Option<**String**> | Note:  * Add all variables and values that will be used to stream according to the data source you choose to use.    | [optional]
**active** | Option<**bool**> |  | [optional]
**endpoint** | Option<**String**> |  | [optional]
**all_domains** | Option<**bool**> | Note:  * Field not used with the rtm_activity data source.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


