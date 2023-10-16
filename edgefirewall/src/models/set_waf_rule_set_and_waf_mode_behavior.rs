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
pub struct SetWafRuleSetAndWafModeBehavior {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(rename = "argument", skip_serializing_if = "Option::is_none")]
    pub argument: Option<Box<crate::models::SetWafRuleSetAndWafModeBehaviorArgument>>,
}

impl SetWafRuleSetAndWafModeBehavior {
    pub fn new() -> SetWafRuleSetAndWafModeBehavior {
        SetWafRuleSetAndWafModeBehavior {
            name: None,
            argument: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "set_waf_ruleset_and_waf_mode")]
    SetWafRulesetAndWafMode,
}

impl Default for Name {
    fn default() -> Name {
        Self::SetWafRulesetAndWafMode
    }
}

