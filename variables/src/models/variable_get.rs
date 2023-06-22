/*
 * Variables API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VariableGet {
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    #[serde(rename = "key")]
    pub key: String,
    /// Given the *incoming* primitive data, return the value for this field that should be validated and transformed to a native value.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "secret")]
    pub secret: bool,
    #[serde(rename = "last_editor")]
    pub last_editor: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl VariableGet {
    pub fn new(uuid: uuid::Uuid, key: String, value: String, secret: bool, last_editor: String, created_at: String, updated_at: String) -> VariableGet {
        VariableGet {
            uuid,
            key,
            value,
            secret,
            last_editor,
            created_at,
            updated_at,
        }
    }
}


