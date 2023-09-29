use regex::Regex;
use serde_json;

pub async fn filter(data: &str, regex: &str, target_field_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(regex)?;
    let json_value: serde_json::Value = serde_json::from_str(data)?;
    if let serde_json::Value::Array(entries) = json_value {
        let matching_entries: Vec<&serde_json::Value> = entries
            .iter()
            .filter(|entry| {
                if let Some(target_field) = entry.get(target_field_name) {
                    if let Some(target_string) = target_field.as_str() {
                        return re.is_match(target_string);
                    }
                }
                false
            })
            .collect();
        let result_json = serde_json::json!(matching_entries);
        return Ok(result_json.to_string());
    }
    return Err("couldn't filter data".into());
}
