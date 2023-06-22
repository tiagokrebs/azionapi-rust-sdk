/*
 * Edge Node API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceResponse {
    #[serde(rename = "bind_id")]
    pub bind_id: i64,
    #[serde(rename = "is_bound")]
    pub is_bound: i64,
    #[serde(rename = "last_editor")]
    pub last_editor: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "service_id")]
    pub service_id: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ServiceResponse {
    pub fn new(bind_id: i64, is_bound: i64, last_editor: String, name: String, service_id: i64, updated_at: String) -> ServiceResponse {
        ServiceResponse {
            bind_id,
            is_bound,
            last_editor,
            name,
            service_id,
            updated_at,
        }
    }
}


