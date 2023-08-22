# CreateNewDataStreamingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**template_id** | Option<**i32**> | Options:  * `2` - Edge Applications Event Collector  * `4` - WAF Event Collector  * `86` - Edge Functions Event Collector  * `184` - Edge Applications + WAF Event Collector  * `251` - Activity History Collector  | [optional]
**data_source** | Option<**String**> | Options:  * `http` - Edge Applications (default)  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History    | [optional]
**active** | Option<**bool**> |  | [optional][default to true]
**endpoint** | Option<[**crate::models::DataStreamingEndpointTypeStandard**](DataStreamingEndpointTypeStandard.md)> |  | [optional]
**domains_ids** | Option<**Vec<i32>**> | Note:  * Field not used with the rtm_activity data source.  | [optional]
**all_domains** | Option<**bool**> | Note:  * Field not used with the rtm_activity data source.  | [optional][default to false]
**sampling_percentage** | Option<**i32**> | Note:  * `Range` - From 0 to 100.  * `To use:` [Contact the sales team](https://www.azion.com/en/contact-sales/) to activate this feature in your account.  | [optional]
**template_model** | Option<**String**> | Note:  * Add all variables and values that will be used to stream according to the data source you choose to use.    * All data streaming [variables can be found on the reference documentation](https://www.azion.com/en/documentation/products/data-streaming/#selecting-data-sources).    | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


