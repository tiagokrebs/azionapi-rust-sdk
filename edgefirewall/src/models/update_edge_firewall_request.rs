/*
 * Edge Firewall API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateEdgeFirewallRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<i64>>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "edge_functions_enabled", skip_serializing_if = "Option::is_none")]
    pub edge_functions_enabled: Option<bool>,
    #[serde(rename = "network_protection_enabled", skip_serializing_if = "Option::is_none")]
    pub network_protection_enabled: Option<bool>,
    #[serde(rename = "waf_enabled", skip_serializing_if = "Option::is_none")]
    pub waf_enabled: Option<bool>,
}

impl UpdateEdgeFirewallRequest {
    pub fn new() -> UpdateEdgeFirewallRequest {
        UpdateEdgeFirewallRequest {
            name: None,
            domains: None,
            is_active: None,
            edge_functions_enabled: None,
            network_protection_enabled: None,
            waf_enabled: None,
        }
    }
}


