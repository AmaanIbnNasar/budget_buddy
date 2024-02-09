pub mod calculator;
use calculator::{calculate_new_balance, MonthlySpend};
use chrono::{DateTime, Utc};
use lambda_http::{http::StatusCode, run, Error, IntoResponse, Request, Response};
use lambda_runtime::service_fn;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct PostRequest {
    current_balance: f32,
    current_date: DateTime<Utc>,
    spend: f32,
    monthly_spends: Vec<MonthlySpend>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct BalanceResponse {
    new_balance: f32,
    spends_applied: Vec<String>,
}

fn parse_body(event: Request) -> Result<PostRequest, String> {
    match event.body() {
        lambda_http::Body::Empty => Err("No body")?,
        lambda_http::Body::Text(body) => serde_json::from_str(body).map_err(|err| err.to_string()),
        lambda_http::Body::Binary(_) => Err("Weird body")?,
    }
}

async fn lambda_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let parsed_body = match parse_body(event).and_then(|request| Ok(calculate_new_balance(request)))
    {
        Ok(foo) => serde_json::to_string(&foo).unwrap(),
        Err(err) => err.to_string(),
    };
    let resp: Response<String> = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(parsed_body)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(lambda_handler)).await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use lambda_http::Request;
    use serde_json::json;

    #[test]
    fn test_parse_body() {
        let event = json!({
                "current_balance": 1000,
                "current_date": "2024-01-01T0:00:00.000000000Z",
                "spend": 100,
                "monthly_spends": [
                ]
        })
        .to_string();
        let new_event = Request::new(lambda_http::Body::Text(event));
        let expected_response = PostRequest {
            current_balance: 1000.0,
            current_date: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            spend: 100.0,
            monthly_spends: vec![],
        };
        assert_eq!(parse_body(new_event).unwrap(), expected_response)
    }

    #[test]
    fn test_parse_body_fails_with_no_body() {
        assert_eq!(
            parse_body(Request::default()).unwrap_err(),
            String::from("No body")
        )
    }
}
