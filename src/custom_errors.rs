use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Attempting to add component to an entity without calling create component first")]
    CreateComponentNeverCalled,
    #[error("attempting to reference a component that wasn't registered")]
    ComponentNotRegistered,
    #[error("attempting to reference an entity that doesn't exist")]
    EntityDoesNotExist,
}
