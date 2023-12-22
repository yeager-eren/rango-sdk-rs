use crate::{
    check::{
        is_approved::{CheckApprovalResponse, IsApprovedRequest},
        status::{StatusRequest, StatusResponse},
    },
    error::SdkErr,
};

impl super::Client {
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
    pub async fn status(&self, request: StatusRequest) -> Result<StatusResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/status", self.config.api_key, qs
        );

        let body: StatusResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
