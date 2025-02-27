# 河南地名查询库

这是一个用于查询河南省地名和行政区划信息的Rust库，提供了与中国民政部地名服务接口的交互功能。

## 功能特点

- **行政区划查询**：根据行政区划代码查询相关行政区划信息，支持查询本级、下级及下下级区划
- **地名搜索**：根据地名名称搜索相关地名信息，支持精确和模糊匹配
- **地名详情查询**：根据地名ID查询地名的详细信息，包括地名的含义、来历、历史沿革等


## 使用示例

### 行政区划查询

```rust
use henan_toponym::{Cli, QueryLevel};
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    // 查询河南省（410000000000）及其下级行政区划
    let result = cli
        .division("410000000000".to_string(), QueryLevel::Child)
        .await;
    
    if let Ok(data) = result {
        println!("河南省: {}", data.name);
        if let Some(children) = data.children {
            for child in children {
                println!("  {}: {}", child.name, child.code);
            }
        }
    }
}
```

### 地名搜索

```rust
use henan_toponym::{Cli, SearchParamsBuilder, SearchType};
use tokio;

#[tokio::main]
async fn main() {
    // 搜索名为"唐庄村"的地名
    let params = SearchParamsBuilder::default()
        .st_name("唐庄村")
        .search_type(SearchType::Exact)
        .page(1)
        .size(100)
        .build()
        .unwrap();

    let records = Cli::search(&params).await;
    
    if let Ok(records) = records {
        println!("找到 {} 条匹配记录", records.len());
        for record in records {
            println!("{}: {}", record.standard_name, record.id);
        }
    }
}
```

### 地名详情查询

```rust
use henan_toponym::Cli;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    // 根据ID查询地名详情
    let result = cli.details("411221000000".to_string()).await;
    
    if let Ok(details) = result {
        println!("地名: {}", details.standard_name);
        println!("地名代码: {}", details.place_code);
        println!("地名含义: {}", details.place_meaning);
        if let Some(origin) = details.place_origin {
            println!("地名来历: {}", origin);
        }
        println!("历史沿革: {}", details.government_history);
    }
}
```

## API文档

### 行政区划查询

行政区划查询服务根据输入的行政区划代码来查询相关行政区划信息。

**接口URL**：`https://dmfw.mca.gov.cn/9095/xzqh/getList`

**请求参数**：
- `year`：年份，默认为最新年版
- `code`：行政区划代码
- `maxLevel`：最大查询深度，最多支持2级深度
  - `0`：仅查询本级
  - `1`：查询本级及下级区划
  - `2`：查询本级、下级及下下级区划

### 地名搜索

地名搜索服务根据输入的地名名称来查询相关地名信息。

**接口URL**：`https://dmfw.mca.gov.cn/9095/stname/listPub`

**请求参数**：
- `stName`：标准名称
- `PlaceTypeCode`：类别代码
- `year`：年份
- `searchType`：匹配方式（精确/模糊）
- `code`：行政区划代码
- `page`：页码
- `size`：每页大小

### 地名详情查询

地名详情服务根据输入的地名ID来查询相关地名详细信息。

**接口URL**：`https://dmfw.mca.gov.cn/9095/stname/detailsPub`

**请求参数**：
- `id`：地名ID
