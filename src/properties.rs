// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "profileActions")]
    profile_actions: Vec<Option<serde_json::Value>>,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "properties")]
    pub(crate) properties: Vec<Property>,
}

#[derive(Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "value")]
    pub(crate) value: String,
}
