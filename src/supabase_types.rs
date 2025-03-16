use serde::{Serialize, Deserialize};

use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::SupabaseClient;
use crate::query::QueryBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoFinancialInstitution {
    pub id: Uuid,
    pub name: Option<String>,
    pub status: Option<String>,
    pub deprecated: Option<bool>,
    pub country: Option<String>,
    pub bic: Option<String>,
    pub bulk_payments_enabled: Option<bool>,
    pub bulk_payments_product_types: Option<String>,
    pub future_dated_payments_allowed: Option<bool>,
    pub logo_url: Option<String>,
    pub maintenance_from: Option<String>,
    pub maintenance_to: Option<String>,
    pub maintenance_type: Option<String>,
    pub payments_enabled: Option<bool>,
    pub payments_product_types: Option<String>,
    pub periodic_payments_enabled: Option<bool>,
    pub periodic_payments_product_types: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub shared_brand_name: Option<String>,
    pub shared_brand_reference: Option<String>,
    pub time_zone: Option<String>,
}

impl PontoFinancialInstitution {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "name",
            "status",
            "deprecated",
            "country",
            "bic",
            "bulk_payments_enabled",
            "bulk_payments_product_types",
            "future_dated_payments_allowed",
            "logo_url",
            "maintenance_from",
            "maintenance_to",
            "maintenance_type",
            "payments_enabled",
            "payments_product_types",
            "periodic_payments_enabled",
            "periodic_payments_product_types",
            "primary_color",
            "secondary_color",
            "shared_brand_name",
            "shared_brand_reference",
            "time_zone",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | name | String | Yes |
    /// | status | String | Yes |
    /// | deprecated | bool | Yes |
    /// | country | String | Yes |
    /// | bic | String | Yes |
    /// | bulk_payments_enabled | bool | Yes |
    /// | bulk_payments_product_types | String | Yes |
    /// | future_dated_payments_allowed | bool | Yes |
    /// | logo_url | String | Yes |
    /// | maintenance_from | String | Yes |
    /// | maintenance_to | String | Yes |
    /// | maintenance_type | String | Yes |
    /// | payments_enabled | bool | Yes |
    /// | payments_product_types | String | Yes |
    /// | periodic_payments_enabled | bool | Yes |
    /// | periodic_payments_product_types | String | Yes |
    /// | primary_color | String | Yes |
    /// | secondary_color | String | Yes |
    /// | shared_brand_name | String | Yes |
    /// | shared_brand_reference | String | Yes |
    /// | time_zone | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_financial_institution(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_financial_institution")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub email_id: Uuid,
    pub subject: Option<String>,
    pub recipient_email: Option<String>,
    pub sender_email: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub time: Option<Decimal>,
    pub user_id: Option<String>,
    pub email_code: Option<String>,
}

impl Emails {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "email_id",
            "subject",
            "recipient_email",
            "sender_email",
            "body",
            "status",
            "time",
            "user_id",
            "email_code",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | email_id | Uuid | No |
    /// | subject | String | Yes |
    /// | recipient_email | String | Yes |
    /// | sender_email | String | Yes |
    /// | body | String | Yes |
    /// | status | String | Yes |
    /// | time | Decimal | Yes |
    /// | user_id | String | Yes |
    /// | email_code | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_emails(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "emails")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoOnboardingDetails {
    pub id: Uuid,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_street_address: Option<String>,
    pub enterprise_number: Option<String>,
    pub vat_number: Option<String>,
    pub phone_number: Option<String>,
    pub automatic_submission_on_completed_forms: Option<bool>,
    pub initial_financial_institution_id: Option<String>,
    pub organization_name: Option<String>,
    pub organization_type: Option<String>,
    pub preferred_otp_method: Option<String>,
    pub requested_organisation_id: Option<String>,
    pub partner_reference: Option<String>,
    pub created_at_unixtime: Option<Decimal>,
}

impl PontoOnboardingDetails {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "email",
            "first_name",
            "last_name",
            "address_city",
            "address_country",
            "address_postal_code",
            "address_street_address",
            "enterprise_number",
            "vat_number",
            "phone_number",
            "automatic_submission_on_completed_forms",
            "initial_financial_institution_id",
            "organization_name",
            "organization_type",
            "preferred_otp_method",
            "requested_organisation_id",
            "partner_reference",
            "created_at_unixtime",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | email | String | Yes |
    /// | first_name | String | Yes |
    /// | last_name | String | Yes |
    /// | address_city | String | Yes |
    /// | address_country | String | Yes |
    /// | address_postal_code | String | Yes |
    /// | address_street_address | String | Yes |
    /// | enterprise_number | String | Yes |
    /// | vat_number | String | Yes |
    /// | phone_number | String | Yes |
    /// | automatic_submission_on_completed_forms | bool | Yes |
    /// | initial_financial_institution_id | String | Yes |
    /// | organization_name | String | Yes |
    /// | organization_type | String | Yes |
    /// | preferred_otp_method | String | Yes |
    /// | requested_organisation_id | String | Yes |
    /// | partner_reference | String | Yes |
    /// | created_at_unixtime | Decimal | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_onboarding_details(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_onboarding_details")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoAccount {
    pub id: Uuid,
    pub deprecated: Option<bool>,
    pub description: Option<String>,
    pub product: Option<String>,
    pub reference: Option<String>,
    pub currency: Option<String>,
    pub subtype: Option<String>,
    pub available_balance: Option<Decimal>,
    pub available_balance_changed_at: Option<NaiveDateTime>,
    pub available_balance_reference_date: Option<NaiveDateTime>,
    pub current_balance: Option<Decimal>,
    pub current_balance_changed_at: Option<NaiveDateTime>,
    pub current_balance_reference_date: Option<NaiveDateTime>,
    pub holder_name: Option<String>,
    pub reference_type: Option<String>,
    pub authorization_expiration_expected_at: Option<NaiveDateTime>,
    pub authorized_at: Option<NaiveDateTime>,
    pub available_balance_variation_observed_at: Option<NaiveDateTime>,
    pub current_balance_variation_observed_at: Option<NaiveDateTime>,
    pub internal_reference: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub user_id: String,
    pub organization_id: Uuid,
    pub financial_institution_id: Option<Uuid>,
    pub revoked: bool,
}

