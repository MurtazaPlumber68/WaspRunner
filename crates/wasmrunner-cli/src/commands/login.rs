
use anyhow::Result;
use tracing::{info, error};
use dialoguer::{Input, Password};
use serde_json::json;

pub async fn execute(email: Option<String>) -> Result<()> {
    info!("Logging into WasmRunner app store");
    
    let email = match email {
        Some(e) => e,
        None => Input::new()
            .with_prompt("Email")
            .interact_text()?,
    };
    
    let password = Password::new()
        .with_prompt("Password")
        .interact()?;
    
    // TODO: Replace with actual Supabase client
    let supabase_url = std::env::var("SUPABASE_URL")
        .unwrap_or_else(|_| "https://your-project.supabase.co".to_string());
    let supabase_key = std::env::var("SUPABASE_ANON_KEY")
        .unwrap_or_else(|_| "your-anon-key".to_string());
    
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/auth/v1/token?grant_type=password", supabase_url))
        .header("apikey", &supabase_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;
    
    if response.status().is_success() {
        let auth_response: serde_json::Value = response.json().await?;
        
        if let Some(access_token) = auth_response.get("access_token") {
            // Store token in config file
            let config_dir = dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".wasmrunner");
            
            std::fs::create_dir_all(&config_dir)?;
            
            let auth_file = config_dir.join("auth.json");
            std::fs::write(
                auth_file,
                serde_json::to_string_pretty(&json!({
                    "access_token": access_token,
                    "email": email,
                    "supabase_url": supabase_url
                }))?
            )?;
            
            println!("✅ Successfully logged in as {}", email);
        } else {
            error!("Invalid response from authentication server");
            return Err(anyhow::anyhow!("Authentication failed"));
        }
    } else {
        let error_text = response.text().await?;
        error!("Authentication failed: {}", error_text);
        return Err(anyhow::anyhow!("Login failed: {}", error_text));
    }
    
    Ok(())
}
