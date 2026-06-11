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

pub async fn get_with_username(
    pool: &sqlx::PgPool,
    username: &String,
) -> Result<textsender_models::user::ServiceUser, sqlx::Error> {
    match sqlx::query(
        r#"SELECT id, username, passphrase, created, last_login FROM "service_user" WHERE username = $1"#
        ).bind(username)
        .fetch_one(pool).await {
        Ok(row) => {
            let service_user = textsender_models::user::ServiceUser {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                passphrase: row.try_get("passphrase")?,
                created: row.try_get("created")?,
                last_login: row.try_get("last_login")?,
                salt_id: row.try_get("salt_id")?
            };

            Ok(service_user)
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub async fn insert(
    pool: &sqlx::PgPool,
    service_user: &textsender_models::user::ServiceUser,
) -> Result<time::OffsetDateTime, sqlx::Error> {
    match sqlx::query(
        r#"INSERT INTO "service_user" (username, passphrase, salt_id)
            VALUES ($1, $2, $3)
            RETURNING created
        "#,
    )
    .bind(&service_user.username)
    .bind(&service_user.passphrase)
    .bind(service_user.salt_id)
    .fetch_one(pool)
    .await
    {
        Ok(row) => {
            let created: time::OffsetDateTime = row.try_get("created")?;
            Ok(created)
        }
        Err(err) => Err(err),
    }
}

pub async fn exists(pool: &sqlx::PgPool, service_username: &String) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
    SELECT 1 FROM "service_user" WHERE username = $1
    "#,
    )
    .bind(service_username)
    .fetch_optional(pool)
    .await;

    match result {
        Ok(r) => match r {
            Some(row) => {
                if row.is_empty() {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            None => Ok(false),
        },
        Err(e) => {
            eprintln!("What??");
            eprintln!("Error: {e:?}");
            Err(e)
        }
    }
}
