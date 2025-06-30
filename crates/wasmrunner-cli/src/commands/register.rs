
use anyhow::Result;
use tracing::{info, error};
use dialoguer::{Input, Password, Confirm};
use serde_json::json;

pub async fn execute(email: Option<String>, username: Option<String>) -> Result<()> {
    info!("Creating new WasmRunner account");
    
    let email = match email {
        Some(e) => e,
        None => Input::new()
            .with_prompt("Email")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.contains('@') {
                    Ok(())
                } else {
                    Err("Please enter a valid email address")
                }
            })
            .interact_text()?,
    };
    
    let username = match username {
        Some(u) => u,
        None => Input::new()
            .with_prompt("Username")
            .with_initial_text(&email.split('@').next().unwrap_or("user").to_string())
            .interact_text()?,
    };
    
    let password = Password::new()
        .with_prompt("Password")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()?;
    
    let terms_accepted = Confirm::new()
        .with_prompt("Do you accept the Terms of Service?")
        .interact()?;
    
    if !terms_accepted {
        println!("Registration cancelled - Terms of Service must be accepted");
        return Ok(());
    }
    
    let supabase_url = std::env::var("SUPABASE_URL")
        .unwrap_or_else(|_| "https://your-project.supabase.co".to_string());
    let supabase_key = std::env::var("SUPABASE_ANON_KEY")
        .unwrap_or_else(|_| "your-anon-key".to_string());
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/auth/v1/signup", supabase_url))
        .header("apikey", &supabase_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
            "data": {
                "username": username
            }
        }))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("✅ Account created successfully!");
        println!("📧 Please check your email to verify your account.");
        println!("🔑 Once verified, run `wasmrunner login` to start publishing apps.");
    } else {
        let error_text = response.text().await?;
        error!("Registration failed: {}", error_text);
        return Err(anyhow::anyhow!("Registration failed: {}", error_text));
    }
    
    Ok(())
}
