pub struct LoanResult {
    pub total_cost: i64,
    pub monthly_cost: i64,
    pub interest_paid: i64,
}

pub fn calculate_loan(
    loan_amount: f64,
    interest_percentage: f64,
    loan_period_years: f64,
    price_per_term: f64,
) -> LoanResult {
    // Calculate monthly interest rate
    let monthly_interest_rate = interest_percentage / 100.0 / 12.0;

    // Calculate number of monthly payments
    let num_payments = loan_period_years * 12.0;

    // Calculate monthly installment using the annuity formula
    let monthly_installment =
        (loan_amount * monthly_interest_rate * (1.0 + monthly_interest_rate).powf(num_payments)
            / ((1.0 + monthly_interest_rate).powf(num_payments) - 1.0)
            + price_per_term)
            .round();
    let total_cost = monthly_installment * num_payments;

    LoanResult {
        total_cost: total_cost.round() as i64,
        monthly_cost: monthly_installment.round() as i64,
        interest_paid: (total_cost - loan_amount).round() as i64,
    }
}
