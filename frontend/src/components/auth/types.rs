use core::fmt;

/// `FormType` indicates the `AuthorizationForm` type
/// Has two variants:
/// * `FormType::SignUp` =>"/auth/signup"
/// * `FormType::LogIn` => "auth/login"
///
/// Implements Display trait to convert `FormType` to URI string.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FormType {
    SignUp,
    LogIn,
}

//make uri from FormType
impl fmt::Display for FormType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogIn => write!(f, "/auth/login"),
            Self::SignUp => write!(f, "/auth/signup"),
        }
    }
}

/// The `FormSettings` struct represents the settings for a form, with a title and a value.
///
/// Properties:
///
/// * `title`: The `title` property is a string that represents the title of the form. It can be used to
/// display a message or a heading to the user.
/// * `value`: The `value` property in the `FormSettings` struct represents the value that will be
/// displayed on the form button. It can be either "Sign Up" or "Log In" depending on the `FormType`
/// variant.
pub struct FormSettings {
    pub title: String,
    pub value: String,
}

impl From<FormType> for FormSettings {
    fn from(value: FormType) -> Self {
        match value {
            FormType::SignUp => FormSettings {
                title: "Create an account!".to_owned(),
                value: "Sign Up".to_owned(),
            },
            FormType::LogIn => FormSettings {
                title: "Welcome back!".to_owned(),
                value: "Log In".to_string(),
            },
        }
    }
}
