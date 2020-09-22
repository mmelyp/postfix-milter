use super::mail_filter::{
    MailFilter, MailFilterRepository
};
use async_trait::async_trait;
use sqlx::PgConnection;
use crate::error::AppError;

#[async_trait]
impl MailFilterRepository for PgConnection {
    async fn get_global_scope_filters(
        &mut self,
    ) -> Result<Vec<MailFilter>, AppError> {
        let result = sqlx::query_as!(
            MailFilter,
            r#"
            SELECT id, email_routing_type, discr, domain_id, mail_account_id FROM postadmin_mail_filter
            WHERE discr = 'filterScopeGlobal'
            ORDER BY priority DESC
            "#,
        )
            .fetch_all(self)
            .await?;

        Ok(result)
    }

    async fn get_global_scope_filters_by_routing_type(
        &mut self,
        email_routing_type: String,
    ) -> Result<Vec<MailFilter>, AppError> {
        let result = sqlx::query_as!(
            MailFilter,
            r#"
            SELECT id, email_routing_type, discr, domain_id, mail_account_id FROM postadmin_mail_filter
            WHERE discr = 'filterScopeGlobal'
            AND (email_routing_type = $1 or email_routing_type = 'BOTH')
            ORDER BY priority DESC
            "#,
            email_routing_type,
        )
            .fetch_all(self)
            .await?;

        Ok(result)
    }
}
