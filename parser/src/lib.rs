pub mod csv_format;
pub mod errors;

#[derive(Debug)]
pub enum Status {}

#[derive(Debug)]
pub struct Transaction {
    tx_id: u64,
    tx_type: String,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}
pub struct Parser {}

#[cfg(test)]
mod tests {
    use super::*;
}
