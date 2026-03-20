use crate::contexts::runtime::domain::RuntimeEvent;
use crate::contexts::runtime::domain::ToolCallRecord;
use crate::contexts::runtime::ports::{RuntimeEventStorePort, ToolCallStorePort};

pub mod tool_registry;
#[cfg(test)]
mod tool_registry_tests;

pub use tool_registry::ToolRegistry;

/// 过渡期内存适配器，作为持久化端口默认实现。
#[derive(Default)]
pub struct InMemoryRuntimeStores {
    pub events: Vec<RuntimeEvent>,
    pub tool_calls: Vec<ToolCallRecord>,
}

impl RuntimeEventStorePort for InMemoryRuntimeStores {
    fn store(&mut self, event: RuntimeEvent) {
        self.events.push(event);
    }
}

impl ToolCallStorePort for InMemoryRuntimeStores {
    fn store(&mut self, tool_call: ToolCallRecord) {
        self.tool_calls.push(tool_call);
    }
}
