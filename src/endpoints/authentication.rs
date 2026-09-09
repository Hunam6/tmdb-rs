use serde::Deserialize;

use crate::common::StatusResponse;
use crate::SessionId;

#[derive(Debug, Clone, Deserialize)]
pub struct RequestToken {
    pub success: bool,
    pub request_token: String,
    pub expires_at: String,
}

impl RequestToken {
    /// the url to send the user to so they can approve the token
    pub fn redirect_url(&self) -> String {
        format!(
            "https://www.themoviedb.org/authenticate/{}",
            self.request_token
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    pub success: bool,
    pub session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuestSession {
    pub success: bool,
    pub guest_session_id: String,
    pub expires_at: String,
}

endpoint! {
    /// validate the credentials: the key is good when this answers
    validate_key(): GET "/authentication" => bool [success]
}

endpoint! {
    /// step 1 of the v3 auth flow: a request token for the user to approve
    request_token(): GET "/authentication/token/new" => RequestToken
}

endpoint! {
    /// step 2, app-approved variant: validate with the user's credentials
    validate_with_login(): POST "/authentication/token/validate_with_login" => RequestToken {
        body { username: &str, password: &str, request_token: &str }
    }
}

endpoint! {
    /// step 3: exchange an approved request token for a session
    create_session(): POST "/authentication/session/new" => Session {
        body { request_token: &str }
    }
}

endpoint! {
    /// a guest session, for rating without an account
    create_guest_session(): GET "/authentication/guest_session/new" => GuestSession
}

endpoint! {
    /// end a session
    delete_session(): DELETE "/authentication/session" => StatusResponse {
        body { session_id: SessionId }
    }
}
