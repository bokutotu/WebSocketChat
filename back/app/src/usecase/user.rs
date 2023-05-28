use adapter::module::UserRepository;

use crate::model::user::CreateUser;

pub struct UserUseCase<R: RepositoryModuleExt> {
    repository: Arc<R>,
}

impl UserUseCase<R: RepositoryModuleExt> {
    pub async fn add_user(&self, user: CreateUser) -> anyhow::Result<()> {
        todo!();
    }
}
