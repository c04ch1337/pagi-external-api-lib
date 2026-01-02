use crate::config::PAGIConfig;
use serde::{Deserialize, Serialize};

/// LLM orchestration wrapper.
#[derive(Debug, Clone)]
pub struct LLMProvider {
    pub config: PAGIConfig,
    client: reqwest::Client,
}

impl LLMProvider {
    /// Creates a new provider with config loaded from the environment.
    pub fn new() -> LLMProvider {
        LLMProvider {
            config: PAGIConfig::load(),
            client: reqwest::Client::new(),
        }
    }

    /// Calls OpenRouter Chat Completions and returns the raw text response.
    ///
    /// Endpoint:
    /// `https://openrouter.ai/api/v1/chat/completions`
    pub async fn generate_response(
        &self,
        prompt: &str,
        system_prompt: &str,
        model: Option<&str>,
    ) -> Result<String, reqwest::Error> {
        let model = model.unwrap_or(&self.config.default_model);

        let body = OpenRouterChatCompletionsRequest {
            model,
            messages: vec![
                OpenRouterMessage {
                    role: "system",
                    content: system_prompt,
                },
                OpenRouterMessage {
                    role: "user",
                    content: prompt,
                },
            ],
        };

        let resp = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .bearer_auth(&self.config.openrouter_api_key)
            // Recommended by OpenRouter docs; harmless if unset.
            .header("HTTP-Referer", "https://localhost")
            .header("X-Title", "pagi-external-api-lib")
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<OpenRouterChatCompletionsResponse>()
            .await?;

        Ok(resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default())
    }
}

#[derive(Debug, Serialize)]
struct OpenRouterChatCompletionsRequest<'a> {
    model: &'a str,
    messages: Vec<OpenRouterMessage<'a>>,
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChatCompletionsResponse {
    #[allow(dead_code)]
    id: Option<String>,
    choices: Vec<OpenRouterChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoiceMessage {
    content: String,
}
