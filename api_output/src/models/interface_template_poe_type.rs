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
pub struct InterfaceTemplatePoeType {
    /// * `type1-ieee802.3af` - 802.3af (Type 1) * `type2-ieee802.3at` - 802.3at (Type 2) * `type3-ieee802.3bt` - 802.3bt (Type 3) * `type4-ieee802.3bt` - 802.3bt (Type 4) * `passive-24v-2pair` - Passive 24V (2-pair) * `passive-24v-4pair` - Passive 24V (4-pair) * `passive-48v-2pair` - Passive 48V (2-pair) * `passive-48v-4pair` - Passive 48V (4-pair)
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl InterfaceTemplatePoeType {
    pub fn new() -> InterfaceTemplatePoeType {
        InterfaceTemplatePoeType {
            value: None,
            label: None,
        }
    }
}

/// * `type1-ieee802.3af` - 802.3af (Type 1) * `type2-ieee802.3at` - 802.3at (Type 2) * `type3-ieee802.3bt` - 802.3bt (Type 3) * `type4-ieee802.3bt` - 802.3bt (Type 4) * `passive-24v-2pair` - Passive 24V (2-pair) * `passive-24v-4pair` - Passive 24V (4-pair) * `passive-48v-2pair` - Passive 48V (2-pair) * `passive-48v-4pair` - Passive 48V (4-pair)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "type1-ieee802.3af")]
    Type1Ieee802Period3af,
    #[serde(rename = "type2-ieee802.3at")]
    Type2Ieee802Period3at,
    #[serde(rename = "type3-ieee802.3bt")]
    Type3Ieee802Period3bt,
    #[serde(rename = "type4-ieee802.3bt")]
    Type4Ieee802Period3bt,
    #[serde(rename = "passive-24v-2pair")]
    Passive24v2pair,
    #[serde(rename = "passive-24v-4pair")]
    Passive24v4pair,
    #[serde(rename = "passive-48v-2pair")]
    Passive48v2pair,
    #[serde(rename = "passive-48v-4pair")]
    Passive48v4pair,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Type1Ieee802Period3af
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "802.3af (Type 1)")]
    Variant802Period3afLeftParenthesisType1RightParenthesis,
    #[serde(rename = "802.3at (Type 2)")]
    Variant802Period3atLeftParenthesisType2RightParenthesis,
    #[serde(rename = "802.3bt (Type 3)")]
    Variant802Period3btLeftParenthesisType3RightParenthesis,
    #[serde(rename = "802.3bt (Type 4)")]
    Variant802Period3btLeftParenthesisType4RightParenthesis,
    #[serde(rename = "Passive 24V (2-pair)")]
    Passive24VLeftParenthesis2PairRightParenthesis,
    #[serde(rename = "Passive 24V (4-pair)")]
    Passive24VLeftParenthesis4PairRightParenthesis,
    #[serde(rename = "Passive 48V (2-pair)")]
    Passive48VLeftParenthesis2PairRightParenthesis,
    #[serde(rename = "Passive 48V (4-pair)")]
    Passive48VLeftParenthesis4PairRightParenthesis,
}

impl Default for Label {
    fn default() -> Label {
        Self::Variant802Period3afLeftParenthesisType1RightParenthesis
    }
}

