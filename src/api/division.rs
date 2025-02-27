//! # 行政区划查询
//!
//! ## 接口描述
//!
//! 行政区划查询服务根据输入的行政区划代码来查询相关行政区划信息
//!
//! ## 接口网址格式
//! ```text
//! https://dmfw.mca.gov.cn/9095/xzqh/getList
//! ```
//!
//! ## 请求报文结构
//!
//! 请求方式: GET
//! | 参数名称 | 是否必须 | 类型 | 默认值 | 描述 (示例) |
//! | --- | --- | --- | --- | --- |
//! | year | 否 | int |  | 年份，默认为最新年版  |
//! | code | 否 | string |  | 行政区划代码  |
//! | maxLevel | 是 | int| | 最大查询深度，最多支持2级深度，0表示仅查询本级，1表示本级及下级区划，2表示查询本级、下级及下下级区划 |
//!
//! ## 响应报文结构
//!
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | data | 返回结果对象 | Object |
//! | message | 服务信息 | String |
//! | status | 服务状态码 | int |
//! | total | 总条数 | int |
//! | tag | 标签 | String |
//!
//! 返回结果
//! | 参数名称 | 描述 | 类型 |
//! | :---: | :---: | :---: |
//! | code | 行政区划代码 | String |
//! | name | 标准名称 | String |
//! | level | 行政区划级别 | int |
//! | type | 行政区划单位 | String |
//! | children | 下级区划 | Object[] |

use serde::{Deserialize, Serialize};

/// 行政区划查询接口
pub const DIVISION_QUERY_URL: &str = "https://dmfw.mca.gov.cn/9095/xzqh/getList";

/// 行政区划搜索请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DivisionQueryParams {
    /// 年份，默认查询最新年版
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    /// 行政区划代码
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// 最大查询深度，最多支持2级深度，0表示仅查询本级，1表示本级及下级区划，2表示查询本级、下级及下下级区划
    #[serde(rename = "maxLevel")]
    max_level: QueryLevel,
}

impl Default for DivisionQueryParams {
    fn default() -> Self {
        Self {
            year: None,
            code: None,
            max_level: QueryLevel::Current,
        }
    }
}

impl DivisionQueryParams {
    pub fn new(year: i32, code: String, max_level: QueryLevel) -> Self {
        Self {
            year: Some(year),
            code: Some(code),
            max_level,
        }
    }

    pub fn latest(code: &str, max_level: QueryLevel) -> Self {
        Self {
            year: None,
            code: Some(code.to_string()),
            max_level,
        }
    }
}

/// 查询深度
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryLevel {
    /// 仅查询本级
    #[serde(rename = "0")]
    Current,
    /// 查询本级及下级区划
    #[serde(rename = "1")]
    Child,
    /// 查询本级、下级及下下级区划
    #[serde(rename = "2")]
    GrandChild,
}

/// 行政区划搜索响应
#[derive(Debug, Clone, Deserialize)]
pub struct DivisionQueryResponse {
    /// 返回结果对象
    pub data: DivisonQueryResult,
    /// 服务信息
    pub message: Option<String>,
    /// 服务状态码
    pub status: i32,
    /// 总条数
    pub total: usize,
    /// 标签
    pub tag: Option<String>,
}

/// 行政区划查询结果
#[derive(Debug, Clone, Deserialize)]
pub struct DivisonQueryResult {
    /// 行政区划代码
    pub code: String,
    /// 标准名称
    pub name: String,
    /// 行政区划级别
    pub level: i32,
    /// 行政区划单位
    pub r#type: String,
    /// 下级区划
    pub children: Vec<DivisonQueryResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_query_request() {
        let request = DivisionQueryParams::new(2024, "110101".to_string(), QueryLevel::Child);
        let json_str = r#"{"year":2024,"code":"110101","maxLevel":"1"}"#;
        assert_eq!(serde_json::to_string(&request).unwrap(), json_str);
    }
}
