/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableExportTemplateRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableExportTemplateRequest {
    #[serde(rename = "content_types", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>.
    #[serde(rename = "template_code", skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// Defaults to <code>text/plain; charset=utf-8</code>
    #[serde(rename = "mime_type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Extension to append to the rendered filename
    #[serde(rename = "file_extension", skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// Download file as attachment
    #[serde(rename = "as_attachment", skip_serializing_if = "Option::is_none")]
    pub as_attachment: Option<bool>,
    /// Remote data source
    #[serde(rename = "data_source", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Option<i32>>,
}

impl PatchedWritableExportTemplateRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new() -> PatchedWritableExportTemplateRequest {
        PatchedWritableExportTemplateRequest {
            content_types: None,
            name: None,
            description: None,
            template_code: None,
            mime_type: None,
            file_extension: None,
            as_attachment: None,
            data_source: None,
        }
    }
}


