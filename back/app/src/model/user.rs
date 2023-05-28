use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct JsonUserView {
    pub id: Uuid,
    pub name: String,
}
