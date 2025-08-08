use serde::Serialize;
use serde_json::{Value, json};

/// 递归比较两个 serde_json::Value
pub fn diff_values(before: &Value, after: &Value) -> Value {
    match (before, after) {
        // 对象递归比较
        (Value::Object(b_map), Value::Object(a_map)) => {
            let mut changes = serde_json::Map::new();
            for (key, b_val) in b_map {
                if let Some(a_val) = a_map.get(key) {
                    let sub_diff = diff_values(b_val, a_val);
                    if !sub_diff.is_null() {
                        changes.insert(key.clone(), sub_diff);
                    }
                }
            }
            if changes.is_empty() {
                Value::Null
            } else {
                Value::Object(changes)
            }
        }
        // 非对象，直接比较
        _ => {
            if before != after {
                json!({ "before": before, "after": after })
            } else {
                Value::Null
            }
        }
    }
}

pub fn diff_structs<T>(before: &T, after: &T) -> Value
where
    T: Serialize,
{
    let before_val = serde_json::to_value(before).unwrap();
    let after_val = serde_json::to_value(after).unwrap();
    diff_values(&before_val, &after_val)
}

#[cfg(test)]
mod tests {
    use crate::diff_values::diff_structs;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Address {
        city: String,
        street: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct House {
        price: u32,
        status: String,
        address: Address,
        tags: Vec<String>,
        owner: Option<String>,
    }

    #[test]
    fn diff_structs_test() {
        let before = House {
            price: 1200000,
            status: "on_sale".to_string(),
            address: Address {
                city: "Shanghai".to_string(),
                street: "Nanjing Road".to_string(),
            },
            tags: vec!["balcony".to_string(), "sea_view".to_string()],
            owner: Some("Alice".to_string()),
        };

        let after = House {
            price: 1150000,
            status: "on_sale".to_string(),
            address: Address {
                city: "Beijing".to_string(), // 改了城市
                street: "Nanjing Road".to_string(),
            },
            tags: vec!["balcony".to_string()], // 去掉一个tag
            owner: None,                       // 删除了 owner
        };

        let diff = diff_structs(&before, &after);
        println!("{}", serde_json::to_string_pretty(&diff).unwrap());
    }
}
