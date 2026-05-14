use anyhow::Result;
use crate::client::VaroorClient;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

const PREFIX: &str = "aprefix";

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

pub const DANGEROUS_CHARS: [char; 15] = [
    '"', ' ', '<', '>', '$', '|', '(', ')', '`', ':', ';', '{', '}', '[', ']',
];

pub async fn check_reflection(
    client: &Arc<VaroorClient>,
    url: &str,
) -> Result<HashMap<String, Vec<String>>> {
    let parsed = Url::parse(url)?;
    let query = parsed.query().unwrap_or_default();
    let fragment = parsed.fragment().map(|f| f.to_string());

    if query.is_empty() && fragment.is_none() {
        return Ok(HashMap::new());
    }

    let response = send_with_retry(client, url).await?;

    if response.status().is_redirection() {
        return Ok(HashMap::new());
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !content_type.contains("html") {
        return Ok(HashMap::new());
    }

    let body = response.text().await?;

    let mut param_map: HashMap<String, Vec<String>> = HashMap::new();
    for (key, value) in parsed.query_pairs().into_owned() {
        param_map.entry(key.to_string()).or_default().push(value.to_string());
    }

    let mut reflected: HashMap<String, Vec<String>> = HashMap::new();

    for (key, values) in param_map {
        let mut found_values = Vec::new();
        for value in values {
            found_values.push(value);
        }
        if !found_values.is_empty() {
            reflected.insert(key, found_values);
        }
    }

    if let Some(frag) = fragment {
        if !frag.is_empty() && body.contains(&frag) {
            reflected.insert("#".to_string(), vec![frag]);
        }
    }

    Ok(reflected)
}

pub async fn check_unfiltered_chars(
    client: &Arc<VaroorClient>,
    url: &str,
    param: &str,
) -> Vec<char> {
    let mut found = Vec::new();

    for char in DANGEROUS_CHARS {
        let test_value = format!("{}{}", PREFIX, char);

        let parsed = match Url::parse(url) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let original_values: Vec<String> = parsed
            .query_pairs()
            .into_owned()
            .filter(|(k, _)| k == param)
            .map(|(_, v)| v.to_string())
            .collect();

        if original_values.is_empty() {
            continue;
        }

        for orig_val in original_values {
            let modified_value = format!("{}{}", orig_val, test_value);
            let test_url = set_param_value(url, param, &modified_value);

            if let Ok(response) = send_with_retry(client, &test_url).await {
                let body = response.text().await.unwrap_or_default();

                if body.contains(&modified_value) && body.contains(&char.to_string()) {
                    found.push(char);
                    break;
                }
            }
        }
    }

    found
}

fn set_param_value(url: &str, param: &str, new_value: &str) -> String {
    let mut parsed = match Url::parse(url) {
        Ok(p) => p,
        Err(_) => return url.to_string(),
    };

    let pairs: Vec<(String, String)> = parsed
        .query_pairs()
        .into_owned()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let mut new_pairs: Vec<(String, String)> = Vec::new();
    let mut found = false;

    for (k, v) in pairs {
        if k == param && !found {
            new_pairs.push((k, new_value.to_string()));
            found = true;
        } else {
            new_pairs.push((k, v));
        }
    }

    if !found {
        new_pairs.push((param.to_string(), new_value.to_string()));
    }

    let new_query: String = new_pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    parsed.set_query(Some(&new_query));
    parsed.set_fragment(None);
    parsed.to_string()
}

async fn send_with_retry(
    client: &Arc<VaroorClient>,
    url: &str,
) -> Result<reqwest::Response> {
    let mut last_error = None;

    for attempt in 0..MAX_RETRIES {
        match client.client().get(url).send().await {
            Ok(response) => return Ok(response),
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_RETRIES - 1 {
                    tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * (attempt as u64 + 1))).await;
                }
            }
        }
    }

    Err(anyhow::anyhow!("Request failed after {} retries: {:?}", MAX_RETRIES, last_error))
}