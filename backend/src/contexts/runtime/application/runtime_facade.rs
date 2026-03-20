use crate::contexts::runtime::application::{
    AgentRuntimeError, AgentRuntimeService, ExecutionInput, ExecutionReport,
};

/// Runtime 上下文应用门面。
///
/// 过渡阶段由门面托管旧服务，后续再把实现内聚到 runtime context。
pub struct RuntimeApplicationFacade {
    service: AgentRuntimeService,
}

impl RuntimeApplicationFacade {
    pub fn new(service: AgentRuntimeService) -> Self {
        Self { service }
    }

    pub fn execute(&self, input: ExecutionInput) -> Result<ExecutionReport, AgentRuntimeError> {
        self.service.execute(input)
    }

    pub fn into_inner(self) -> AgentRuntimeService {
        self.service
    }
}
