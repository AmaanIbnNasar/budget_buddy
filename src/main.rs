pub mod calculator;
use calculator::calculate_balance;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(PartialEq, Debug, Deserialize)]
pub struct RequestBody {
    current_balance: f32,
    current_date: DateTime<Utc>,
    spend: f32,
}

fn parse_body(event: Value) -> Result<RequestBody, String> {
    let body = match event.get("body") {
        Some(body) => serde_json::from_value(body.clone()).unwrap(),
        None => Err("No body found")?,
    };
    Ok(body)
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::json;

    #[test]
    fn test_parse_body() {
        let event = json!({
            "body": {
                "current_balance": 1000,
                "current_date": "2024-01-01T0:00:00.000000000Z",
                "spend": 100
            }
        });
        let expected_response = RequestBody {
            current_balance: 1000.0,
            current_date: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            spend: 100.0,
        };
        assert_eq!(parse_body(event).ok(), Some(expected_response))
    }

    #[test]
    fn test_parse_body_fails_with_no_body() {
        assert_eq!(
            parse_body(json!({})).err(),
            Some(String::from("No body found"))
        )
    }
}
