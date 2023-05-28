use async_trait;

use crate::model::user::UserView;

#[async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> anyhow::Result<UserView>;
}
