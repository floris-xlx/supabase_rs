use serde::{Serialize, Deserialize};

use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::SupabaseClient;
use crate::query::QueryBuilder;

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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ip_identities(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ip_identities")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_onboarding_details(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_onboarding_details")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_products(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "products")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_users(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "users")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_scopes(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "scopes")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_users_flagged(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "users_flagged")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_synchronization(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_synchronization")
    }
}

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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_financial_institution(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_financial_institution")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_webhook_events(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_webhook_events")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_notifications(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "notifications")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_user_info(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_user_info")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_banking_accounts(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "banking_accounts")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_companies(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "companies")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_organizations(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "organizations")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_account(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_account")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_audit_log(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "audit_log")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_transaction(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_transaction")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_access_groups(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "access_groups")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_notes(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "notes")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_usage(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_usage")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_event_log(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "event_log")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_transaction_categories(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "transaction_categories")
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

impl SupabaseClient {
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_contacts(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "contacts")
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

impl SupabaseClient {
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_invoices(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "invoices")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_imports(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "imports")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_sessions(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "sessions")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_transactions(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "transactions")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_ponto_tokens(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "ponto_tokens")
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

impl SupabaseClient {
    #[cfg(feature = "nightly")]
    pub fn select_emails(&self) -> QueryBuilder {
        QueryBuilder::new(self.clone(), "emails")
    }
}

pub const ALL_TABLES: &[&str] = &[
    "ip_identities",
    "ponto_onboarding_details",
    "products",
    "users",
    "scopes",
    "users_flagged",
    "ponto_synchronization",
    "ponto_financial_institution",
    "ponto_webhook_events",
    "notifications",
    "ponto_user_info",
    "banking_accounts",
    "companies",
    "organizations",
    "ponto_account",
    "audit_log",
    "ponto_transaction",
    "access_groups",
    "notes",
    "ponto_usage",
    "event_log",
    "transaction_categories",
    "api_logs",
    "contacts",
    "invoice_logs",
    "invoices",
    "imports",
    "sessions",
    "transactions",
    "ponto_tokens",
    "emails",
];

