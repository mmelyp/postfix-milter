use super::mail_filter_action::{
    MailFilterAction, MailFilterActionRepository
};
use async_trait::async_trait;
use sqlx::PgConnection;
use crate::error::AppError;

#[async_trait]
impl MailFilterActionRepository for PgConnection {
    async fn get_actions_by_filter(
        &mut self,
        filter_id: uuid::Uuid,
    ) -> Result<Vec<MailFilterAction>, AppError> {
        let result = sqlx::query_as!(
            MailFilterAction,
            r#"
            SELECT id, mail_filter_id, discr, reason, dest_address FROM postadmin_mail_filter_action
            WHERE mail_filter_id = $1
            "#,
            filter_id
        )
            .fetch_all(self)
            .await?;

        Ok(result)
    }

    async fn has_final_actions_only(
        &mut self,
        filter_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        let result = sqlx::query!(
            "
            SELECT COUNT(*) as count FROM postadmin_mail_filter_action
            WHERE mail_filter_id = $1 AND is_final = '0'
            ",
            filter_id
        )
            .fetch_one(self)
            .await?;

        if result.count.unwrap() > 0 {
            return Ok(false);
        }

        return Ok(true)
    }
}
