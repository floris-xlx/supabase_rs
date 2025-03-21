//! User-related data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a user in the system
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct UserSchema {
    /// Unique identifier for the user
    pub id: Uuid,

    pub aud: String,
    pub role: String,
    // email user's primary contact email. In most cases you can uniquely identify a user by their email address, but not in all cases.
    pub email: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_confirmed_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub invited_at: Option<time::OffsetDateTime>,
    // Phone user's primary contact phone number. In most cases you can uniquely identify a user by their phone number, but not in all cases.
    pub phone: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub phone_confirmed_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub confirmation_sent_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub confirmed_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub recovery_sent_at: Option<time::OffsetDateTime>,
    pub new_email: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_change_sent_at: Option<time::OffsetDateTime>,
    pub new_phone: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub phone_change_sent_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub reauthentication_sent_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_sign_in_at: Option<time::OffsetDateTime>,
    pub user_metadata: Option<HashMap<String, serde_json::Value>>,
    pub app_metadata: Option<HashMap<String, serde_json::Value>>,
    pub factors: Vec<MFAFactorSchema>,
    pub identities: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub banned_until: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<time::OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct MFAFactorSchema {
    factor_type: Option<String>,
    friendly_name: Option<String>,
    id: Option<Uuid>,
    status: Option<MFAFactorStatus>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub enum MFAFactorStatus {
    Verified,
    #[default]
    Unverified,
}
