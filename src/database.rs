use crate::database::models::User;
use sha2::{Digest, Sha512};

pub mod models;
mod validators;

use sqlx::{Pool, Postgres};
use validators::{check_for_duplicates, validate_data};

// enum for result of user creation
#[derive(PartialEq, Debug)]
pub enum UserCreationResult {
    TooShortPassword,
    TooLongPassword,
    TooLongEmail,
    TooLongUsername,
    UserNameIsTaken,
    EmailIsTaken,
    UnknownError,
    ValidData,
}

/// Function hashes password
fn sha512_password(password: &str) -> String {
    let mut sha512 = Sha512::new();
    sha512.update(password);

    // returning hashed password
    format!("{:X}", sha512.finalize())
}

/// Function validates provided data using sqlx database connection manager
/// (checks if email or username is already taken) and if data is valid,
/// inserts new user into table "users" in the database
pub async fn add_user(
    user: &User,
    pool: &Pool<Postgres>,
) -> Result<UserCreationResult, UserCreationResult> {
    // checking length of provided data
    if let Err(err) = validate_data(&user.username(), &user.email(), &user.password()) {
        // return an error if occurred
        return Err(err);
    }

    // searching for duplicates
    if let Err(err) = check_for_duplicates(&user.username(), &user.email(), &pool).await {
        // return an error if occurred
        return Err(err);
    }

    // hashing password for security purposes (using SHA-512 algorithm)
    let hashed_password = sha512_password(user.password());

    // if everything is correct, then insert new user into table in the database
    // sqlx query is safe (no sql injection is possible)
    let result = sqlx::query_as!(
        User,
        "insert into users(username, password, email) values($1, $2, $3)",
        user.username(),
        hashed_password,
        user.email(),
    )
    .execute(pool)
    .await;

    // checking for errors
    // if they occurred: return an unknown error,
    // because it is unsafe to return errors except listed in UserCreationResult enum
    match result {
        Err(_) => Err(UserCreationResult::UnknownError),
        _ => Ok(UserCreationResult::ValidData),
    }
}

/// unit tests
#[cfg(test)]
mod db_tests {
    use super::*;
    use sqlx::pool::Pool;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Error;
    use std::detect::__is_feature_detected::rdrand;

    #[actix_rt::test]
    async fn username_is_taken() {
        let user = User::new(
            String::from("new_user"),
            String::from("12345678"),
            String::from("some_email@gmail.com"),
        );
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .connect(&dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let res = add_user(&user, &pool).await;
        let res = add_user(&user, &pool).await;

        assert_eq!(res, Err(UserCreationResult::UserNameIsTaken));
    }

    #[actix_rt::test]
    async fn email_is_taken() {
        let user = User::new(
            String::from("some_user12345678"),
            String::from("12345678"),
            String::from("new_user@mail.com"),
        );
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .connect(&dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        // we add call add_user twice, so database 100% already has that user
        let res = add_user(&user, &pool).await;
        let res = add_user(&user, &pool).await;

        assert_eq!(res, Err(UserCreationResult::EmailIsTaken));
    }

    #[actix_rt::test]
    async fn password_is_short() {
        let user = User::new(
            String::from("some_user"),
            String::from("123"),
            String::from("new_user@mail.com"),
        );
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .connect(&dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        // we add call add_user twice, so database 100% already has that user
        let res = add_user(&user, &pool).await;
        let res = add_user(&user, &pool).await;

        assert_eq!(res, Err(UserCreationResult::TooShortPassword));
    }

    #[actix_rt::test]
    async fn email_is_long() {
        let user = User::new(
            String::from("some_user"),
            String::from("12345678"),
            // max email length is 255
            std::iter::repeat("0").take(256).collect::<String>(),
        );
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .connect(&dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        // we add call add_user twice, so database 100% already has that user
        let res = add_user(&user, &pool).await;
        let res = add_user(&user, &pool).await;

        assert_eq!(res, Err(UserCreationResult::TooLongEmail));
    }

    #[actix_rt::test]
    async fn sql_injection() {
        let user = User::new(
            String::from("'DROP TABLE users;--"),
            String::from("12345678"),
            String::from("mamkin_hacker@protonmail.com"),
        );

        let pool: Pool<Postgres> = PgPoolOptions::new()
            .connect(&dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        // shouldn't panic
        add_user(&user, &pool).await;
    }
}
