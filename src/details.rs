//! # 地名详情
//!
//! ## 接口描述
//!
//! 地名详情服务根据输入的地名ID来查询相关地名详细信息
//!
//! ## 接口网址格式
//! ```text
//! https://dmfw.mca.gov.cn/9095/stname/detailsPub
//! ```
//!
//! ## 请求报文结构
//!
//! 请求方式: POST
//!
//! | 参数名称 | 是否必须 | 类型 | 默认值 | 描述 (示例) |
//! | :---: | :---: | :---: | :---: | :---: |
//! | id | 是 | String | | 地名 ID |
//!
//! ## 响应报文结构
//!
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | area_name | 所在区县名称 | String |
//! | city_name | 所在地市名称 | String |
//! | old_name | 历史地名 | String |
//! | ethnic_minorities_writing | 少数民族语书写 | String |
//! | gdm | 空间坐标信息，GeoJson 格式 | Object |
//! | government_history | 政区的历史沿革 | String |
//! | id | 数据 ID | String |
//! | place_code |  地名代码 | String |
//! | place_meaning | 地名的含义 | String |
//! | place_origin | 地名的来历 | String |
//! | place_type | 地名类别 | String |
//! | place_type_code | 地名类别代码 | String |
//! | province_name | 省级政区名称 | String |
//! | roman_alphabet_spelling | 罗马字母拼写 | String |
//! | standard_name | 标准名称 | String |
//! | area | 区县级行政代码 | String |
//! | city | 市级行政代码 | String |
//! | province | 省级行政代码 | String |
//!
//! gdm
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | type | 类型 | String |
//! | coordinates | 坐标 | Object |

use serde::{Deserialize, Serialize};

pub const DETAILS_QUERY_URL: &str = "https://dmfw.mca.gov.cn/9095/stname/detailsPub";

/// 地名查询请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DetailsQueryParams {
    /// 地名 ID
    id: String,
}

impl DetailsQueryParams {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// 地名查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct DetailsQueryResponse {
    /// 所在区县名称
    pub area_name: Option<String>,
    /// 所在地市名称
    pub city_name: Option<String>,
    /// 历史地名
    pub old_name: Option<String>,
    /// 少数民族语书写
    pub ethnic_minorities_writing: String,
    /// 空间坐标信息，GeoJson 格式
    pub gdm: Geometry,
    /// 政区的历史沿革
    pub government_history: Option<String>,
    /// 数据 ID
    pub id: String,
    /// 地名代码
    pub place_code: String,
    /// 地名的含义
    pub place_meaning: String,
    /// 地名的来历
    pub place_origin: String,
    /// 地名类别
    pub place_type: String,
    /// 地名类别代码
    pub place_type_code: String,
    /// 省级政区名称
    pub province_name: String,
    /// 罗马字母拼写
    pub roman_alphabet_spelling: String,
    /// 标准名称
    pub standard_name: String,
    /// 区县级行政代码
    pub area: Option<String>,
    /// 市级行政代码
    pub city: Option<String>,
    /// 省级行政代码
    pub province: String,
}

/// 空间坐标信息，GeoJson 格式
#[derive(Debug, Clone, Deserialize)]
pub struct Geometry {
    /// 类型
    pub r#type: String,
    /// 坐标
    pub coordinates: Vec<Vec<f64>>,
}
