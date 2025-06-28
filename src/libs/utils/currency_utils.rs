pub fn format_currency(amount: u64) -> String {
    format!("{}.000 VNĐ", amount / 1000)
}
