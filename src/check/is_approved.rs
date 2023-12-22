#[derive(Debug)]
pub enum TransactionStatus {
    FAILED,
    RUNNING,
    SUCCESS,
}

#[derive(Debug)]
pub struct CheckApprovalResponse {
    is_approved: bool,
    tx_status: Option<TransactionStatus>,
    required_approved_amount: Option<String>,
    current_approved_amount: Option<String>,
}
