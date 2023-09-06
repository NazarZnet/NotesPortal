use core::fmt;
use serde::Serialize;

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

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct FormData {
    pub username: String,
    pub password: String,
}
