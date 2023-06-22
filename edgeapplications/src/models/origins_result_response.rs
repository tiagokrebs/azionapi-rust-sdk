/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginsResultResponse {
    #[serde(rename = "origin_id")]
    pub origin_id: i64,
    #[serde(rename = "origin_key")]
    pub origin_key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "origin_type")]
    pub origin_type: String,
    #[serde(rename = "addresses")]
    pub addresses: Vec<crate::models::OriginsResultResponseAddresses>,
    #[serde(rename = "origin_protocol_policy")]
    pub origin_protocol_policy: String,
    #[serde(rename = "is_origin_redirection_enabled")]
    pub is_origin_redirection_enabled: bool,
    #[serde(rename = "host_header")]
    pub host_header: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "origin_path")]
    pub origin_path: String,
    #[serde(rename = "connection_timeout")]
    pub connection_timeout: i64,
    #[serde(rename = "timeout_between_bytes")]
    pub timeout_between_bytes: i64,
    #[serde(rename = "hmac_authentication")]
    pub hmac_authentication: bool,
    #[serde(rename = "hmac_region_name")]
    pub hmac_region_name: String,
    #[serde(rename = "hmac_access_key")]
    pub hmac_access_key: String,
    #[serde(rename = "hmac_secret_key")]
    pub hmac_secret_key: String,
}

impl OriginsResultResponse {
    pub fn new(origin_id: i64, origin_key: String, name: String, origin_type: String, addresses: Vec<crate::models::OriginsResultResponseAddresses>, origin_protocol_policy: String, is_origin_redirection_enabled: bool, host_header: String, method: String, origin_path: String, connection_timeout: i64, timeout_between_bytes: i64, hmac_authentication: bool, hmac_region_name: String, hmac_access_key: String, hmac_secret_key: String) -> OriginsResultResponse {
        OriginsResultResponse {
            origin_id,
            origin_key,
            name,
            origin_type,
            addresses,
            origin_protocol_policy,
            is_origin_redirection_enabled,
            host_header,
            method,
            origin_path,
            connection_timeout,
            timeout_between_bytes,
            hmac_authentication,
            hmac_region_name,
            hmac_access_key,
            hmac_secret_key,
        }
    }
}


