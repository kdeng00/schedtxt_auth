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

pub async fn get(
    pool: &sqlx::PgPool,
    id: &uuid::Uuid,
) -> Result<textsender_models::user::ServiceUser, sqlx::Error> {
    match sqlx::query(
        r#"SELECT id, username, passphrase, created, last_login, salt_id FROM "service_user" WHERE id = $1"#
        ).bind(id)
        .fetch_one(pool).await {
        Ok(row) => {
            let last_login: Option<time::OffsetDateTime> = match row.try_get("last_login") {
                Ok(login) => {
                    Some(login)
                }
                Err(err) => {
                    eprintln!("Error: {err:?}");
                    None
                }
            };

            let service_user = textsender_models::user::ServiceUser {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                passphrase: row.try_get("passphrase")?,
                created: row.try_get("created")?,
                last_login,
                salt_id: row.try_get("salt_id")?,
            };

            Ok(service_user)
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub async fn get_with_username(
    pool: &sqlx::PgPool,
    username: &String,
) -> Result<textsender_models::user::ServiceUser, sqlx::Error> {
    match sqlx::query(
        r#"SELECT id, username, passphrase, created, last_login, salt_id FROM "service_user" WHERE username = $1"#
        ).bind(username)
        .fetch_one(pool).await {
        Ok(row) => {
            let last_login: Option<time::OffsetDateTime> = match row.try_get("last_login") {
                Ok(login) => {
                    Some(login)
                }
                Err(err) => {
                    eprintln!("Error: {err:?}");
                    None
                }
            };

            let service_user = textsender_models::user::ServiceUser {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                passphrase: row.try_get("passphrase")?,
                created: row.try_get("created")?,
                last_login,
                salt_id: row.try_get("salt_id")?,
            };

            Ok(service_user)
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub async fn update_last_login(
    pool: &sqlx::PgPool,
    service_user: &textsender_models::user::ServiceUser,
    time: &time::OffsetDateTime,
) -> Result<time::OffsetDateTime, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE "service_user" SET last_login = $1 WHERE id = $2 RETURNING last_login
        "#,
    )
    .bind(time)
    .bind(service_user.id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Error updating time: {e}");
        e
    });

    match result {
        Ok(row) => match row {
            Some(r) => {
                let last_login: time::OffsetDateTime = r
                    .try_get("last_login")
                    .map_err(|_e| sqlx::Error::RowNotFound)?;
                Ok(last_login)
            }
            None => Err(sqlx::Error::RowNotFound),
        },
        Err(err) => Err(err),
    }
}

pub async fn insert(
    pool: &sqlx::PgPool,
    service_user: &textsender_models::user::ServiceUser,
) -> Result<(uuid::Uuid, time::OffsetDateTime), sqlx::Error> {
    match sqlx::query(
        r#"INSERT INTO "service_user" (username, passphrase, salt_id)
            VALUES ($1, $2, $3)
            RETURNING id, created
        "#,
    )
    .bind(&service_user.username)
    .bind(&service_user.passphrase)
    .bind(service_user.salt_id)
    .fetch_one(pool)
    .await
    {
        Ok(row) => {
            let id: uuid::Uuid = row.try_get("id")?;
            let created: time::OffsetDateTime = row.try_get("created")?;
            Ok((id, created))
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
