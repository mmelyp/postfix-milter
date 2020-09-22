use serde::{Deserialize, Serialize};
use crate::error::AppError;
use async_trait::async_trait;

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MailFilterAction {
    pub id: uuid::Uuid,
    pub mail_filter_id: uuid::Uuid,
    pub discr: String,
    pub reason: Option<String>,
    pub dest_address: Option<String>,
}

#[async_trait]
pub(crate) trait MailFilterActionRepository: Send + Sync + 'static {
    async fn get_actions_by_filter(
        &mut self,
        filter_id: uuid::Uuid,
    ) -> Result<Vec<MailFilterAction>, AppError>;

    async fn has_final_actions_only(
        &mut self,
        filter_id: uuid::Uuid,
    ) -> Result<bool, AppError>;
}