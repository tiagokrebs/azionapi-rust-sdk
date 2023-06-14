/*
 * Variables
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VariableCreate {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
}

impl VariableCreate {
    pub fn new(key: String, value: String) -> VariableCreate {
        VariableCreate {
            key,
            value,
            secret: None,
        }
    }
}


