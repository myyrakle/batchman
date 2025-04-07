use super::TaskDefinitionRepository;

pub struct TaskDefinitionSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

impl TaskDefinitionRepository for TaskDefinitionSeaOrmRepository {}

impl TaskDefinitionSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
