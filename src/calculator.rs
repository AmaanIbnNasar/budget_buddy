use chrono::prelude::*;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::BalanceResponse;
use crate::PostRequest;

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct MonthlySpend {
    name: String,
    date: DateTime<Utc>,
    amount: f32,
}

pub fn calculate_new_balance(post_request: PostRequest) -> BalanceResponse {
    let PostRequest {
        spend,
        current_balance,
        current_date,
        monthly_spends,
    } = post_request;
    let mut spends_applied: Vec<String> = vec![];

    let new_balance = monthly_spends
        .iter()
        .filter(|spend| spend.date <= current_date)
        .map(|bill| {
            spends_applied.push(bill.name.to_string());
            bill.amount
        })
        .fold(current_balance - spend, |acc, x| acc - x);
    BalanceResponse {
        new_balance,
        spends_applied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_balance() {
        let balance = 100.0;
        let expected_balance = BalanceResponse {
            new_balance: 90.0,
            spends_applied: vec![],
        };

        let fixed_time = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let post_request = PostRequest {
            current_balance: balance,
            current_date: fixed_time,
            spend: 10.0,
            monthly_spends: vec![],
        };
        let actual_balance = calculate_new_balance(post_request);
        assert_eq!(actual_balance, expected_balance);
    }
    #[test]
    fn test_calculate_balance_with_monthly_spends() {
        let balance = 100.0;
        let expected_balance = BalanceResponse {
            new_balance: 80.0,
            spends_applied: vec!["Phone Bill".to_string()],
        };
        let fixed_time = Utc.with_ymd_and_hms(2024, 3, 15, 0, 0, 0).unwrap();
        let phone_bill = MonthlySpend {
            name: String::from("Phone Bill"),
            date: Utc.with_ymd_and_hms(2024, 3, 2, 0, 0, 0).unwrap(),
            amount: 10.0,
        };
        let rent_bill = MonthlySpend {
            name: String::from("Rent"),
            date: Utc.with_ymd_and_hms(2024, 3, 17, 0, 0, 0).unwrap(),
            amount: 10.0,
        };
        let bills = Box::new([phone_bill, rent_bill]);
        let post_request = PostRequest {
            current_balance: balance,
            current_date: fixed_time,
            spend: 10.0,
            monthly_spends: bills.to_vec(),
        };

        let actual_balance = calculate_new_balance(post_request);
        assert_eq!(actual_balance, expected_balance);
    }
}
