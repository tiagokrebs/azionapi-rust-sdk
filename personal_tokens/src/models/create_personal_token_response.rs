/*
 * Personal Tokens - OpenAPI
 *
 * The Personal Tokens API allows you to manage your existing personal tokens. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePersonalTokenResponse {
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<f32>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl CreatePersonalTokenResponse {
    pub fn new() -> CreatePersonalTokenResponse {
        CreatePersonalTokenResponse {
            uuid: None,
            name: None,
            key: None,
            user_id: None,
            created: None,
            expires_at: None,
            description: None,
        }
    }
}


