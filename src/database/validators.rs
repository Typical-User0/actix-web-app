use super::UserCreationResult;
use sqlx::{Pool, Postgres};

pub fn validate_data(
    username: &str,
    email: &str,
    password: &str,
) -> Result<(), UserCreationResult> {
    if password.len() < 8 {
        return Err(UserCreationResult::TooShortPassword);
    }

    if password.len() > 512 {
        return Err(UserCreationResult::TooLongPassword);
    }

    if email.len() > 255 {
        return Err(UserCreationResult::TooLongEmail);
    }

    if username.len() > 100 {
        return Err(UserCreationResult::TooLongUsername);
    } else {
        Ok(())
    }
}

pub async fn check_for_duplicates(
    username: &str,
    email: &str,
    pool: &Pool<Postgres>,
) -> Result<(), UserCreationResult> {
    // check for username duplicate
    let user_request = sqlx::query!("select from users where username = $1", username)
        .execute(pool)
        .await;

    if let Ok(res) = user_request {
        if res.rows_affected() > 0 {
            return Err(UserCreationResult::UserNameIsTaken);
        }
    } else {
        return Err(UserCreationResult::UnknownError);
    }

    // check for email duplicate
    let email_request = sqlx::query!("select from users where email = $1", email)
        .execute(pool)
        .await;

    if let Ok(res) = email_request {
        if res.rows_affected() > 0 {
            return Err(UserCreationResult::EmailIsTaken);
        }
    } else {
        return Err(UserCreationResult::UnknownError);
    }

    Ok(())
}
