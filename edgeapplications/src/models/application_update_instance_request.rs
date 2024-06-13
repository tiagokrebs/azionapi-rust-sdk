/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpdateInstanceRequest {
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    #[serde(rename = "edge_function_id", deserialize_with = "Option::deserialize")]
    pub edge_function_id: Option<i64>,
    #[serde(rename = "args", deserialize_with = "Option::deserialize")]
    pub args: Option<serde_json::Value>,
}

impl ApplicationUpdateInstanceRequest {
    pub fn new(name: Option<String>, edge_function_id: Option<i64>, args: Option<serde_json::Value>) -> ApplicationUpdateInstanceRequest {
        ApplicationUpdateInstanceRequest {
            name,
            edge_function_id,
            args,
        }
    }
}

