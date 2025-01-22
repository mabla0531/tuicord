use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String
}

pub mod login_success {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Success {
        pub user_id: String,
        pub token: String,
        pub user_settings: UserSettings,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserSettings {
        pub locale: String,
        pub theme: String,
    }
}

pub mod login_failure {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Failure {
        pub message: String,
        pub code: u32,
        pub errors: Errors,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Errors {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<FieldError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<FieldError>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub login: Option<FieldError>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FieldError {
        #[serde(rename = "_errors")]
        pub errors: Vec<ErrorDetail>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorDetail {
        pub code: String,
        pub message: String,
    }
}

pub mod captcha_required {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Captcha {
        pub captcha_key: Vec<String>,
        pub captcha_sitekey: String,
        pub captcha_service: String,
        pub captcha_rqdata: String,
        pub captcha_rqtoken: String,
    }
}
