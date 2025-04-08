use super::JobRepository;

pub struct JobSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

impl JobRepository for JobSeaOrmRepository {}

impl JobSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
