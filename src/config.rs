use std::env;

/// Secure configuration for all external providers.
///
/// NOTE: This struct intentionally keeps values as `String` to avoid accidental
/// lifetime issues and to make it easy to pass into clients.
#[derive(Debug, Clone)]
pub struct PAGIConfig {
    pub openrouter_api_key: String,
    pub default_model: String,

    // Placeholder fields for other external integrations.
    pub jira_api_token: String,
    pub jira_base_url: String,

    pub crowdstrike_api_token: String,
    pub crowdstrike_base_url: String,
}

impl PAGIConfig {
    /// Loads configuration from `.env` (if present) and environment variables.
    ///
    /// # Panics
    /// Panics if `OPENROUTER_API_KEY` is missing. This enforces secure
    /// initialization at startup.
    pub fn load() -> PAGIConfig {
        // Load `.env` if available; ignore error so production env-only works.
        let _ = dotenvy::dotenv();

        let openrouter_api_key =
            env::var("OPENROUTER_API_KEY").expect("Missing required env var: OPENROUTER_API_KEY");

        let default_model = env::var("OPENROUTER_DEFAULT_MODEL")
            .unwrap_or_else(|_| "openai/gpt-4o-mini".to_string());

        let jira_api_token = env::var("JIRA_API_TOKEN").unwrap_or_default();
        let jira_base_url =
            env::var("JIRA_BASE_URL").unwrap_or_else(|_| "https://jira.example.com".to_string());

        let crowdstrike_api_token = env::var("CROWDSTRIKE_API_TOKEN").unwrap_or_default();
        let crowdstrike_base_url = env::var("CROWDSTRIKE_BASE_URL")
            .unwrap_or_else(|_| "https://api.crowdstrike.com".to_string());

        PAGIConfig {
            openrouter_api_key,
            default_model,
            jira_api_token,
            jira_base_url,
            crowdstrike_api_token,
            crowdstrike_base_url,
        }
    }
}
