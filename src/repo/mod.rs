pub mod service;

pub mod user {
    use sqlx::Row;

    #[derive(Debug, serde::Serialize, sqlx::FromRow)]
    pub struct InsertedData {
        pub id: uuid::Uuid,
        pub date_created: Option<time::OffsetDateTime>,
    }

    pub async fn get(
        pool: &sqlx::PgPool,
        username: &String,
    ) -> Result<textsender_models::user::User, sqlx::Error> {
        let result = sqlx::query(
            r#"
        SELECT id, username, password, phone_number, salt_id, firstname, lastname, created, last_login FROM "user" WHERE username = $1
        "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await;

        match result {
            Ok(r) => match r {
                Some(r) => Ok(textsender_models::user::User {
                    id: r.try_get("id")?,
                    username: r.try_get("username")?,
                    password: r.try_get("password")?,
                    phone_number: r.try_get("phone_number")?,
                    salt_id: r.try_get("salt_id")?,
                    firstname: r.try_get("firstname")?,
                    lastname: r.try_get("lastname")?,
                    created: r.try_get("created")?,
                    last_login: r.try_get("last_login")?,
                }),
                None => Err(sqlx::Error::RowNotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_with_id(
        pool: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> Result<textsender_models::user::User, sqlx::Error> {
        match sqlx::query(
            r#"
        SELECT id, username, password, phone_number, salt_id, firstname, lastname, created, last_login FROM "user" WHERE id = $1
        "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await {
            Ok(r) => match r {
                Some(r) => Ok(textsender_models::user::User {
                    id: r.try_get("id")?,
                    username: r.try_get("username")?,
                    password: r.try_get("password")?,
                    phone_number: r.try_get("phone_number")?,
                    salt_id: r.try_get("salt_id")?,
                    firstname: r.try_get("firstname")?,
                    lastname: r.try_get("lastname")?,
                    created: r.try_get("created")?,
                    last_login: r.try_get("last_login")?,
                }),
                None => Err(sqlx::Error::RowNotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update_last_login(
        pool: &sqlx::PgPool,
        user: &textsender_models::user::User,
        time: &time::OffsetDateTime,
    ) -> Result<time::OffsetDateTime, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE "user" SET last_login = $1 WHERE id = $2 RETURNING last_login
            "#,
        )
        .bind(time)
        .bind(user.id)
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

    pub async fn update_password(
        pool: &sqlx::PgPool,
        user: &textsender_models::user::User,
        updated_hashed_password: &String,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query(
            r#"
            UPDATE "user" SET password = $1 WHERE id = $2
            "#,
        )
        .bind(updated_hashed_password)
        .bind(user.id)
        .execute(pool)
        .await
        {
            Ok(row) => {
                if row.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(sqlx::Error::RowNotFound)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn update_name(
        pool: &sqlx::PgPool,
        id: &uuid::Uuid,
        firstname: &str,
        lastname: &str,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query(
            r#"
            UPDATE "user" SET firstname = $1, lastname = $2 WHERE id = $3
            "#,
        )
        .bind(firstname)
        .bind(lastname)
        .bind(id)
        .execute(pool)
        .await
        {
            Ok(row) => {
                if row.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(sqlx::Error::RowNotFound)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn exists(pool: &sqlx::PgPool, username: &String) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
        SELECT 1 FROM "user" WHERE username = $1
        "#,
        )
        .bind(username)
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

    pub async fn insert(
        pool: &sqlx::PgPool,
        user: &textsender_models::user::User,
    ) -> Result<(uuid::Uuid, std::option::Option<time::OffsetDateTime>), sqlx::Error> {
        let row = sqlx::query(
            r#"
                INSERT INTO "user" (username, password, phone_number, firstname, lastname, salt_id) 
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, created;
            "#,
        )
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.phone_number)
        .bind(&user.firstname)
        .bind(&user.lastname)
        .bind(user.salt_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Error inserting item: {e}");
            e
        })?;

        let result = InsertedData {
            id: row.try_get("id").map_err(|_e| sqlx::Error::RowNotFound)?,
            date_created: row
                .try_get("created")
                .map_err(|_e| sqlx::Error::RowNotFound)?,
        };

        if result.id.is_nil() && result.date_created.is_none() {
            Err(sqlx::Error::RowNotFound)
        } else {
            Ok((result.id, result.date_created))
        }
    }
}

pub mod salt {
    use sqlx::Row;

    #[derive(Debug, serde::Serialize, sqlx::FromRow)]
    pub struct InsertedData {
        pub id: uuid::Uuid,
    }

    pub async fn get(
        pool: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> Result<textsender_models::user::Salt, sqlx::Error> {
        let result = sqlx::query(
            r#"
        SELECT id, salt FROM "salt" WHERE id = $1
        "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await;

        match result {
            Ok(r) => match r {
                Some(r) => Ok(textsender_models::user::Salt {
                    id: r.try_get("id")?,
                    salt: r.try_get("salt")?,
                }),
                None => Err(sqlx::Error::RowNotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn insert(
        pool: &sqlx::PgPool,
        salt: &textsender_models::user::Salt,
    ) -> Result<uuid::Uuid, sqlx::Error> {
        let row = sqlx::query(
            r#"
                INSERT INTO "salt" (salt) 
                VALUES ($1)
                RETURNING id;
            "#,
        )
        .bind(&salt.salt)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Error inserting item: {e}");
            e
        })?;

        let result = InsertedData {
            id: row.try_get("id").map_err(|_e| sqlx::Error::RowNotFound)?,
        };

        if !result.id.is_nil() {
            Ok(result.id)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}
