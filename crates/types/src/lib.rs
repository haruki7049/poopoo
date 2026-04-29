//! # Poopoo types for the client & server

use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Post {
    pub username: Username,
    pub content: String,
}

impl Post {
    pub fn new(username: Username, content: String) -> Self {
        Self { username, content }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        let inner = value.to_string();
        Self(inner)
    }
}
