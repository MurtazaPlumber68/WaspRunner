
use anyhow::Result;
use tracing::{info, error};
use serde_json::json;

pub async fn execute(app: String, remove: bool) -> Result<()> {
    let action = if remove { "Removing" } else { "Adding" };
    info!("{} {} to/from favorites", action, app);
    
    // Load auth token
    let config_dir = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".wasmrunner");
    
    let auth_file = config_dir.join("auth.json");
    if !auth_file.exists() {
        error!("Not logged in. Run `wasmrunner login` first.");
        return Err(anyhow::anyhow!("Authentication required"));
    }
    
    let auth_data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(auth_file)?
    )?;
    
    let access_token = auth_data.get("access_token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid auth token"))?;
    
    let supabase_url = auth_data.get("supabase_url")
        .and_then(|u| u.as_str())
        .unwrap_or("https://your-project.supabase.co");
    
    let client = reqwest::Client::new();
    
    if remove {
        // Remove favorite
        let response = client
            .delete(&format!("{}/rest/v1/user_favorites", supabase_url))
            .header("apikey", std::env::var("SUPABASE_ANON_KEY").unwrap_or_default())
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .query(&[("app_id", &format!("eq.{}", app))])
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("💔 Removed {} from favorites", app);
        } else {
            error!("Failed to remove favorite");
            return Err(anyhow::anyhow!("Failed to remove favorite"));
        }
    } else {
        // Add favorite
        let favorite_data = json!({
            "app_id": app
        });
        
        let response = client
            .post(&format!("{}/rest/v1/user_favorites", supabase_url))
            .header("apikey", std::env::var("SUPABASE_ANON_KEY").unwrap_or_default())
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&favorite_data)
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("⭐ Added {} to favorites", app);
        } else {
            error!("Failed to add favorite");
            return Err(anyhow::anyhow!("Failed to add favorite"));
        }
    }
    
    Ok(())
}
