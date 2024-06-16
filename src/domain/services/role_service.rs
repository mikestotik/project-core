use crate::domain::entities::role::Model as RoleModel;
use crate::infrastructure::repositories::role_repository::RoleRepository;


#[derive(Clone)]
pub struct RoleService {
    repo: RoleRepository,
}


impl RoleService {
    pub fn new(repo: RoleRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_roles(&self) -> Result<Vec<RoleModel>, sea_orm::DbErr> {
        self.repo.find_all().await
    }
}
