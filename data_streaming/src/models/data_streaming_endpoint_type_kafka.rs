/*
 * Data Streaming - OpenAPI
 *
 * The Data Streaming API allows you to manage your existing data streamings and templates. Data Streaming allows you to feed your stream processing, SIEM, and big data platforms with the event logs from your applications on Azion in real time. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStreamingEndpointTypeKafka {
    #[serde(rename = "endpoint_type", skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<bool>,
    #[serde(rename = "kafka_topic", skip_serializing_if = "Option::is_none")]
    pub kafka_topic: Option<String>,
    #[serde(rename = "bootstrap_servers", skip_serializing_if = "Option::is_none")]
    pub bootstrap_servers: Option<String>,
}

impl DataStreamingEndpointTypeKafka {
    pub fn new() -> DataStreamingEndpointTypeKafka {
        DataStreamingEndpointTypeKafka {
            endpoint_type: None,
            use_tls: None,
            kafka_topic: None,
            bootstrap_servers: None,
        }
    }
}


