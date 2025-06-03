pub mod dao;
pub mod repository;

#[async_trait::async_trait]
pub trait ContainerRepository {}
