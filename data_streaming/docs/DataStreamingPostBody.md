# DataStreamingPostBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**template_id** | Option<**i32**> | Options:  * `2` - Edge Applications Event Collector  * `4` - WAF Event Collector  * `86` - Edge Functions Event Collector  * `184` - Edge Applications + WAF Event Collector  * `251` - Activity History Collector  | [optional]
**data_source** | Option<**String**> | Options:  * `http` - Edge Applications (default)  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History  | [optional]
**active** | Option<**bool**> |  | [optional][default to true]
**endpoint** | Option<**String**> | Options' examples:  - `Standard HTTP/HTTPS POST` - { \"endpoint_type\": \"standard\", \"url\": \"http://example.com\", \"log_line_separator\": \"\\n\", \"payload_format\": \"$dataset\", \"max_size\": 1000024 }  - `Apache Kafka` - { \"endpoint_type\": \"kafka\", \"kafka_topic\": \"example_topic\", \"bootstrap_servers\": \"kafka-server.com:9092,kafka-server-2.com:9092\", \"use_tls\":true }  - `Simple Storage Service (S3)` - { \"endpoint_type\": \"s3\", \"access_key\": \"MYACCESSKEY\", \"region\": \"us-east-1\", \"object_key_prefix\": \"my_prefix_\", \"bucket_name\": \"bucket_example\", \"content_type\": \"plain/text\", \"host_url\": \"http://aws-host.com\", \"secret_key\": \"MYSECRETKEY\" }  - `Google BigQuery` - { \"endpoint_type\": \"big_query\", \"dataset_id\": \"my_dataset\", \"project_id\": \"my_project\", \"table_id\": \"my_table\", \"service_account_key\": \"{ \"service_account_key\": \"key_content\" }\" }  - `Elasticsearch` - { “endpoint_type”: \"elasticsearch\", “url”: “http://elasticsearch.com”, “api_key”: “XYZ_API_KEY” }  - `AWS Kinesis Data Firehose` -  { \"endpoint_type\": \"aws_kinesis_firehose\", \"access_key\": \"MYACCESSKEY\", \"stream_name\": \"my_stream_name\", \"region\": \"us-east-1\", \"secret_key\": \"MYSECRETKEY\" }  - `Datadog` - { \"endpoint_type\": \"datadog\", \"url\": \"https://http-intake.logs.datadoghq.com/v1/input\", \"api_key\": \"MYAPIKEY\" }  - `IBM QRadar` - { \"endpoint_type\": \"qradar\", \"url\": \"http://137.15.824.10:14440” }  - `Azure Monitor` - { \"endpoint_type\": \"azure_monitor\", \"log_type\": \"myLogType\", \"shared_key\": \"mysharedkey\", \"time_generated_field\": \"timeGeneratedField\", \"workspace_id\": \"anfhw-123sd-466gcs\"}  - `Azure Blob Storage` - { \"endpoint_type\": \"azure_blob_storage\", \"storage_account\": \"mystorageaccount\", \"container_name\": \"log_container\", \"blob_sas_token\": \"fd56e23e1f12efe\" }  - `Splunk` - { \"endpoint_type\": \"splunk\", \"url\": \"https://inputs.splunk-client.splunkcloud.com:1337/services/collector\", \"api_key\": \"MYAPIKEY\" }  | [optional]
**domains_ids** | Option<**Vec<i32>**> | Note:  * Field not used with the rtm_activity data source.  | [optional]
**all_domains** | Option<**bool**> | Note:  * Field not used with the rtm_activity data source.  | [optional][default to false]
**sampling_percentage** | Option<**i32**> | Note:  * `Range` - From 0 to 100.  * `To use:` [Contact the sales team](https://www.azion.com/en/contact-sales/) to activate this feature in your account.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


