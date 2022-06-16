pub mod spider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MovieItem {
    pub name: Option<String>,         // 电影名称
    pub url: Option<String>,          // url 地址
    pub image_url: Option<String>,    // 导演
    pub director: Option<String>,     // 预览图片 url 地址
    pub desc: Option<String>,         // 简介
    pub publish_year: Option<String>, // 发布时间
    pub loc: Option<String>,          // 地区
    pub class: Option<String>,        // 分类
    pub from: String,                 // 网站来源
}
