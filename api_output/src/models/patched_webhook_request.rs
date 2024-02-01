/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWebhookRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWebhookRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body.
    #[serde(rename = "payload_url", skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    /// * `GET` - GET * `POST` - POST * `PUT` - PUT * `PATCH` - PATCH * `DELETE` - DELETE
    #[serde(rename = "http_method", skip_serializing_if = "Option::is_none")]
    pub http_method: Option<HttpMethod>,
    /// The complete list of official content types is available <a href=\"https://www.iana.org/assignments/media-types/media-types.xhtml\">here</a>.
    #[serde(rename = "http_content_type", skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below).
    #[serde(rename = "additional_headers", skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>.
    #[serde(rename = "body_template", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Enable SSL certificate verification. Disable with caution!
    #[serde(rename = "ssl_verification", skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults.
    #[serde(rename = "ca_file_path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<Option<String>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
}

impl PatchedWebhookRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWebhookRequest {
        PatchedWebhookRequest {
            name: None,
            description: None,
            payload_url: None,
            http_method: None,
            http_content_type: None,
            additional_headers: None,
            body_template: None,
            secret: None,
            ssl_verification: None,
            ca_file_path: None,
            custom_fields: None,
            tags: None,
        }
    }
}

/// * `GET` - GET * `POST` - POST * `PUT` - PUT * `PATCH` - PATCH * `DELETE` - DELETE
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for HttpMethod {
    fn default() -> HttpMethod {
        Self::Get
    }
}
