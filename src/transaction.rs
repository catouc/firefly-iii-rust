use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use crate::http::Method;
use crate::requests::Request;
use crate::response::*;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Data {
    pub r#type: String,
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_title: Option<String>,
    pub transactions: Vec<Transaction>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Transaction {
    pub amount: String,
    pub date: String,
    pub r#type: TransactionType,
    pub description: String,
    pub source_id: String,
    pub destination_id: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_currency_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_currency_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_currency_decimal_places: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_amount: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_source: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_total: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ct_op: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ct_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_db: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ep: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_batch_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_journal_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_hash_v2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    #[default]
    Withdrawal,
    Deposit,
    Transfer,
    Reconciliation,
    OpeningBalance,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Create {
    pub group_title: Option<String>,
    pub transactions: Vec<CreateTransaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTransaction {
    pub r#type: TransactionType,
    pub date: String,
    pub amount: String,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ct_op: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ct_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_db: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ep: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_ci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_batch_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
}

impl Request for Create {
    type Body = Create;
    type Response = Response<Data>;
    const METHOD: Method = Method::Post;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/transactions").into()
    }
    fn body(&self) -> Option<Self::Body> {
        Some(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Get {
    pub id: String,
}

impl Request for Get {
    type Body = ();
    type Response = Response<Data>;
    const METHOD: Method = Method::Get;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/transactions/{}", self.id).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Delete {
    pub id: String,
}

impl Request for Delete {
    type Body = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::Delete;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/transactions/{}", self.id).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct List {
    pub current_page: u64,
    pub total_pages: u64,
}

impl Request for List {
    type Body = ();
    type Response = PaginatedResponse<Vec<Data>>;
    const METHOD: Method = Method::Get;
    fn endpoint(&self) -> Cow<str> {
        let mut endpoint = "/v1/transactions".to_string();
        endpoint.push_str("?");
        endpoint.push_str(&format!("page={}", self.current_page));
        endpoint.push_str("&");
        endpoint.push_str("limit=50");
        endpoint.into()
    }
}

impl Paginated for List {
    fn set_page(&mut self, page: u64) {
        self.current_page = page;
    }

    fn get_page(&self) -> u64 {
        self.current_page
    }

    fn max_page(&self) -> u64 {
        self.total_pages
    }
}

