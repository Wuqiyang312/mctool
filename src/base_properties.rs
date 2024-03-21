// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BaseProperties {
    #[serde(rename = "profileName")]
    profile_name: String,

    #[serde(rename = "textures")]
    pub(crate) textures: Textures,

    #[serde(rename = "profileId")]
    profile_id: String,

    #[serde(rename = "timestamp")]
    timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Textures {
    #[serde(rename = "SKIN")]
    pub(crate) skin: Cape,

    #[serde(rename = "CAPE")]
    pub(crate) cape: Cape,
}

#[derive(Serialize, Deserialize)]
pub struct Cape {
    #[serde(rename = "url")]
    pub(crate) url: String,
}
