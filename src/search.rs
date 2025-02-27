//! # 地名搜索
//! 
//! ## 接口描述
//! 
//! 地名搜索服务根据输入的地名名称来查询相关地名信息
//! 
//! ## 接口网址格式
//! ```text
//! https://dmfw.mca.gov.cn/9095/stname/listPub
//! ```
//! 
//! ## 请求报文结构
//! 
//! 请求方式: GET
//! 
//! | 参数名称 | 是否必须 | 类型 | 默认值 | 描述 (示例) |
//! | :---: | :---: | :---: | :---: | :---: |
//! | stName | 是 | String | | 标准名称 |
//! | PlaceTypeCode | 否 | String |  | 类别代码 |
//! | year | 否 | int |  | 年份 |
//! | searchType | 否 | String | 模糊 | 匹配方式 精确/模糊 |
//! | code | 否 | String |  | 行政区划代码 |
//! | page | 否 | int |  | 页码 |
//! | size | 否 | int |  | 每页大小 |
//! 
//! ## 响应报文结构
//! 
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | records | 返回结果集 | Object[] |
//! | total | 数据总数 | int |
//! 
//! 返回结果集
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | id | 数据ID | String |    
//! | place_code | 地名代码 | String |
//! | standard_name | 标准名称 | String |
//! | ethnic_minorities_writing | 少数民族语书写 | String |
//! | place_type | 地名类别 | String |
//! | place_type_code | 地名类别代码 | String |
//! | province_name | 省级政区名称 | String |
//! | city_name | 市级政区名称 | String |
//! | area_name | 区县级政区名称 | String |
//! | area | 区县级行政代码 | String |
//! | city | 市级行政代码 | String |
//! | province | 省级行政代码 | String |
//! | gdm | 空间坐标信息，GeoJson格式 | Object |  
//! 


use serde::{Deserialize, Serialize};
use crate::details::Geometry;
use derive_builder::Builder;

pub const SEARCH_URL: &str = "https://dmfw.mca.gov.cn/9095/stname/listPub";

/// 地名搜索请求参数
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct SearchParams {
    /// 标准名称
    #[builder(setter(into))]
    #[serde(rename = "stName")]
    st_name: String,
    /// 类别代码
    #[builder(setter(strip_option, into), default)]
    #[serde(rename = "PlaceTypeCode", skip_serializing_if = "Option::is_none")]
    place_type_code: Option<String>,
    /// 年份
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    /// 匹配方式 精确/模糊
    #[builder(setter(strip_option), default)]
    #[serde(rename = "searchType", skip_serializing_if = "Option::is_none")]
    search_type: Option<SearchType>,
    /// 行政区划代码
    #[builder(setter(strip_option, into), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// 页码
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<usize>,
    /// 每页大小
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
}

/// 匹配方式 精确/模糊
#[derive(Debug, Clone, Serialize, Eq, PartialEq, Default)]
pub enum SearchType {
    /// 精确
    #[default]
    #[serde(rename = "精确")]
    Exact,
    /// 模糊
    #[serde(rename = "模糊")]
    Fuzzy,
}

/// 返回结果集
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    /// 返回结果集
    pub records: Vec<Record>,
    /// 数据总数
    pub total: usize,
}

/// 返回结果集
#[derive(Debug, Clone, Deserialize)]
pub struct Record {
    /// 数据ID
    pub id: String,
    /// 地名代码
    pub place_code: String,
    /// 标准名称
    pub standard_name: String,
    /// 标准名称拼音
    pub roman_alphabet_spelling: String,
    /// 少数民族语书写
    pub ethnic_minorities_writing: String,
    /// 地名类别
    pub place_type: String,
    /// 地名类别代码
    pub place_type_code: Option<String>,
    /// 省级政区名称
    pub province_name: Option<String>,
    /// 市级政区名称
    pub city_name: Option<String>,
    /// 区县级政区名称
    pub area_name: Option<String>,
    /// 区县级行政代码
    pub area: Option<String>,
    /// 市级行政代码
    pub city: Option<String>,
    /// 省级行政代码
    pub province: Option<String>,
    /// 空间坐标信息，GeoJson格式
    pub gdm: Option<Geometry>,
}
