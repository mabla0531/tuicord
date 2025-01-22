mod model;

use model::{login_failure::Failure, login_success::Success, captcha_required::Captcha, LoginCredentials};

// endpoint = https://discord.com/api/v9/

#[tokio::main]
async fn main() {
    let response = reqwest::Client::new()
        .post("https://discord.com/api/v9/auth/login")
        .json(&LoginCredentials {
            email:    "NOT MY BURNER ACCOUNT EMAIL".to_string(),
            password: "NOT MY BURNER ACCOUNT PASSWORD".to_string(),
        })
        .send()
        .await
        .unwrap();

    let response= response.text().await.unwrap();

    match serde_json::from_str::<Success>(response.as_str()) {
        Ok(success) => {
            println!("User ID: {}", success.user_id);
            println!("Token: {}", success.token);
            println!("User Settings: {:?}", success.user_settings);
        }
        Err(_) => match serde_json::from_str::<Failure>(response.as_str()) {
            Ok(failure) => {
                println!("Message: {}", failure.message);
                println!("Code: {}", failure.code);
                println!("Errors: {:?}", failure.errors);
            }
            Err(_) => match serde_json::from_str::<Captcha>(response.as_str()) {
                // Captcha handling is a massive question mark right now, I have no clue how/if it will be done in a CLI
                Ok(_) => println!("Captcha required"),
                Err(_) => println!("I have no effing idea what went wrong on the API end lmao, payload is \n{}", response)
            }
        }
    }
}
