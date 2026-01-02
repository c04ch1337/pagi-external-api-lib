//! PAGI External API & LLM Provider Library
//!
//! This crate centralizes:
//! - secure config loading (via `.env` + environment variables)
//! - external network I/O
//! - LLM provider orchestration (OpenRouter)

pub mod api_clients;
pub mod config;
pub mod llm_provider;

pub use api_clients::{CrowdstrikeClient, JiraClient};
pub use config::PAGIConfig;
pub use llm_provider::LLMProvider;
