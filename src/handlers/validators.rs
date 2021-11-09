use crate::database::UserCreationResult;

pub fn validate_user_creation_result(
    result: Result<UserCreationResult, UserCreationResult>,
) -> &'static str {
    match result {
        Ok(UserCreationResult::ValidData) => "you successfully signed up",
        Err(UserCreationResult::UserNameIsTaken) => "this username is already taken!",

        // if password is too long or too short
        Err(UserCreationResult::TooLongPassword) | Err(UserCreationResult::TooShortPassword) => {
            "password length should be between 8 and 512 characters"
        }

        Err(UserCreationResult::EmailIsTaken) => "this email is already taken!",
        Err(UserCreationResult::TooLongEmail) => "email length should be no more than 255",
        Err(UserCreationResult::TooLongUsername) => {
            "username length should be no more than 100 characters"
        }
        _ => "unknown error occurred",
    }
}
