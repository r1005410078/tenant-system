use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Community {
    pub id: Option<String>,
    // 小区名称
    pub name: Option<String>,
    // 小区地址
    pub address: Option<String>,
    // 城市
    pub city: Option<String>,
    // 建成年份
    pub year_built: Option<DateTimeUtc>,
    // 小区描述
    pub description: Option<String>,
    // 小区图片
    pub images: Option<Vec<String>>,
    // 小区类型
    pub typecode: Option<String>,
    // 所属行政区（如“朝阳区”）
    pub district: Option<String>,
    // 所属行政区划代码（如“110105”，代表朝阳区）
    pub adcode: Option<String>,
    // 物业
    pub property_management_company: Option<String>,
    // 备注
    pub remark: Option<String>,
    // 位置
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

impl Community {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_none() {
            return Err(anyhow::anyhow!("name is required"));
        }

        if self.address.is_none() {
            return Err(anyhow::anyhow!("address is required"));
        }

        if self.city.is_none() {
            return Err(anyhow::anyhow!("city is required"));
        }

        if self.typecode.is_none() {
            return Err(anyhow::anyhow!("typecode is required"));
        }

        if self.lat.is_none() {
            return Err(anyhow::anyhow!("lat is required"));
        }

        if self.lng.is_none() {
            return Err(anyhow::anyhow!("lng is required"));
        }

        if self.adcode.is_none() {
            return Err(anyhow::anyhow!("adcode is required"));
        }

        if self.district.is_none() {
            return Err(anyhow::anyhow!("district is required"));
        }

        Ok(())
    }
}
