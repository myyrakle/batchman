use crate::domain::container::ContainerRepository;

#[derive(Debug, Clone)]
pub struct ContainerDockerRepository {}

impl ContainerDockerRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ContainerRepository for ContainerDockerRepository {}
