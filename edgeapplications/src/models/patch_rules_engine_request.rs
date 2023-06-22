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
pub struct PatchRulesEngineRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "criteria", skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<Vec<crate::models::RulesEngineCriteria>>>,
    #[serde(rename = "behaviors", skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<crate::models::RulesEngineBehavior>>,
}

impl PatchRulesEngineRequest {
    pub fn new() -> PatchRulesEngineRequest {
        PatchRulesEngineRequest {
            name: None,
            criteria: None,
            behaviors: None,
        }
    }
}


