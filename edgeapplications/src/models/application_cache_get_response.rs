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
pub struct ApplicationCacheGetResponse {
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
    #[serde(rename = "links")]
    pub links: Box<models::ApplicationLinks>,
    #[serde(rename = "results")]
    pub results: Vec<models::ApplicationCacheResults>,
}

impl ApplicationCacheGetResponse {
    pub fn new(count: i64, total_pages: i64, schema_version: i64, links: models::ApplicationLinks, results: Vec<models::ApplicationCacheResults>) -> ApplicationCacheGetResponse {
        ApplicationCacheGetResponse {
            count,
            total_pages,
            schema_version,
            links: Box::new(links),
            results,
        }
    }
}

