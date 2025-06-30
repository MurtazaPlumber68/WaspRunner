
use anyhow::Result;
use tracing::{info, error};
use std::path::Path;
use serde_json::json;

pub async fn execute(
    image: String,
    manifest_path: Option<String>,
    description: Option<String>,
    category: Option<String>,
    tags: Vec<String>
) -> Result<()> {
    info!("Publishing {} to WasmRunner app store", image);
    
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
    
    // Parse image name and tag
    let (name, version) = if let Some((n, v)) = image.split_once(':') {
        (n.to_string(), v.to_string())
    } else {
        (image.clone(), "latest".to_string())
    };
    
    // Create slug from name
    let slug = name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();
    
    // Read manifest if provided
    let manifest_content = if let Some(path) = manifest_path {
        std::fs::read_to_string(path)?
    } else {
        // Generate basic manifest
        json!({
            "name": name,
            "version": version,
            "description": description.unwrap_or_else(|| format!("WASM application: {}", name)),
            "runtime": "wasmtime",
            "memory_limit": 128,
            "cpu_limit": 100
        }).to_string()
    };
    
    // TODO: Upload WASM file to storage and get URL
    let wasm_url = format!("https://storage.supabase.co/wasm/{}/{}.wasm", slug, version);
    let manifest_url = format!("https://storage.supabase.co/manifests/{}/{}.json", slug, version);
    
    // Create app record
    let client = reqwest::Client::new();
    let app_data = json!({
        "name": name,
        "slug": slug,
        "description": description.unwrap_or_else(|| format!("WASM application: {}", name)),
        "version": version,
        "category": category.unwrap_or_else(|| "utility".to_string()),
        "tags": tags,
        "manifest_url": manifest_url,
        "wasm_url": wasm_url,
        "is_published": true
    });
    
    let response = client
        .post(&format!("{}/rest/v1/apps", supabase_url))
        .header("apikey", std::env::var("SUPABASE_ANON_KEY").unwrap_or_default())
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(&app_data)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("✅ Successfully published {} v{}", name, version);
        println!("🔍 Search: wasmrunner search {}", name);
        println!("📦 Install: wasmrunner install {}", slug);
    } else {
        let error_text = response.text().await?;
        error!("Publish failed: {}", error_text);
        return Err(anyhow::anyhow!("Publish failed: {}", error_text));
    }
    
    Ok(())
}
