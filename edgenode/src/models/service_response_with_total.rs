/*
 * Edgenode API
 *
 * Azion Orchestration
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceResponseWithTotal {
    #[serde(rename = "services")]
    pub services: Vec<crate::models::ServiceResponse>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl ServiceResponseWithTotal {
    pub fn new(services: Vec<crate::models::ServiceResponse>, total: i64) -> ServiceResponseWithTotal {
        ServiceResponseWithTotal {
            services,
            total,
        }
    }
}


