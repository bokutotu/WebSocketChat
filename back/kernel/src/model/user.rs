use uuid::Uuid;

use crate::model::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
}
