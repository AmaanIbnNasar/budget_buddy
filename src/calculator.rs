use chrono::prelude::*;
use chrono::Utc;

struct MonthlySpend {
    name: String,
    date: DateTime<Utc>,
    amount: f32,
}

pub fn calculate_balance(
    current_date: DateTime<Utc>,
    current_balance: f32,
    spend: f32,
    monthly_spends: Box<[MonthlySpend]>,
) -> f32 {
    monthly_spends
        .iter()
        .filter(|spend| spend.date <= current_date)
        .map(|bill| bill.amount)
        .fold(current_balance - spend, |acc, x| acc - x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_balance() {
        let balance = 100.0;
        let expected_balance = 90.0;

        let fixed_time = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let actual_balance = calculate_balance(fixed_time, balance, 10.0, Box::new([]));
        println!("{}", actual_balance);
        assert_eq!(actual_balance, expected_balance);
    }
    #[test]
    fn test_calculate_balance_with_monthly_spends() {
        let balance = 100.0;
        let expected_balance = 80.0;
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

        let actual_balance = calculate_balance(fixed_time, balance, 10.0, bills);
        assert_eq!(actual_balance, expected_balance);
    }
}
