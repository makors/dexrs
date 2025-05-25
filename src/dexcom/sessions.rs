use super::{client::DexcomClient, consts, error::DexcomApiError};
use serde_json::json;
use uuid::Uuid;

impl DexcomClient {
    #[doc = "Creates a new Dexcom Share API session"]
    pub(crate) fn create_session(&mut self) -> Result<(), DexcomApiError> {
        // get account id
        if self.account_id.is_none() {
            self.account_id = Some(self.get_account_id()?);
        }

        let account_id = self.account_id.as_ref().unwrap().as_str();

        // validate account id
        if self.account_id.is_none()
            || Uuid::parse_str(account_id).is_err()
            || self.account_id.as_deref() == Some(consts::DEXCOM_NULL_UUID)
        {
            return Err(DexcomApiError::InvalidAccountId);
        }

        // get session id
        self.session_id = Some(self.get_session_id()?);

        // validate session id
        let session_id = self.session_id.as_ref().unwrap().as_str();

        if self.session_id.is_none()
            || Uuid::parse_str(session_id).is_err()
            || self.session_id.as_deref() == Some(consts::DEXCOM_NULL_UUID)
        {
            return Err(DexcomApiError::InvalidSessionId);
        }

        Ok(())
    }

    #[doc = "Gets Dexcom Share API account ID"]
    pub(crate) fn get_account_id(&mut self) -> Result<String, DexcomApiError> {
        let account_response = self.post(
            consts::DEXCOM_AUTH_ENDPOINT,
            json!({
                "applicationId": consts::DEXCOM_APP_ID,
                "accountName": self.username,
                "password": self.password,
            }),
            Vec::new(),
        );

        match account_response {
            Ok(r) => Ok(r.text().unwrap().replace("\"", "")),
            Err(e) => Err(DexcomApiError::LoginError(e.to_string())),
        }
    }

    #[doc = "Fetches a Dexcom Share API session ID"]
    pub(crate) fn get_session_id(&mut self) -> Result<String, DexcomApiError> {
        let session_response = self.post(
            consts::DEXCOM_LOGIN_ENDPOINT,
            json!({
                "accountId": self.account_id,
                "password": self.password,
                "applicationId": consts::DEXCOM_APP_ID,
            }),
            Vec::new(),
        );

        match session_response {
            Ok(r) => Ok(r.text().unwrap().replace("\"", "")),
            Err(e) => Err(DexcomApiError::SessionError(e.to_string())),
        }
    }
}
