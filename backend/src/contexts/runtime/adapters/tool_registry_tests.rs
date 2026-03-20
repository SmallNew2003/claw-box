use super::tool_registry::ToolRegistry;
use crate::contexts::runtime::ports::{ToolInvocationError, ToolInvokerPort};
use serde_json::json;

#[test]
fn invoke_registered_tool() {
    let mut registry = ToolRegistry::new();
    registry.register("echo", |input| Ok(input.clone()));

    let out = registry.invoke("echo", &json!({"v": 1}));

    assert_eq!(out, Ok(json!({"v": 1})));
}

#[test]
fn invoke_unknown_tool_returns_not_found() {
    let registry = ToolRegistry::new();

    let out = registry.invoke("missing", &json!({}));

    assert_eq!(
        out,
        Err(ToolInvocationError::NotFound {
            tool_name: "missing".to_string(),
        })
    );
}
