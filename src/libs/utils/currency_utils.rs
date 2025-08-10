pub fn format_currency(amount: u64) -> String {
    let mut i = 1;
    let mut formatted_amount = String::new();
    let str_amount = amount.to_string();
    for n in (&str_amount).chars().rev() {
        formatted_amount.push(n);
        if i % 3 == 0 && i != (&str_amount).len() {
            formatted_amount.push('.');
        }
        i += 1;
    }
    formatted_amount.chars().rev().collect()
}