impl PontoAccount {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "deprecated",
            "description",
            "product",
            "reference",
            "currency",
            "subtype",
            "available_balance",
            "available_balance_changed_at",
            "available_balance_reference_date",
            "current_balance",
            "current_balance_changed_at",
            "current_balance_reference_date",
            "holder_name",
            "reference_type",
            "authorization_expiration_expected_at",
            "authorized_at",
            "available_balance_variation_observed_at",
            "current_balance_variation_observed_at",
            "internal_reference",
            "created_at",
            "user_id",
            "organization_id",
            "financial_institution_id",
            "revoked",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | deprecated | bool | Yes |
    /// | description | String | Yes |
    /// | product | String | Yes |
    /// | reference | String | Yes |
    /// | currency | String | Yes |
    /// | subtype | String | Yes |
    /// | available_balance | Decimal | Yes |
    /// | available_balance_changed_at | NaiveDateTime | Yes |
    /// | available_balance_reference_date | NaiveDateTime | Yes |
    /// | current_balance | Decimal | Yes |
    /// | current_balance_changed_at | NaiveDateTime | Yes |
    /// | current_balance_reference_date | NaiveDateTime | Yes |
    /// | holder_name | String | Yes |
    /// | reference_type | String | Yes |
    /// | authorization_expiration_expected_at | NaiveDateTime | Yes |
    /// | authorized_at | NaiveDateTime | Yes |
    /// | available_balance_variation_observed_at | NaiveDateTime | Yes |
    /// | current_balance_variation_observed_at | NaiveDateTime | Yes |
    /// | internal_reference | String | Yes |
    /// | created_at | NaiveDateTime | Yes |
    /// | user_id | String | No |
    /// | organization_id | Uuid | No |
    /// | financial_institution_id | Uuid | Yes |
    /// | revoked | bool | No |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_account(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_account")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scopes {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub scope_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Scopes {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "scope_id",
            "name",
            "description",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | scope_id | String | No |
    /// | name | String | Yes |
    /// | description | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_scopes(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "scopes")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoTokens {
    pub created_at: Decimal,
    pub expires_at: Option<Decimal>,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

impl PontoTokens {
    pub fn columns() -> &'static [&'static str] {
        &[
            "created_at",
            "expires_at",
            "user_id",
            "access_token",
            "refresh_token",
            "token_type",
            "scope",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | created_at | Decimal | No |
    /// | expires_at | Decimal | Yes |
    /// | user_id | String | No |
    /// | access_token | String | No |
    /// | refresh_token | String | Yes |
    /// | token_type | String | Yes |
    /// | scope | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_tokens(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_tokens")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessGroups {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub organization_id: Option<String>,
    pub user_id: Option<String>,
    pub role: Option<String>,
    pub auth_index: Option<Decimal>,
    pub pending: Option<bool>,
    pub invite_code: Option<String>,
    pub invited_by_user_id: Option<String>,
    pub time: Option<Decimal>,
    pub frozen: Option<bool>,
    pub flagged: Option<bool>,
    pub status: Option<String>,
    pub company_id: Option<Uuid>,
    pub exp_date: Option<Decimal>,
    pub scopes: Option<String>,
}

impl AccessGroups {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "organization_id",
            "user_id",
            "role",
            "auth_index",
            "pending",
            "invite_code",
            "invited_by_user_id",
            "time",
            "frozen",
            "flagged",
            "status",
            "company_id",
            "exp_date",
            "scopes",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | organization_id | String | Yes |
    /// | user_id | String | Yes |
    /// | role | String | Yes |
    /// | auth_index | Decimal | Yes |
    /// | pending | bool | Yes |
    /// | invite_code | String | Yes |
    /// | invited_by_user_id | String | Yes |
    /// | time | Decimal | Yes |
    /// | frozen | bool | Yes |
    /// | flagged | bool | Yes |
    /// | status | String | Yes |
    /// | company_id | Uuid | Yes |
    /// | exp_date | Decimal | Yes |
    /// | scopes | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_access_groups(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "access_groups")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organizations {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub name: Option<String>,
    pub organization_id: String,
    pub avatar: Option<String>,
    pub tier: Option<String>,
    pub bio: Option<String>,
    pub verified: Option<bool>,
    pub billing_address_line_1: Option<String>,
    pub billing_address_line_2: Option<String>,
    pub billing_city: Option<String>,
    pub billing_country: Option<String>,
    pub billing_postal_code: Option<String>,
    pub billing_state: Option<String>,
    pub billing_company_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_invoice_language: Option<String>,
}

impl Organizations {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "name",
            "organization_id",
            "avatar",
            "tier",
            "bio",
            "verified",
            "billing_address_line_1",
            "billing_address_line_2",
            "billing_city",
            "billing_country",
            "billing_postal_code",
            "billing_state",
            "billing_company_name",
            "billing_email",
            "billing_invoice_language",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | name | String | Yes |
    /// | organization_id | String | No |
    /// | avatar | String | Yes |
    /// | tier | String | Yes |
    /// | bio | String | Yes |
    /// | verified | bool | Yes |
    /// | billing_address_line_1 | String | Yes |
    /// | billing_address_line_2 | String | Yes |
    /// | billing_city | String | Yes |
    /// | billing_country | String | Yes |
    /// | billing_postal_code | String | Yes |
    /// | billing_state | String | Yes |
    /// | billing_company_name | String | Yes |
    /// | billing_email | String | Yes |
    /// | billing_invoice_language | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_organizations(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "organizations")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoWebhookEvents {
    pub id: Uuid,
    pub resource: String,
    pub event: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub synchronization_subtype: Option<String>,
    pub account_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub synchronization_id: Option<Uuid>,
    pub payment_request_id: Option<Uuid>,
    pub count: Option<Decimal>,
}

impl PontoWebhookEvents {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "resource",
            "event",
            "created_at",
            "synchronization_subtype",
            "account_id",
            "organization_id",
            "synchronization_id",
            "payment_request_id",
            "count",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | resource | String | No |
    /// | event | String | Yes |
    /// | created_at | NaiveDateTime | Yes |
    /// | synchronization_subtype | String | Yes |
    /// | account_id | Uuid | Yes |
    /// | organization_id | Uuid | Yes |
    /// | synchronization_id | Uuid | Yes |
    /// | payment_request_id | Uuid | Yes |
    /// | count | Decimal | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_webhook_events(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_webhook_events")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: String,
    pub email: String,
    pub avatar: Option<String>,
    pub onboarded: Option<bool>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Option<String>,
    pub auth_index: Option<Decimal>,
    pub domain: Option<String>,
    pub mfa_setup: Option<bool>,
    pub last_mfa_challenge_at: Option<Decimal>,
    pub awaiting_mfa_challenge: Option<bool>,
    pub display_name: Option<String>,
    pub default_organization: Option<String>,
    pub default_scope: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub totp_secret: Option<String>,
    pub notification_count: Option<Decimal>,
    pub theme: Option<String>,
    pub is_verified: Option<bool>,
    pub is_frozen: Option<bool>,
    pub is_flagged: Option<bool>,
    pub phone_number: Option<String>,
    pub username: Option<String>,
    pub number_format: Option<String>,
}

impl Users {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "email",
            "avatar",
            "onboarded",
            "country",
            "city",
            "address_line_1",
            "address_line_2",
            "postal_code",
            "state",
            "full_name",
            "first_name",
            "last_name",
            "status",
            "auth_index",
            "domain",
            "mfa_setup",
            "last_mfa_challenge_at",
            "awaiting_mfa_challenge",
            "display_name",
            "default_organization",
            "default_scope",
            "mfa_enabled",
            "totp_secret",
            "notification_count",
            "theme",
            "is_verified",
            "is_frozen",
            "is_flagged",
            "phone_number",
            "username",
            "number_format",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | String | No |
    /// | email | String | No |
    /// | avatar | String | Yes |
    /// | onboarded | bool | Yes |
    /// | country | String | Yes |
    /// | city | String | Yes |
    /// | address_line_1 | String | Yes |
    /// | address_line_2 | String | Yes |
    /// | postal_code | String | Yes |
    /// | state | String | Yes |
    /// | full_name | String | Yes |
    /// | first_name | String | Yes |
    /// | last_name | String | Yes |
    /// | status | String | Yes |
    /// | auth_index | Decimal | Yes |
    /// | domain | String | Yes |
    /// | mfa_setup | bool | Yes |
    /// | last_mfa_challenge_at | Decimal | Yes |
    /// | awaiting_mfa_challenge | bool | Yes |
    /// | display_name | String | Yes |
    /// | default_organization | String | Yes |
    /// | default_scope | String | Yes |
    /// | mfa_enabled | bool | Yes |
    /// | totp_secret | String | Yes |
    /// | notification_count | Decimal | Yes |
    /// | theme | String | Yes |
    /// | is_verified | bool | Yes |
    /// | is_frozen | bool | Yes |
    /// | is_flagged | bool | Yes |
    /// | phone_number | String | Yes |
    /// | username | String | Yes |
    /// | number_format | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_users(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "users")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankingAccounts {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub account_id: Uuid,
    pub added_by_user_id: Option<String>,
    pub company_id: Option<Uuid>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub card_number: Option<String>,
    pub expiry_date: Option<String>,
    pub bank: Option<String>,
    pub status: Option<String>,
    pub hash: Option<String>,
}

impl BankingAccounts {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "account_id",
            "added_by_user_id",
            "company_id",
            "name",
            "type",
            "card_number",
            "expiry_date",
            "bank",
            "status",
            "hash",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | account_id | Uuid | No |
    /// | added_by_user_id | String | Yes |
    /// | company_id | Uuid | Yes |
    /// | name | String | Yes |
    /// | type | String | Yes |
    /// | card_number | String | Yes |
    /// | expiry_date | String | Yes |
    /// | bank | String | Yes |
    /// | status | String | Yes |
    /// | hash | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_banking_accounts(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "banking_accounts")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLogs {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub invoice_id: Option<Uuid>,
    pub user_id: Option<String>,
    pub company_id: Option<Uuid>,
    pub message: Option<String>,
    pub action: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub view_key: Option<String>,
    pub invoice_url: Option<String>,
    pub time: Option<Decimal>,
    pub display_name: Option<String>,
    pub log_id: Uuid,
}

impl InvoiceLogs {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "invoice_id",
            "user_id",
            "company_id",
            "message",
            "action",
            "ip",
            "user_agent",
            "view_key",
            "invoice_url",
            "time",
            "display_name",
            "log_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | invoice_id | Uuid | Yes |
    /// | user_id | String | Yes |
    /// | company_id | Uuid | Yes |
    /// | message | String | Yes |
    /// | action | String | Yes |
    /// | ip | String | Yes |
    /// | user_agent | String | Yes |
    /// | view_key | String | Yes |
    /// | invoice_url | String | Yes |
    /// | time | Decimal | Yes |
    /// | display_name | String | Yes |
    /// | log_id | Uuid | No |
    #[cfg(feature = "nightly")]
    pub fn select_invoice_logs(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "invoice_logs")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoices {
    pub id: i64,
    pub recipient_name: String,
    pub recipient_email: String,
    pub invoice_nr: String,
    pub recipient_postal_code: Option<String>,
    pub recipient_phone: Option<String>,
    pub recipient_first_name: Option<String>,
    pub recipient_last_name: Option<String>,
    pub recipient_address: Option<String>,
    pub recipient_country: Option<String>,
    pub due_date: Decimal,
    pub created_at: NaiveDateTime,
    pub currency: Option<String>,
    pub contact: Option<Uuid>,
    pub status: Option<String>,
    pub items: Option<String>,
    pub owner_id: Option<String>,
    pub recipient_company: Option<String>,
    pub recipient_company_id: Option<Uuid>,
    pub invoice_id: Uuid,
    pub paid: Option<bool>,
    pub payment_instructions: Option<String>,
    pub pay_button_href: Option<String>,
    pub author_name: Option<String>,
    pub link_tos: Option<String>,
    pub link_privacy_policy: Option<String>,
    pub memo: Option<String>,
    pub author_email: Option<String>,
    pub author_postal_code: Option<String>,
    pub author_phone: Option<String>,
    pub author_first_name: Option<String>,
    pub author_last_name: Option<String>,
    pub author_address: Option<String>,
    pub author_country: Option<String>,
    pub author_company_id: Option<String>,
    pub author_vat_id: Option<String>,
    pub author_tax_id: Option<String>,
    pub author_kvk: Option<String>,
    pub recipient_kvk: Option<String>,
    pub recipient_tax_id: Option<String>,
    pub recipient_vat_id: Option<String>,
    pub amount: Option<Decimal>,
    pub amount_paid: Option<Decimal>,
    pub amount_remaining: Option<Decimal>,
    pub paid_at: Option<Decimal>,
    pub payment_method: Option<String>,
    pub number_format: Option<String>,
    pub email_sent: Option<bool>,
    pub email_id: Option<Uuid>,
    pub email_sent_at: Option<Decimal>,
    pub times_opened: Option<Decimal>,
    pub times_opened_unique: Option<Decimal>,
    pub company_logo: Option<String>,
    pub company_logo_href: Option<String>,
    pub discount_code: Option<String>,
    pub discount_total: Option<Decimal>,
    pub discount: Option<bool>,
    pub shipping_rate: Option<bool>,
    pub shipping_total: Option<Decimal>,
    pub send_as_email: Option<bool>,
    pub subscription: Option<bool>,
    pub subscription_id: Option<String>,
    pub relation_hash: Option<String>,
    pub scheduled_email_send_at: Option<Decimal>,
    pub descriptor_global: Option<String>,
    pub descriptor_relation: Option<String>,
    pub number_global: Option<Decimal>,
    pub number_relation: Option<Decimal>,
    pub pay_button_text: Option<String>,
    pub url: Option<String>,
    pub brand_color: Option<String>,
}

impl Invoices {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "recipient_name",
            "recipient_email",
            "invoice_nr",
            "recipient_postal_code",
            "recipient_phone",
            "recipient_first_name",
            "recipient_last_name",
            "recipient_address",
            "recipient_country",
            "due_date",
            "created_at",
            "currency",
            "contact",
            "status",
            "items",
            "owner_id",
            "recipient_company",
            "recipient_company_id",
            "invoice_id",
            "paid",
            "payment_instructions",
            "pay_button_href",
            "author_name",
            "link_tos",
            "link_privacy_policy",
            "memo",
            "author_email",
            "author_postal_code",
            "author_phone",
            "author_first_name",
            "author_last_name",
            "author_address",
            "author_country",
            "author_company_id",
            "author_vat_id",
            "author_tax_id",
            "author_kvk",
            "recipient_kvk",
            "recipient_tax_id",
            "recipient_vat_id",
            "amount",
            "amount_paid",
            "amount_remaining",
            "paid_at",
            "payment_method",
            "number_format",
            "email_sent",
            "email_id",
            "email_sent_at",
            "times_opened",
            "times_opened_unique",
            "company_logo",
            "company_logo_href",
            "discount_code",
            "discount_total",
            "discount",
            "shipping_rate",
            "shipping_total",
            "send_as_email",
            "subscription",
            "subscription_id",
            "relation_hash",
            "scheduled_email_send_at",
            "descriptor_global",
            "descriptor_relation",
            "number_global",
            "number_relation",
            "pay_button_text",
            "url",
            "brand_color",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | recipient_name | String | No |
    /// | recipient_email | String | No |
    /// | invoice_nr | String | No |
    /// | recipient_postal_code | String | Yes |
    /// | recipient_phone | String | Yes |
    /// | recipient_first_name | String | Yes |
    /// | recipient_last_name | String | Yes |
    /// | recipient_address | String | Yes |
    /// | recipient_country | String | Yes |
    /// | due_date | Decimal | No |
    /// | created_at | NaiveDateTime | No |
    /// | currency | String | Yes |
    /// | contact | Uuid | Yes |
    /// | status | String | Yes |
    /// | items | String | Yes |
    /// | owner_id | String | Yes |
    /// | recipient_company | String | Yes |
    /// | recipient_company_id | Uuid | Yes |
    /// | invoice_id | Uuid | No |
    /// | paid | bool | Yes |
    /// | payment_instructions | String | Yes |
    /// | pay_button_href | String | Yes |
    /// | author_name | String | Yes |
    /// | link_tos | String | Yes |
    /// | link_privacy_policy | String | Yes |
    /// | memo | String | Yes |
    /// | author_email | String | Yes |
    /// | author_postal_code | String | Yes |
    /// | author_phone | String | Yes |
    /// | author_first_name | String | Yes |
    /// | author_last_name | String | Yes |
    /// | author_address | String | Yes |
    /// | author_country | String | Yes |
    /// | author_company_id | String | Yes |
    /// | author_vat_id | String | Yes |
    /// | author_tax_id | String | Yes |
    /// | author_kvk | String | Yes |
    /// | recipient_kvk | String | Yes |
    /// | recipient_tax_id | String | Yes |
    /// | recipient_vat_id | String | Yes |
    /// | amount | Decimal | Yes |
    /// | amount_paid | Decimal | Yes |
    /// | amount_remaining | Decimal | Yes |
    /// | paid_at | Decimal | Yes |
    /// | payment_method | String | Yes |
    /// | number_format | String | Yes |
    /// | email_sent | bool | Yes |
    /// | email_id | Uuid | Yes |
    /// | email_sent_at | Decimal | Yes |
    /// | times_opened | Decimal | Yes |
    /// | times_opened_unique | Decimal | Yes |
    /// | company_logo | String | Yes |
    /// | company_logo_href | String | Yes |
    /// | discount_code | String | Yes |
    /// | discount_total | Decimal | Yes |
    /// | discount | bool | Yes |
    /// | shipping_rate | bool | Yes |
    /// | shipping_total | Decimal | Yes |
    /// | send_as_email | bool | Yes |
    /// | subscription | bool | Yes |
    /// | subscription_id | String | Yes |
    /// | relation_hash | String | Yes |
    /// | scheduled_email_send_at | Decimal | Yes |
    /// | descriptor_global | String | Yes |
    /// | descriptor_relation | String | Yes |
    /// | number_global | Decimal | Yes |
    /// | number_relation | Decimal | Yes |
    /// | pay_button_text | String | Yes |
    /// | url | String | Yes |
    /// | brand_color | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_invoices(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "invoices")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub author_user_id: Option<String>,
    pub note_id: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub content: Option<String>,
}

impl Notes {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "author_user_id",
            "note_id",
            "resource_type",
            "resource_id",
            "content",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | author_user_id | String | Yes |
    /// | note_id | String | No |
    /// | resource_type | String | Yes |
    /// | resource_id | String | Yes |
    /// | content | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_notes(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "notes")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiLogs {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub method: Option<String>,
    pub http_code: Option<Decimal>,
    pub body: Option<String>,
    pub message: Option<String>,
    pub time: Option<Decimal>,
    pub request: Option<String>,
    pub user_id: Option<String>,
    pub server: Option<String>,
    pub domain: Option<String>,
    pub requester_id: Option<String>,
    pub api_version: Option<String>,
    pub response: Option<String>,
    pub log_message: Option<String>,
    pub response_data: Option<String>,
    pub json_response: Option<String>,
}

impl ApiLogs {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "method",
            "http_code",
            "body",
            "message",
            "time",
            "request",
            "user_id",
            "server",
            "domain",
            "requester_id",
            "api_version",
            "response",
            "log_message",
            "response_data",
            "json_response",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | method | String | Yes |
    /// | http_code | Decimal | Yes |
    /// | body | String | Yes |
    /// | message | String | Yes |
    /// | time | Decimal | Yes |
    /// | request | String | Yes |
    /// | user_id | String | Yes |
    /// | server | String | Yes |
    /// | domain | String | Yes |
    /// | requester_id | String | Yes |
    /// | api_version | String | Yes |
    /// | response | String | Yes |
    /// | log_message | String | Yes |
    /// | response_data | String | Yes |
    /// | json_response | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_api_logs(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "api_logs")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    pub id: i64,
    pub contact_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub company_name: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub iban: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub owner_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub address: Option<String>,
}

impl Contacts {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "contact_id",
            "first_name",
            "last_name",
            "company_name",
            "email",
            "phone",
            "postal_code",
            "city",
            "country",
            "iban",
            "type",
            "owner_id",
            "created_at",
            "address",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | contact_id | Uuid | No |
    /// | first_name | String | No |
    /// | last_name | String | No |
    /// | company_name | String | Yes |
    /// | email | String | No |
    /// | phone | String | Yes |
    /// | postal_code | String | Yes |
    /// | city | String | Yes |
    /// | country | String | Yes |
    /// | iban | String | Yes |
    /// | type | String | Yes |
    /// | owner_id | String | Yes |
    /// | created_at | NaiveDateTime | No |
    /// | address | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_contacts(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "contacts")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    pub id: i64,
    pub product_id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub owner_id: String,
    pub tax_id: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Products {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "product_id",
            "name",
            "price",
            "owner_id",
            "tax_id",
            "created_at",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | product_id | Uuid | No |
    /// | name | String | No |
    /// | price | Decimal | No |
    /// | owner_id | String | No |
    /// | tax_id | String | Yes |
    /// | created_at | NaiveDateTime | No |
    #[cfg(feature = "nightly")]
    pub fn select_products(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "products")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpIdentities {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
    pub ipv4: Option<String>,
    pub os: Option<String>,
    pub user_agent: Option<String>,
    pub resolution: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub locale: Option<String>,
    pub last_sign_in: Option<Decimal>,
    pub first_seen: Option<Decimal>,
    pub provider: Option<String>,
    pub hostname: Option<String>,
    pub session_id: Option<String>,
}

impl IpIdentities {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "ipv4",
            "os",
            "user_agent",
            "resolution",
            "country",
            "state",
            "postal_code",
            "locale",
            "last_sign_in",
            "first_seen",
            "provider",
            "hostname",
            "session_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | String | Yes |
    /// | ipv4 | String | Yes |
    /// | os | String | Yes |
    /// | user_agent | String | Yes |
    /// | resolution | String | Yes |
    /// | country | String | Yes |
    /// | state | String | Yes |
    /// | postal_code | String | Yes |
    /// | locale | String | Yes |
    /// | last_sign_in | Decimal | Yes |
    /// | first_seen | Decimal | Yes |
    /// | provider | String | Yes |
    /// | hostname | String | Yes |
    /// | session_id | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ip_identities(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ip_identities")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCategories {
    pub id: i64,
    pub label: String,
    pub company_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub time: Option<Decimal>,
    pub author_user_id: String,
    pub icon: Option<String>,
    pub transaction_category_id: Uuid,
}

impl TransactionCategories {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "label",
            "company_id",
            "created_at",
            "time",
            "author_user_id",
            "icon",
            "transaction_category_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | label | String | No |
    /// | company_id | Uuid | Yes |
    /// | created_at | NaiveDateTime | No |
    /// | time | Decimal | Yes |
    /// | author_user_id | String | No |
    /// | icon | String | Yes |
    /// | transaction_category_id | Uuid | No |
    #[cfg(feature = "nightly")]
    pub fn select_transaction_categories(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "transaction_categories")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersFlagged {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
    pub flagger_user_id: Option<String>,
    pub reason: Option<String>,
    pub time: Option<Decimal>,
}

impl UsersFlagged {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "flagger_user_id",
            "reason",
            "time",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | String | Yes |
    /// | flagger_user_id | String | Yes |
    /// | reason | String | Yes |
    /// | time | Decimal | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_users_flagged(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "users_flagged")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Imports {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub file_path: Option<String>,
    pub server: Option<String>,
    pub host: Option<String>,
    pub time: Option<Decimal>,
    pub user_id: Option<String>,
    pub filename: Option<String>,
    pub extension: Option<String>,
    pub mime: Option<String>,
    pub proxy_url: Option<String>,
}

impl Imports {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "file_path",
            "server",
            "host",
            "time",
            "user_id",
            "filename",
            "extension",
            "mime",
            "proxy_url",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | file_path | String | Yes |
    /// | server | String | Yes |
    /// | host | String | Yes |
    /// | time | Decimal | Yes |
    /// | user_id | String | Yes |
    /// | filename | String | Yes |
    /// | extension | String | Yes |
    /// | mime | String | Yes |
    /// | proxy_url | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_imports(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "imports")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub description: Option<String>,
    pub currency: Option<String>,
    pub digest: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub additional_information: Option<String>,
    pub bank_transaction_code: Option<String>,
    pub card_reference: Option<String>,
    pub card_reference_type: Option<String>,
    pub counterpart_name: Option<String>,
    pub counterpart_reference: Option<String>,
    pub creditor_id: Option<String>,
    pub end_to_end_id: Option<String>,
    pub execution_date: Option<NaiveDateTime>,
    pub mandate_id: Option<String>,
    pub proprietary_bank_transaction_code: Option<String>,
    pub purpose_code: Option<String>,
    pub remittance_information: Option<String>,
    pub remittance_information_type: Option<String>,
    pub value_date: Option<NaiveDateTime>,
    pub internal_reference: Option<String>,
    pub user_id: String,
    pub created_at_unixtime_milli: Decimal,
    pub updated_at_unixtime_milli: Decimal,
    pub execution_date_unixtime_milli: Option<Decimal>,
}

impl PontoTransaction {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "account_id",
            "description",
            "currency",
            "digest",
            "created_at",
            "updated_at",
            "amount",
            "fee",
            "additional_information",
            "bank_transaction_code",
            "card_reference",
            "card_reference_type",
            "counterpart_name",
            "counterpart_reference",
            "creditor_id",
            "end_to_end_id",
            "execution_date",
            "mandate_id",
            "proprietary_bank_transaction_code",
            "purpose_code",
            "remittance_information",
            "remittance_information_type",
            "value_date",
            "internal_reference",
            "user_id",
            "created_at_unixtime_milli",
            "updated_at_unixtime_milli",
            "execution_date_unixtime_milli",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | account_id | Uuid | No |
    /// | description | String | Yes |
    /// | currency | String | Yes |
    /// | digest | String | Yes |
    /// | created_at | NaiveDateTime | Yes |
    /// | updated_at | NaiveDateTime | Yes |
    /// | amount | Decimal | Yes |
    /// | fee | Decimal | Yes |
    /// | additional_information | String | Yes |
    /// | bank_transaction_code | String | Yes |
    /// | card_reference | String | Yes |
    /// | card_reference_type | String | Yes |
    /// | counterpart_name | String | Yes |
    /// | counterpart_reference | String | Yes |
    /// | creditor_id | String | Yes |
    /// | end_to_end_id | String | Yes |
    /// | execution_date | NaiveDateTime | Yes |
    /// | mandate_id | String | Yes |
    /// | proprietary_bank_transaction_code | String | Yes |
    /// | purpose_code | String | Yes |
    /// | remittance_information | String | Yes |
    /// | remittance_information_type | String | Yes |
    /// | value_date | NaiveDateTime | Yes |
    /// | internal_reference | String | Yes |
    /// | user_id | String | No |
    /// | created_at_unixtime_milli | Decimal | No |
    /// | updated_at_unixtime_milli | Decimal | No |
    /// | execution_date_unixtime_milli | Decimal | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_transaction(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_transaction")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLog {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub directive: Option<String>,
    pub time: Option<Decimal>,
    pub path: Option<String>,
    pub content: Option<String>,
    pub server: Option<String>,
    pub api_version: Option<String>,
}

impl EventLog {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "directive",
            "time",
            "path",
            "content",
            "server",
            "api_version",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | directive | String | Yes |
    /// | time | Decimal | Yes |
    /// | path | String | Yes |
    /// | content | String | Yes |
    /// | server | String | Yes |
    /// | api_version | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_event_log(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "event_log")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoUserInfo {
    pub name: Option<String>,
    pub sub: Uuid,
    pub payments_activated: Option<bool>,
    pub payments_activation_requested: Option<bool>,
    pub payment_requests_activated: Option<bool>,
    pub payment_requests_activation_requested: Option<bool>,
    pub onboarding_complete: Option<bool>,
    pub user_id: String,
    pub blocked: bool,
}

impl PontoUserInfo {
    pub fn columns() -> &'static [&'static str] {
        &[
            "name",
            "sub",
            "payments_activated",
            "payments_activation_requested",
            "payment_requests_activated",
            "payment_requests_activation_requested",
            "onboarding_complete",
            "user_id",
            "blocked",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | name | String | Yes |
    /// | sub | Uuid | No |
    /// | payments_activated | bool | Yes |
    /// | payments_activation_requested | bool | Yes |
    /// | payment_requests_activated | bool | Yes |
    /// | payment_requests_activation_requested | bool | Yes |
    /// | onboarding_complete | bool | Yes |
    /// | user_id | String | No |
    /// | blocked | bool | No |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_user_info(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_user_info")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transactions {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub bank: Option<String>,
    pub iban_recipient: Option<String>,
    pub iban_sender: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub product: Option<String>,
    pub started_date: Option<Decimal>,
    pub completed_date: Option<Decimal>,
    pub description: Option<String>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub currency: Option<String>,
    pub balance: Option<Decimal>,
    pub user_id: Option<String>,
    pub bank_account_id: Option<String>,
    pub record_id: Option<String>,
    pub verified: Option<bool>,
    pub data_author: Option<String>,
    pub money_out: Option<Decimal>,
    pub money_in: Option<Decimal>,
    pub opening_balance: Option<Decimal>,
    pub closing_balance: Option<Decimal>,
    pub user_id_recipient: Option<String>,
    pub user_id_sender: Option<String>,
    pub bic: Option<String>,
    pub document_provider: Option<String>,
    pub transaction_id: Option<String>,
    pub hash: Option<String>,
    pub transaction_type: Option<String>,
    pub company_id: Option<String>,
    pub status: Option<String>,
    pub category: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub memo: Option<String>,
}

impl Transactions {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "bank",
            "iban_recipient",
            "iban_sender",
            "type",
            "product",
            "started_date",
            "completed_date",
            "description",
            "amount",
            "fee",
            "currency",
            "balance",
            "user_id",
            "bank_account_id",
            "record_id",
            "verified",
            "data_author",
            "money_out",
            "money_in",
            "opening_balance",
            "closing_balance",
            "user_id_recipient",
            "user_id_sender",
            "bic",
            "document_provider",
            "transaction_id",
            "hash",
            "transaction_type",
            "company_id",
            "status",
            "category",
            "contact",
            "memo",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | bank | String | Yes |
    /// | iban_recipient | String | Yes |
    /// | iban_sender | String | Yes |
    /// | type | String | Yes |
    /// | product | String | Yes |
    /// | started_date | Decimal | Yes |
    /// | completed_date | Decimal | Yes |
    /// | description | String | Yes |
    /// | amount | Decimal | Yes |
    /// | fee | Decimal | Yes |
    /// | currency | String | Yes |
    /// | balance | Decimal | Yes |
    /// | user_id | String | Yes |
    /// | bank_account_id | String | Yes |
    /// | record_id | String | Yes |
    /// | verified | bool | Yes |
    /// | data_author | String | Yes |
    /// | money_out | Decimal | Yes |
    /// | money_in | Decimal | Yes |
    /// | opening_balance | Decimal | Yes |
    /// | closing_balance | Decimal | Yes |
    /// | user_id_recipient | String | Yes |
    /// | user_id_sender | String | Yes |
    /// | bic | String | Yes |
    /// | document_provider | String | Yes |
    /// | transaction_id | String | Yes |
    /// | hash | String | Yes |
    /// | transaction_type | String | Yes |
    /// | company_id | String | Yes |
    /// | status | String | Yes |
    /// | category | Uuid | Yes |
    /// | contact | Uuid | Yes |
    /// | memo | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_transactions(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "transactions")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Companies {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub company_id: Uuid,
    pub name: Option<String>,
    pub owner_user_id: Option<String>,
}

impl Companies {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "company_id",
            "name",
            "owner_user_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | company_id | Uuid | No |
    /// | name | String | Yes |
    /// | owner_user_id | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_companies(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "companies")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub action: Option<String>,
    pub time: Option<Decimal>,
    pub route: Option<String>,
    pub request: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub author_user_id: Option<String>,
    pub email: Option<String>,
    pub domain: Option<String>,
    pub organization_id: Option<String>,
    pub new_status: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub company_id: Option<String>,
}

impl AuditLog {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "username",
            "action",
            "time",
            "route",
            "request",
            "status",
            "message",
            "author_user_id",
            "email",
            "domain",
            "organization_id",
            "new_status",
            "old_value",
            "new_value",
            "company_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | String | Yes |
    /// | username | String | Yes |
    /// | action | String | Yes |
    /// | time | Decimal | Yes |
    /// | route | String | Yes |
    /// | request | String | Yes |
    /// | status | String | Yes |
    /// | message | String | Yes |
    /// | author_user_id | String | Yes |
    /// | email | String | Yes |
    /// | domain | String | Yes |
    /// | organization_id | String | Yes |
    /// | new_status | String | Yes |
    /// | old_value | String | Yes |
    /// | new_value | String | Yes |
    /// | company_id | String | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_audit_log(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "audit_log")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoSynchronization {
    pub id: Uuid,
    pub status: Option<String>,
    pub errors: Option<String>,
    pub subtype: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub resource_id: Option<Uuid>,
    pub resource_type: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_transactions_link: Option<String>,
    pub created_at_unixtime_milli: Option<Decimal>,
    pub updated_at_unixtime_milli: Option<Decimal>,
}

impl PontoSynchronization {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "status",
            "errors",
            "subtype",
            "created_at",
            "resource_id",
            "resource_type",
            "updated_at",
            "updated_transactions_link",
            "created_at_unixtime_milli",
            "updated_at_unixtime_milli",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | Uuid | No |
    /// | status | String | Yes |
    /// | errors | String | Yes |
    /// | subtype | String | Yes |
    /// | created_at | NaiveDateTime | Yes |
    /// | resource_id | Uuid | Yes |
    /// | resource_type | String | No |
    /// | updated_at | NaiveDateTime | Yes |
    /// | updated_transactions_link | String | Yes |
    /// | created_at_unixtime_milli | Decimal | Yes |
    /// | updated_at_unixtime_milli | Decimal | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_synchronization(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_synchronization")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PontoUsage {
    pub month: String,
    pub organization_id: Uuid,
    pub account_count: Option<Decimal>,
    pub bulk_payment_bundle_count: Option<Decimal>,
    pub bulk_payment_count: Option<Decimal>,
    pub payment_account_count: Option<Decimal>,
    pub payment_count: Option<Decimal>,
    pub total_accounts: Option<Decimal>,
    pub id: i64,
    pub created_at: Option<NaiveDateTime>,
}

impl PontoUsage {
    pub fn columns() -> &'static [&'static str] {
        &[
            "month",
            "organization_id",
            "account_count",
            "bulk_payment_bundle_count",
            "bulk_payment_count",
            "payment_account_count",
            "payment_count",
            "total_accounts",
            "id",
            "created_at",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | month | String | No |
    /// | organization_id | Uuid | No |
    /// | account_count | Decimal | Yes |
    /// | bulk_payment_bundle_count | Decimal | Yes |
    /// | bulk_payment_count | Decimal | Yes |
    /// | payment_account_count | Decimal | Yes |
    /// | payment_count | Decimal | Yes |
    /// | total_accounts | Decimal | Yes |
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_ponto_usage(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_usage")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
    pub read: Option<bool>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub href: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub system_message: Option<bool>,
}

impl Notifications {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "read",
            "title",
            "message",
            "href",
            "type",
            "system_message",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | String | Yes |
    /// | read | bool | Yes |
    /// | title | String | Yes |
    /// | message | String | Yes |
    /// | href | String | Yes |
    /// | type | String | Yes |
    /// | system_message | bool | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_notifications(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "notifications")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sessions {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
    pub updated_at: Option<NaiveDateTime>,
    pub factor_id: Option<Uuid>,
    pub aal: Option<String>,
    pub user_agent: Option<String>,
    pub ip: Option<String>,
    pub tag: Option<String>,
    pub refreshed_at: Option<NaiveDateTime>,
    pub not_after: Option<NaiveDateTime>,
    pub session_id: Option<Uuid>,
}

impl Sessions {
    pub fn columns() -> &'static [&'static str] {
        &[
            "id",
            "created_at",
            "user_id",
            "updated_at",
            "factor_id",
            "aal",
            "user_agent",
            "ip",
            "tag",
            "refreshed_at",
            "not_after",
            "session_id",
        ]
    }
}

impl SupabaseClient {
    /// ### Columns
    /// | Column Name | Type | Optional |
    /// |-------------|------|----------|
    /// | id | i64 | No |
    /// | created_at | NaiveDateTime | No |
    /// | user_id | Uuid | Yes |
    /// | updated_at | NaiveDateTime | Yes |
    /// | factor_id | Uuid | Yes |
    /// | aal | String | Yes |
    /// | user_agent | String | Yes |
    /// | ip | String | Yes |
    /// | tag | String | Yes |
    /// | refreshed_at | NaiveDateTime | Yes |
    /// | not_after | NaiveDateTime | Yes |
    /// | session_id | Uuid | Yes |
    #[cfg(feature = "nightly")]
    pub fn select_sessions(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "sessions")
    }
}

pub const ALL_TABLES: &[&str] = &[
    "ponto_financial_institution",
    "emails",
    "ponto_onboarding_details",
    "ponto_account",
    "scopes",
    "ponto_tokens",
    "access_groups",
    "organizations",
    "ponto_webhook_events",
    "users",
    "banking_accounts",
    "invoice_logs",
    "invoices",
    "notes",
    "api_logs",
    "contacts",
    "products",
    "ip_identities",
    "transaction_categories",
    "users_flagged",
    "imports",
    "ponto_transaction",
    "event_log",
    "ponto_user_info",
    "transactions",
    "companies",
    "audit_log",
    "ponto_synchronization",
    "ponto_usage",
    "notifications",
    "sessions",
];

