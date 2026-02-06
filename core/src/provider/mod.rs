//! LLM provider module
//!
//! This module provides LLM provider abstractions with mofa_sdk integration.
//!
//! LLM interaction and transcription now use MoFA framework directly.

// Re-export ToolCallRequest from types since it's defined there
pub use crate::types::ToolCallRequest;

// Re-export mofa_sdk LLM types for convenience
pub use mofa_sdk::llm::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage, GroqTranscriptionProvider,
    LLMAgent, LLMAgentBuilder, LLMError, LLMProvider as MofaLLMProvider, LLMResult,
    OpenAITranscriptionProvider, Tool as MofaTool, ToolCall as MofaToolCall, TranscriptionProvider,
    openai::OpenAIConfig, openai::OpenAIProvider,
};
