use crate::contexts::runtime::domain::RuntimeEvent;
use crate::contexts::runtime::domain::ToolCallRecord;
use serde_json::Value;
use std::fmt;

/// Runtime 事件持久化端口。
pub trait RuntimeEventStorePort {
    fn store(&mut self, event: RuntimeEvent);
}

/// Tool 调用日志持久化端口。
pub trait ToolCallStorePort {
    fn store(&mut self, tool_call: ToolCallRecord);
}

/// Runtime 工具调用端口。
///
/// 约束：application 只能依赖这个抽象，不得依赖具体工具注册/执行实现。
pub trait ToolInvokerPort: Send + Sync {
    fn invoke(&self, tool_name: &str, input: &Value) -> Result<Value, ToolInvocationError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolInvocationError {
    /// 目标工具未注册。
    NotFound { tool_name: String },
    /// 工具执行失败，message 保留原始失败信息用于上层映射。
    Failed { tool_name: String, message: String },
}

impl fmt::Display for ToolInvocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolInvocationError::NotFound { tool_name } => {
                write!(f, "tool '{}' not found", tool_name)
            }
            ToolInvocationError::Failed { tool_name, message } => {
                write!(f, "tool '{}' failed: {}", tool_name, message)
            }
        }
    }
}

impl std::error::Error for ToolInvocationError {}
