
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
    pub security: SecurityConfig,
    pub registry: RegistryConfig,
    pub plugins: PluginConfig,
    pub supabase: Option<SupabaseConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub default_runtime: String,
    pub memory_limit_mb: u64,
    pub cpu_limit_percent: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_seccomp: bool,
    pub seccomp_profile: Option<PathBuf>,
    pub enable_memory_guard: bool,
    pub allow_network: bool,
    pub allow_filesystem: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub default_registry: String,
    pub cache_dir: PathBuf,
    pub auth_config: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin_dir: PathBuf,
    pub auto_discovery: bool,
    pub trusted_publishers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
    pub service_key: Option<String>,
    pub enable_auth: bool,
    pub enable_storage: bool,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let wasmrunner_dir = home_dir.join(".wasmrunner");
        
        Self {
            runtime: RuntimeConfig {
                default_runtime: "wasmtime".to_string(),
                memory_limit_mb: 128,
                cpu_limit_percent: 100,
                timeout_seconds: 300,
            },
            security: SecurityConfig {
                enable_seccomp: true,
                seccomp_profile: None,
                enable_memory_guard: true,
                allow_network: false,
                allow_filesystem: true,
            },
            registry: RegistryConfig {
                default_registry: "registry.wasmrunner.dev".to_string(),
                cache_dir: wasmrunner_dir.join("cache"),
                auth_config: Some(wasmrunner_dir.join("auth.json")),
            },
            plugins: PluginConfig {
                plugin_dir: wasmrunner_dir.join("plugins"),
                auto_discovery: true,
                trusted_publishers: vec!["wasmrunner.dev".to_string()],
            },
            supabase: None,
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        let mut config = config::Config::builder();
        
        // Add default configuration
        config = config.add_source(config::Config::try_from(&Config::default())?);
        
        // Add config file if specified
        if let Some(path) = config_path {
            config = config.add_source(config::File::with_name(path));
        } else {
            // Try standard locations
            let home_dir = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            let config_file = home_dir.join(".wasmrunner").join("config.toml");
            if config_file.exists() {
                config = config.add_source(config::File::from(config_file));
            }
        }
        
        // Add environment variables
        config = config.add_source(
            config::Environment::with_prefix("WASMRUNNER").separator("_")
        );
        
        let mut config: Config = config.build()?.try_deserialize()?;
        
        // Override Supabase config from environment if available
        if let (Ok(url), Ok(key)) = (std::env::var("SUPABASE_URL"), std::env::var("SUPABASE_ANON_KEY")) {
            config.supabase = Some(SupabaseConfig {
                url,
                anon_key: key,
                service_key: std::env::var("SUPABASE_SERVICE_KEY").ok(),
                enable_auth: true,
                enable_storage: true,
            });
        }
        
        // Ensure directories exist
        std::fs::create_dir_all(&config.registry.cache_dir)?;
        std::fs::create_dir_all(&config.plugins.plugin_dir)?;
        
        Ok(config)
    }
}
