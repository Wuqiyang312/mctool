use std::ffi::{c_char, CStr};
use base64::Engine;
use base64::engine::general_purpose;
use crate::{base_properties, properties, uuid};

/// 通过用户名获取UUID
/// # 参数
/// `username` :string - 需要查询的用户名
/// # 返回值
/// Result<String, Box<dyn std::error::Error>> - 成功时返回UUID的字符串形式，失败时返回错误箱
pub(crate) async fn get_uuid(username: String) -> Result<String, reqwest::Error> {
    // 使用reqwest库向Mojang API发送GET请求，获取指定用户名的UUID
    let body = reqwest::get(format!("https://api.mojang.com/users/profiles/minecraft/{}", username))
        .await?
        .text()
        .await?;
    println!("{}", body);
    // 从响应体中提取UUID并返回
    let uuid: uuid::Uuid = serde_json::from_str(body.as_str()).unwrap();

    Ok(uuid.id.as_str().parse().unwrap())
}


/// 通过UUID获取皮肤和披风的URL
/// # 参数
/// `uuid` :string - 需要查询的用户的UUID
/// # 返回值
/// Result<(String,String)> - 成功时返回一个元组，包含皮肤URL和披风URL，失败时返回错误箱
pub(crate) async fn get_skin_and_cape_url(uuid: String) -> Result<(String, String), reqwest::Error> {
    let body = reqwest::get(format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", uuid))
        .await?
        .text()
        .await?;
    println!("{}", body);
    let properties: properties::Properties = serde_json::from_str(body.as_str()).unwrap();

    let base64_textures = properties.properties.get(0).unwrap().value.as_str();
    let textures = general_purpose::STANDARD
        .decode(base64_textures.as_bytes()).unwrap();

    let c_str = unsafe { CStr::from_ptr(textures.as_ptr() as *const c_char) };
    let str_slice = std::str::from_utf8(c_str.to_bytes()).unwrap();

    println!("{}", str_slice);
    let textures_json: base_properties::BaseProperties = serde_json::from_str(str_slice).unwrap();

    let skin_url = textures_json.textures.skin.url.as_str();
    let cape_url = textures_json.textures.cape.url.as_str();
    Ok((skin_url.to_string(), cape_url.to_string()))
}
