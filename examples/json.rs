use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    city: String,
    street: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct House {
    price: Option<u32>,
    status: Option<String>,
    address: Option<Address>,
    tags: Option<Vec<String>>,
    owner: Option<String>,
}

/// 递归比较两个 serde_json::Value
fn diff_values(before: &Value, after: &Value) -> Value {
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

fn diff_structs<T>(before: &T, after: &T) -> Value
where
    T: Serialize,
{
    let before_val = serde_json::to_value(before).unwrap();
    let after_val = serde_json::to_value(after).unwrap();
    diff_values(&before_val, &after_val)
}

fn main() {
    let before = House {
        price: None,
        status: None,
        address: None,
        tags: None,
        owner: None,
    };

    let after = House {
        price: Some(1150000),
        status: Some("on_sale".to_string()),
        address: Some(Address {
            city: "Beijing".to_string(), // 改了城市
            street: "Nanjing Road".to_string(),
        }),
        tags: Some(vec!["balcony".to_string()]), // 去掉一个tag
        owner: None,                             // 删除了 owner
    };

    let diff = diff_structs(&before, &after);
    println!("{}", serde_json::to_string_pretty(&diff).unwrap());
}
