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
pub struct ApplicationCacheResponseDetails {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "browser_cache_settings")]
    pub browser_cache_settings: String,
    #[serde(rename = "browser_cache_settings_maximum_ttl")]
    pub browser_cache_settings_maximum_ttl: i64,
    #[serde(rename = "cdn_cache_settings")]
    pub cdn_cache_settings: String,
    #[serde(rename = "cdn_cache_settings_maximum_ttl")]
    pub cdn_cache_settings_maximum_ttl: i64,
    #[serde(rename = "cache_by_query_string")]
    pub cache_by_query_string: String,
    #[serde(rename = "query_string_fields", deserialize_with = "Option::deserialize")]
    pub query_string_fields: Option<Vec<String>>,
    #[serde(rename = "enable_query_string_sort")]
    pub enable_query_string_sort: bool,
    #[serde(rename = "cache_by_cookies")]
    pub cache_by_cookies: String,
    #[serde(rename = "cookie_names", deserialize_with = "Option::deserialize")]
    pub cookie_names: Option<Vec<String>>,
    #[serde(rename = "adaptive_delivery_action", skip_serializing_if = "Option::is_none")]
    pub adaptive_delivery_action: Option<String>,
    #[serde(rename = "device_group", skip_serializing_if = "Option::is_none")]
    pub device_group: Option<Vec<i32>>,
    #[serde(rename = "enable_caching_for_post")]
    pub enable_caching_for_post: bool,
    #[serde(rename = "enable_caching_for_options", skip_serializing_if = "Option::is_none")]
    pub enable_caching_for_options: Option<bool>,
    #[serde(rename = "l2_caching_enabled")]
    pub l2_caching_enabled: bool,
    #[serde(rename = "is_slice_configuration_enabled", skip_serializing_if = "Option::is_none")]
    pub is_slice_configuration_enabled: Option<bool>,
    #[serde(rename = "is_slice_edge_caching_enabled", skip_serializing_if = "Option::is_none")]
    pub is_slice_edge_caching_enabled: Option<bool>,
    #[serde(rename = "is_slice_l2_caching_enabled", skip_serializing_if = "Option::is_none")]
    pub is_slice_l2_caching_enabled: Option<bool>,
    #[serde(rename = "slice_configuration_range", skip_serializing_if = "Option::is_none")]
    pub slice_configuration_range: Option<i64>,
    #[serde(rename = "enable_stale_cache", skip_serializing_if = "Option::is_none")]
    pub enable_stale_cache: Option<bool>,
    #[serde(rename = "l2_region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub l2_region: Option<Option<String>>,
}

impl ApplicationCacheResponseDetails {
    pub fn new(id: i64, name: String, browser_cache_settings: String, browser_cache_settings_maximum_ttl: i64, cdn_cache_settings: String, cdn_cache_settings_maximum_ttl: i64, cache_by_query_string: String, query_string_fields: Option<Vec<String>>, enable_query_string_sort: bool, cache_by_cookies: String, cookie_names: Option<Vec<String>>, enable_caching_for_post: bool, l2_caching_enabled: bool) -> ApplicationCacheResponseDetails {
        ApplicationCacheResponseDetails {
            id,
            name,
            browser_cache_settings,
            browser_cache_settings_maximum_ttl,
            cdn_cache_settings,
            cdn_cache_settings_maximum_ttl,
            cache_by_query_string,
            query_string_fields,
            enable_query_string_sort,
            cache_by_cookies,
            cookie_names,
            adaptive_delivery_action: None,
            device_group: None,
            enable_caching_for_post,
            enable_caching_for_options: None,
            l2_caching_enabled,
            is_slice_configuration_enabled: None,
            is_slice_edge_caching_enabled: None,
            is_slice_l2_caching_enabled: None,
            slice_configuration_range: None,
            enable_stale_cache: None,
            l2_region: None,
        }
    }
}


