//! Agent module containing the core agent logic

pub mod context;
pub mod loop_;
pub mod subagent;

pub use context::ContextBuilder;
pub use loop_::{ActiveSubagent, AgentLoop};
pub use subagent::SubagentManager;
