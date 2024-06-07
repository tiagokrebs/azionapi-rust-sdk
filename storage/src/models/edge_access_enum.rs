/*
 * Object Storage
 *
 * REST API OpenAPI documentation for the Object Storage
 *
 * The version of the OpenAPI document: 1.0.0 (v1)
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeAccessEnum {
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "read_write")]
    ReadWrite,
    #[serde(rename = "restricted")]
    Restricted,

}

impl std::fmt::Display for EdgeAccessEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ReadOnly => write!(f, "read_only"),
            Self::ReadWrite => write!(f, "read_write"),
            Self::Restricted => write!(f, "restricted"),
        }
    }
}

impl Default for EdgeAccessEnum {
    fn default() -> EdgeAccessEnum {
        Self::ReadOnly
    }
}

