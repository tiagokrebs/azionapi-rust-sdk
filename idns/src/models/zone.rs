/*
 * Intelligent DNS
 *
 * Azion Intelligent DNS API
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Zone {
    /// Hosted zone id
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Hosted zone name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Hosted zone domain
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// If hosted zone is active
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "retry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub retry: Option<Option<i32>>,
    #[serde(rename = "nx_ttl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nx_ttl: Option<Option<i32>>,
    #[serde(rename = "soa_ttl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub soa_ttl: Option<Option<i32>>,
    #[serde(rename = "refresh", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<Option<i32>>,
    #[serde(rename = "expiry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<Option<i32>>,
    /// List of nameservers
    #[serde(rename = "nameservers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Option<Vec<String>>>,
}

impl Zone {
    pub fn new() -> Zone {
        Zone {
            id: None,
            name: None,
            domain: None,
            is_active: None,
            retry: None,
            nx_ttl: None,
            soa_ttl: None,
            refresh: None,
            expiry: None,
            nameservers: None,
        }
    }
}


