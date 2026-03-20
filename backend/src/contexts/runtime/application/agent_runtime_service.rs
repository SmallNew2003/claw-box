use crate::contexts::runtime::domain::{
    RuntimeEvent, RuntimeEventType, RuntimeExecution, RuntimeExecutionStatus, RuntimeMetrics,
    ToolCallRecord,
};
use crate::contexts::runtime::ports::{
    RuntimeEventStorePort, ToolCallStorePort, ToolInvocationError, ToolInvokerPort,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Runtime 执行输入模型，约束一次执行所需的最小上下文。
pub struct ExecutionInput {
    pub execution_id: String,
    pub session_id: String,
    pub message_id: String,
    pub trace_id: Option<String>,
    pub message_content: String,
    pub started_at: String,
    pub completed_at: String,
    pub requested_tool: Option<RequestedTool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 可选工具调用请求，作为一次执行中的扩展步骤。
pub struct RequestedTool {
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// 运行时执行产物，聚合执行状态、事件流和工具调用记录。
pub struct ExecutionReport {
    pub execution: RuntimeExecution,
    pub events: Vec<RuntimeEvent>,
    pub tool_calls: Vec<ToolCallRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentRuntimeError {
    InvalidState(String),
    Tool(ToolInvocationError),
}

pub struct AgentRuntimeService {
    tools: Box<dyn ToolInvokerPort>,
}

impl AgentRuntimeService {
    /// 创建运行时服务实例，工具注册表由上层注入。
    pub fn new(tools: Box<dyn ToolInvokerPort>) -> Self {
        Self { tools }
    }

    /// 执行一次单 Agent 任务，输出完整的执行报告。
    ///
    /// 约束：该方法只负责一次 execution，不处理会话级并发控制。
    pub fn execute(&self, input: ExecutionInput) -> Result<ExecutionReport, AgentRuntimeError> {
        let mut execution = RuntimeExecution::new(
            input.execution_id.clone(),
            input.session_id.clone(),
            input.message_id,
        );

        execution
            .start(input.started_at.clone())
            .map_err(|error| AgentRuntimeError::InvalidState(error.to_string()))?;

        let mut events = vec![runtime_event(
            "evt-start",
            &execution,
            RuntimeEventType::ExecutionStarted,
            with_trace_id(json!({"status": "running"}), input.trace_id.as_deref()),
            input.started_at.clone(),
        )];

        let mut tool_calls = Vec::new();
        let mut output_text = format!("Echo: {}", input.message_content);
        let mut output_tokens = output_text.chars().count() as u64;

        if let Some(requested_tool) = input.requested_tool {
            let tool_call_id = format!("tool-{}", execution.execution_id());
            events.push(runtime_event(
                "evt-tool-start",
                &execution,
                RuntimeEventType::ToolCallStarted,
                with_trace_id(
                    json!({
                        "tool_call_id": tool_call_id,
                        "tool_name": requested_tool.name,
                        "input": requested_tool.input
                    }),
                    input.trace_id.as_deref(),
                ),
                input.started_at.clone(),
            ));

            let invoked = self
                .tools
                .invoke(&requested_tool.name, &requested_tool.input)
                .map_err(AgentRuntimeError::Tool);

            match invoked {
                Ok(tool_output) => {
                    output_text = format!("Tool {} finished", requested_tool.name);
                    output_tokens = tool_output.to_string().chars().count() as u64;
                    tool_calls.push(ToolCallRecord {
                        tool_call_id: tool_call_id.clone(),
                        execution_id: execution.execution_id().to_string(),
                        session_id: execution.session_id().to_string(),
                        tool_name: requested_tool.name.clone(),
                        status: RuntimeExecutionStatus::Completed,
                        input: requested_tool.input,
                        output: Some(tool_output.clone()),
                        error_message: None,
                        started_at: input.started_at.clone(),
                        completed_at: Some(input.completed_at.clone()),
                    });
                    events.push(runtime_event(
                        "evt-tool-complete",
                        &execution,
                        RuntimeEventType::ToolCallCompleted,
                        with_trace_id(
                            json!({
                                "tool_call_id": tool_call_id,
                                "tool_name": requested_tool.name,
                                "output": tool_output
                            }),
                            input.trace_id.as_deref(),
                        ),
                        input.completed_at.clone(),
                    ));
                }
                Err(AgentRuntimeError::Tool(tool_error)) => {
                    let error_message = tool_error.to_string();
                    tool_calls.push(ToolCallRecord {
                        tool_call_id: tool_call_id.clone(),
                        execution_id: execution.execution_id().to_string(),
                        session_id: execution.session_id().to_string(),
                        tool_name: requested_tool.name.clone(),
                        status: RuntimeExecutionStatus::Failed,
                        input: requested_tool.input,
                        output: None,
                        error_message: Some(error_message.clone()),
                        started_at: input.started_at.clone(),
                        completed_at: Some(input.completed_at.clone()),
                    });
                    execution
                        .fail(input.completed_at.clone(), error_message.clone(), None)
                        .map_err(|error| AgentRuntimeError::InvalidState(error.to_string()))?;
                    events.push(runtime_event(
                        "evt-failed",
                        &execution,
                        RuntimeEventType::ExecutionFailed,
                        with_trace_id(
                            json!({"error_message": error_message}),
                            input.trace_id.as_deref(),
                        ),
                        input.completed_at,
                    ));

                    return Ok(ExecutionReport {
                        execution,
                        events,
                        tool_calls,
                    });
                }
                Err(other) => return Err(other),
            }
        }

        events.push(runtime_event(
            "evt-progress",
            &execution,
            RuntimeEventType::StepProgress,
            with_trace_id(
                json!({"message": "execution step completed"}),
                input.trace_id.as_deref(),
            ),
            input.completed_at.clone(),
        ));

        let metrics = RuntimeMetrics {
            latency_ms: 120,
            input_tokens: input.message_content.chars().count() as u64,
            output_tokens,
        };

        execution
            .complete(
                input.completed_at.clone(),
                Some(output_text.clone()),
                Some(metrics.clone()),
            )
            .map_err(|error| AgentRuntimeError::InvalidState(error.to_string()))?;

        events.push(runtime_event(
            "evt-complete",
            &execution,
            RuntimeEventType::ExecutionCompleted,
            with_trace_id(
                json!({
                    "result_summary": output_text,
                    "metrics": {
                        "latency_ms": metrics.latency_ms,
                        "input_tokens": metrics.input_tokens,
                        "output_tokens": metrics.output_tokens
                    }
                }),
                input.trace_id.as_deref(),
            ),
            input.completed_at,
        ));

        Ok(ExecutionReport {
            execution,
            events,
            tool_calls,
        })
    }

    /// 执行并把副作用写入 runtime adapter 端口。
    pub fn execute_with_adapters<S>(
        &self,
        input: ExecutionInput,
        stores: &mut S,
    ) -> Result<ExecutionReport, AgentRuntimeError>
    where
        S: RuntimeEventStorePort + ToolCallStorePort,
    {
        let report = self.execute(input)?;
        for event in report.events.iter().cloned() {
            RuntimeEventStorePort::store(stores, event);
        }
        for tool_call in report.tool_calls.iter().cloned() {
            ToolCallStorePort::store(stores, tool_call);
        }
        Ok(report)
    }
}

fn runtime_event(
    event_id: &str,
    execution: &RuntimeExecution,
    event_type: RuntimeEventType,
    payload: serde_json::Value,
    created_at: String,
) -> RuntimeEvent {
    RuntimeEvent {
        event_id: event_id.to_string(),
        execution_id: execution.execution_id().to_string(),
        session_id: execution.session_id().to_string(),
        event_type,
        payload,
        created_at,
    }
}

fn with_trace_id(mut payload: serde_json::Value, trace_id: Option<&str>) -> serde_json::Value {
    if let (Some(trace_id), Some(object)) = (trace_id, payload.as_object_mut()) {
        object.insert(
            "trace_id".to_string(),
            serde_json::Value::String(trace_id.to_string()),
        );
    }
    payload
}

impl fmt::Display for AgentRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentRuntimeError::InvalidState(message) => {
                write!(f, "invalid runtime state: {}", message)
            }
            AgentRuntimeError::Tool(err) => write!(f, "tool runtime error: {}", err),
        }
    }
}

impl std::error::Error for AgentRuntimeError {}
