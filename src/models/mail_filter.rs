use serde::{Deserialize, Serialize};
use crate::error::AppError;
use async_trait::async_trait;

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MailFilter {
    pub id: uuid::Uuid,
    pub discr: String,
    pub email_routing_type: String,
    pub domain_id: Option<uuid::Uuid>,
    pub mail_account_id: Option<uuid::Uuid>,
}

#[async_trait]
pub(crate) trait MailFilterRepository: Send + Sync + 'static {
    async fn get_global_scope_filters(
        &mut self,
    ) -> Result<Vec<MailFilter>, AppError>;

    async fn get_global_scope_filters_by_routing_type(
        &mut self,
        email_routing_type: String,
    ) -> Result<Vec<MailFilter>, AppError>;
}