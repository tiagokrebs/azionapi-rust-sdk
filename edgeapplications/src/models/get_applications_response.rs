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
pub struct GetApplicationsResponse {
    #[serde(rename = "count")]
    pub count: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "schema_version")]
    pub schema_version: i64,
    #[serde(rename = "links")]
    pub links: Box<crate::models::ApplicationLinks>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::ApplicationsResults>,
}

impl GetApplicationsResponse {
    pub fn new(count: i64, total_pages: i64, schema_version: i64, links: crate::models::ApplicationLinks, results: Vec<crate::models::ApplicationsResults>) -> GetApplicationsResponse {
        GetApplicationsResponse {
            count,
            total_pages,
            schema_version,
            links: Box::new(links),
            results,
        }
    }
}


