// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Uuid {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "id")]
    pub(crate) id: String,
}
