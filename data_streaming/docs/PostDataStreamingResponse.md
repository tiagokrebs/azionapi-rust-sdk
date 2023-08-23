# PostDataStreamingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**template_id** | Option<**i32**> | Options:  * `2` - Edge Applications Event Collector  * `4` - WAF Event Collector  * `86` - Edge Functions Event Collector  * `184` - Edge Applications + WAF Event Collector  * `251` - Activity History Collector  | [optional]
**data_source** | Option<**String**> | Options:  * `http` - Edge Applications  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History  | [optional]
**active** | Option<**bool**> |  | [optional]
**endpoint** | Option<[**Vec<crate::models::PostDataStreamingResponseEndpointInner>**](PostDataStreamingResponse_endpoint_inner.md)> |  | [optional]
**all_domains** | Option<**bool**> | Note:  * Field not used with the rtm_activity data source.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


