use crate::contexts::runtime::ports::{ToolInvocationError, ToolInvokerPort};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

type ToolHandler = Arc<dyn Fn(&Value) -> Result<Value, String> + Send + Sync>;

#[derive(Clone, Default)]
/// 基础设施层工具注册表适配器。
///
/// 通过实现 ToolInvokerPort，把具体工具执行能力注入到应用层。
pub struct ToolRegistry {
    handlers: HashMap<String, ToolHandler>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, tool_name: impl Into<String>, handler: F)
    where
        F: Fn(&Value) -> Result<Value, String> + Send + Sync + 'static,
    {
        self.handlers.insert(tool_name.into(), Arc::new(handler));
    }
}

impl ToolInvokerPort for ToolRegistry {
    fn invoke(&self, tool_name: &str, input: &Value) -> Result<Value, ToolInvocationError> {
        let handler =
            self.handlers
                .get(tool_name)
                .ok_or_else(|| ToolInvocationError::NotFound {
                    tool_name: tool_name.to_string(),
                })?;

        handler(input).map_err(|message| ToolInvocationError::Failed {
            tool_name: tool_name.to_string(),
            message,
        })
    }
}
