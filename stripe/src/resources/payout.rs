use client::Client;
use error::Error;
use params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use resources::Currency;
use serde_qs as qs;

/// The resource representing a Stripe payout.
///
/// For more details see https://stripe.com/docs/api#payout_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Payout {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub arrival_date: Timestamp,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: String,
    pub destination: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<PayoutFailureCode>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub method: PayoutMethod,
    pub source_type: PayoutSourceType,
    pub statement_descriptor: Option<String>,
    pub status: PayoutStatus,
    #[serde(rename = "type")]
    pub payout_type: PayoutType,
}

impl Identifiable for Payout {
    fn id(&self) -> &str {
        &self.id
    }
}

/// An enum representing the possible values of a `PayOut`'s `failure_code` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/failures](https://stripe.com/docs/api/payouts/failures)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutFailureCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
    Declined,
    InsufficientFunds,
    InvalidAccountNumber,
    IncorrectAccountHolderName,
    InvalidCurrency,
    NoAccount,
    UnsupportedCard,
    #[serde(other)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `method` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-method](https://stripe.com/docs/api/payouts/object#payout_object-method)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    Standard,
    Instant,
    #[serde(other)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `source_type` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-source_type](https://stripe.com/docs/api/payouts/object#payout_object-source_type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSourceType {
    Card,
    BankAccount,
    AlipayAccount,
    #[serde(other)]
    Other,
}
/// An enum representing the possible values of a `PayOut`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-status](https://stripe.com/docs/api/payouts/object#payout_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,
    #[serde(other)]
    Other,
}

/// An enum representing the possible values of a `PayOut`'s `payout_type` field.
///
/// For more details see [https://stripe.com/docs/api/payouts/object#payout_object-type](https://stripe.com/docs/api/payouts/object#payout_object-type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
    #[serde(other)]
    Other,
}

/// The set of parameters that can be used when creating a payout object.
///
/// For more details see [https://stripe.com/docs/api/payouts/create](https://stripe.com/docs/api/payouts/create)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PayoutParams {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

/// The set of parameters that can be used when listing payouts.
///
/// For more details see [https://stripe.com/docs/api/payouts/list](https://stripe.com/docs/api/payouts/list)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayoutListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayoutStatus>,
}

impl Payout {
    /// Creates a new payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/create](https://stripe.com/docs/api/payouts/create).
    pub fn create(client: &Client, params: PayoutParams) -> Result<Payout, Error> {
        client.post("/payouts", params)
    }

    /// Retrieves the details of a payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/retrieve](https://stripe.com/docs/api/payouts/retrieve).
    pub fn retrieve(client: &Client, payout_id: &str) -> Result<Payout, Error> {
        client.get(&format!("/payouts/{}", payout_id))
    }

    /// Updates a payout's properties.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/update](https://stripe.com/docs/api/payouts/update).
    pub fn update(
        client: &Client,
        payout_id: &str,
        metadata: Option<Metadata>,
    ) -> Result<Payout, Error> {
        client.post(&format!("/payouts/{}", payout_id), metadata)
    }

    /// List all payouts.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/list](https://stripe.com/docs/api/payouts/list).
    pub fn list(client: &Client, params: PayoutListParams) -> Result<List<Payout>, Error> {
        client.get(&format!("/payouts?{}", qs::to_string(&params)?))
    }

    /// Cancels the payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/cancel](https://stripe.com/docs/api/payouts/cancel).
    pub fn cancel(client: &Client, payout_id: &str) -> Result<Payout, Error> {
        client.post_empty(&format!("/payouts/{}/cancel", payout_id))
    }
}
