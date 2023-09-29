mod helpers;
use serde_json;
use tokio_stream::{self, StreamExt};
use crate::configuration::Config;

use self::helpers::get_m42_object_from_summary;
mod filters;

pub async fn authoritate(config: &mut Config) -> Result<(), Box<dyn std::error::Error>>{
    let auth_url = "ApiToken/GenerateAccessTokenFromApiToken";
 
    let headers = helpers::default_headers(&config.auth_token).await;
    let url = format!("{}{}", config.api_endpoint, auth_url);
    let response = helpers::make_the_post(headers, &url).await?;
    let body = response.text().await?;
    let json_body = &body;

    let parsed: serde_json::Value = serde_json::from_str(&json_body).expect("failed to parse RawToken");
    let ret = parsed["RawToken"].as_str().expect("no RawToken field or not a string");
    config.set_raw_token(ret);
    Ok(())
}

pub async fn get_people(config: &Config, filter: &str, field: &str) -> Result<String, Box<dyn std::error::Error>> {
    let list_url = "widgetList/getData/924d38c0-925d-c3ce-1ec6-08d46af72bea?orderBy=Status";
    let raw_token = match &config.raw_token {
        Some(token) => token,
        None => {
            // Handle the case where raw_token is None, e.g., return an error or provide a default value
            return Err("no raw_token".into());
        }
    };

    // Fetch json string of all users
    let headers = helpers::default_headers(&raw_token).await;
    let url = format!("{}{}", config.api_endpoint, list_url);
    let response = helpers::make_the_get(headers, &url).await?;
    let body = response.text().await?;

    // Filter the ones we need with regex
    let filtered_list_raw = filters::filter(&body, filter, field).await?;
    let filtered_list = serde_json::json!(filtered_list_raw);

    // Now that we have a filtered list, we iterate over that and
    // expand all the elements into m42_objects
    if let serde_json::Value::Array(list) = filtered_list {
        // Convert our json array into an async stream
        let stream = tokio_stream::iter(list);
        // Compact the config and entries themselves into a single struct
        let mut stream = stream.map(|entry| 
            {
                let boxed_entry = Box::new(entry);
                helpers::Summary{config: config, summary: boxed_entry,}
            });
        // Convert all of the items to m42_objects at once
        let mut stream = stream.then(get_m42_object_from_summary);
        
        // Next I'll have to convert that async stream of serde_json::Value objects back into a regular String I
        // can return

        //let result_json = serde_json::json!(converted_summaries);
        //Ok(result_json.to_string())
    }

    Err("Damn".into())
}
