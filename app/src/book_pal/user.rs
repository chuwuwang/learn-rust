use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    name: String,
    active: bool,
}

pub fn json_test() {
    let user = User { id: 1, name: "Alice".into(), active: true, };

    // 转为紧凑 JSON
    let json_str = serde_json::to_string(&user).unwrap();
    tracing::info!("{}", json_str);

    // 转为漂亮的 JSON
    let pretty_json = serde_json::to_string_pretty(&user).unwrap();
    tracing::error!("{}", pretty_json);

    let json_data = r#"{"id":2,"name":"Bob","active":false}"#;
    let user: User = serde_json::from_str(json_data).unwrap();
    tracing::error!("{:?}", user);

    let data: Value = json!(
        {
            "user": { "name": "Alice", "age": 25 },
            "tags": ["rust", "serde"]
        }
    );
    let user_name = data["user"]["name"].as_str().unwrap_or("unknown");
    let first_tag = data["tags"][0].as_str().unwrap_or_default();
    tracing::info!("Name: {}, First tag: {}", user_name, first_tag);

    let dynamic_json = json!(
        {
            "id": 1001,
            "name": "Charlie",
            "roles": ["admin", "user"]
        }
    );
    tracing::info!("{}", dynamic_json.to_string());
}