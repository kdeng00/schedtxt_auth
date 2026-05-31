use sqlx::Row;

pub async fn valid_passphrase(
    pool: &sqlx::PgPool,
    passphrase: &String,
) -> Result<(uuid::Uuid, String, time::OffsetDateTime), sqlx::Error> {
    let result = sqlx::query(
        r#"
        SELECT id, username, date_created FROM "passphrase" WHERE passphrase = $1
        "#,
    )
    .bind(passphrase)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => {
            let id: uuid::Uuid = row.try_get("id")?;
            let username: String = row.try_get("username")?;
            let date_created: Option<time::OffsetDateTime> = row.try_get("date_created")?;

            Ok((id, username, date_created.unwrap()))
        }
        Err(err) => Err(err),
    }
}

pub async fn get_passphrase(
    pool: &sqlx::PgPool,
    id: &uuid::Uuid,
) -> Result<(String, String, time::OffsetDateTime), sqlx::Error> {
    let result = sqlx::query(
        r#"
        SELECT username, passphrase, date_created FROM "passphrase" WHERE id = $1;
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => {
            let username: String = row.try_get("username")?;
            let passphrase: String = row.try_get("passphrase")?;
            let date_created: time::OffsetDateTime = row.try_get("date_created")?;
            Ok((username, passphrase, date_created))
        }
        Err(err) => Err(err),
    }
}
