use super::ScheduleRepository;

pub struct ScheduleSeaOrmRepository {
    pub connection: sea_orm::DatabaseConnection,
}

impl ScheduleRepository for ScheduleSeaOrmRepository {}

impl ScheduleSeaOrmRepository {
    pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
        Self { connection }
    }
}
