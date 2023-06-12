/*
 * Credentials API
 *
 * Azion Orchestration
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseWithTotal {
    #[serde(rename = "credentials")]
    pub credentials: Vec<crate::models::Response>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl ResponseWithTotal {
    pub fn new(credentials: Vec<crate::models::Response>, total: i64) -> ResponseWithTotal {
        ResponseWithTotal {
            credentials,
            total,
        }
    }
}


