/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IkeProposalAuthenticationAlgorithm {
    /// * `hmac-sha1` - SHA-1 HMAC * `hmac-sha256` - SHA-256 HMAC * `hmac-sha384` - SHA-384 HMAC * `hmac-sha512` - SHA-512 HMAC * `hmac-md5` - MD5 HMAC
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl IkeProposalAuthenticationAlgorithm {
    pub fn new() -> IkeProposalAuthenticationAlgorithm {
        IkeProposalAuthenticationAlgorithm {
            value: None,
            label: None,
        }
    }
}

/// * `hmac-sha1` - SHA-1 HMAC * `hmac-sha256` - SHA-256 HMAC * `hmac-sha384` - SHA-384 HMAC * `hmac-sha512` - SHA-512 HMAC * `hmac-md5` - MD5 HMAC
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "hmac-sha1")]
    Sha1,
    #[serde(rename = "hmac-sha256")]
    Sha256,
    #[serde(rename = "hmac-sha384")]
    Sha384,
    #[serde(rename = "hmac-sha512")]
    Sha512,
    #[serde(rename = "hmac-md5")]
    Md5,
}

impl Default for Value {
    fn default() -> Value {
        Self::Sha1
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "SHA-1 HMAC")]
    Sha1Hmac,
    #[serde(rename = "SHA-256 HMAC")]
    Sha256Hmac,
    #[serde(rename = "SHA-384 HMAC")]
    Sha384Hmac,
    #[serde(rename = "SHA-512 HMAC")]
    Sha512Hmac,
    #[serde(rename = "MD5 HMAC")]
    Md5Hmac,
}

impl Default for Label {
    fn default() -> Label {
        Self::Sha1Hmac
    }
}

