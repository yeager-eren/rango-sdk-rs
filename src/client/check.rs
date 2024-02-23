use crate::{
    check::{
        balance::{BalanceRequest, BalanceResponse},
        is_approved::{CheckApprovalResponse, IsApprovedRequest},
        status::{StatusRequest, StatusResponse},
    },
    error::SdkErr,
};

impl super::Client {
    /// In some blockchains (like evm), before sending the swap transaction, user is needing to give a approval to the conttact. This method will track and check the approval transcation.
    pub async fn is_approved(
        &self,
        request: IsApprovedRequest,
    ) -> Result<CheckApprovalResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/is-approved", self.config.api_key, qs
        );

        let body: CheckApprovalResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
    /// After signing the transaction by the user and receiving transaction hash,
    /// you could periodically call Rango check-status API to track the transaction status.
    pub async fn status(&self, request: StatusRequest) -> Result<StatusResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/status", self.config.api_key, qs
        );

        let body: StatusResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// Getting wallet's balance by giving blockchain and address.
    pub async fn balance(&self, request: BalanceRequest) -> Result<BalanceResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/balance", self.config.api_key, qs
        );

        let body: BalanceResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
