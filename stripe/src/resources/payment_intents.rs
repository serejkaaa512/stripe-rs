use client::Client;
use error::Error;
use params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use resources::{Charge, Currency, ShippingDetails};
use serde_qs as qs;

/// The resource representing a Stripe PaymentIntent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents](https://stripe.com/docs/api/payment_intents).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntent {
    pub id: String,
    pub object: String,
    pub allowed_source_types: Vec<String>,
    pub amount: u64,
    pub amount_capturable: u64,
    pub amount_received: u64,
    pub application: Option<String>,
    pub application_fee_amount: Option<u64>,
    pub canceled_at: Option<Timestamp>,
    pub cancellation_reason: Option<CancellationReason>,
    pub capture_method: CaptureMethod,
    pub charges: List<Charge>,
    pub client_secret: Option<String>,
    pub confirmation_method: Option<ConfirmationMethod>,
    pub created: Timestamp,
    pub currency: Currency,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub last_payment_error: Option<PaymentError>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub next_source_action: Option<NextSourceAction>,
    pub on_behalf_of: Option<String>,
    pub receipt_email: Option<String>,
    pub review: Option<String>,
    pub shipping: Option<ShippingDetails>,
    pub source: String,
    pub statement_descriptor: Option<String>,
    pub status: PaymentIntentStatus,
    pub transfer_data: Option<TransferData>,
    pub transfer_group: Option<String>,
}

impl Identifiable for PaymentIntent {
    fn id(&self) -> &str {
        &self.id
    }
}

/// The resource representing a Stripe PaymentError object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentError {
    #[serde(rename = "type")]
    pub payment_error_type: PaymentErrorType,
    pub charge: Option<String>,
    pub code: Option<String>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub source: Option<String>,
}

/// The resource representing a Stripe PaymentErrorType object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error-type](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-last_payment_error-type).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum PaymentErrorType {
    #[serde(rename = "api_error")]
    Api,
    #[serde(rename = "api_connection_error")]
    Connection,
    #[serde(rename = "authentication_error")]
    Authentication,
    #[serde(rename = "card_error")]
    Card,
    #[serde(rename = "idempotency_error")]
    Idempotency,
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,
    #[serde(rename = "rate_limit_error")]
    RateLimit,
    #[serde(other)]
    Other,
}

/// The resource representing a Stripe PaymentIntentStatus object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-status](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-status).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    RequiresSource,
    RequiresConfirmation,
    RequiresSourceAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded,
    #[serde(other)]
    Other,
}
/// The resource representing a Stripe CancellationReason object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-cancellation_reason](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-cancellation_reason).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CancellationReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    #[serde(other)]
    Other,
}
/// The resource representing a Stripe CaptureMethod object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-capture_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-capture_method).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,
    #[serde(other)]
    Other,
}
/// The resource representing a Stripe ConfirmationMethod object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-confirmation_method](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-confirmation_method).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    Secret,
    Publishable,
    #[serde(other)]
    Other,
}

/// The resource representing a Stripe ConfirmationMethod object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action-type](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action-type).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceActionType {
    AuthorizeWithUrl,
    UseStripeSdk,
    #[serde(other)]
    Other,
}

/// The resource representing a Stripe NextSourceAction object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NextSourceAction {
    pub authorize_with_url: AuthorizeWithUrl,
    #[serde(rename = "type")]
    pub action_type: SourceActionType,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows. The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    pub use_stripe_sdk: serde_json::Value,
}

/// The resource representing a Stripe AuthorizeWithUrl object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action-authorize_with_url](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-next_source_action-authorize_with_url).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthorizeWithUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}

/// The resource representing a Stripe TransferData object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/object#payment_intent_object-transfer_data](https://stripe.com/docs/api/payment_intents/object#payment_intent_object-transfer_data).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferData {
    pub destination: Option<String>,
}

/// The set of parameters that can be used when creating a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/create](https://stripe.com/docs/api/payment_intents/create)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCreateParams {
    pub allowed_source_types: Vec<String>, //The list of source types (e.g. card) that this PaymentIntent is allowed to use.
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>, // Attempt to confirm this PaymentIntent on source attachment. type?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// The set of parameters that can be used when updating a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/update](https://stripe.com/docs/api/payment_intents/update)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// The set of parameters that can be used when confirming a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/confirm](https://stripe.com/docs/api/payment_intents/confirm)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentConfirmParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// The set of parameters that can be used when capturing a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/capture](https://stripe.com/docs/api/payment_intents/capture)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCaptureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
}

/// The set of parameters that can be used when canceling a payment_intent object.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/cancel](https://stripe.com/docs/api/payment_intents/cancel)
#[derive(Clone, Debug, Default, Serialize)]
pub struct PaymentIntentCancelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancellationReason>,
}

/// The set of parameters that can be used when listing payment_intents.
///
/// For more details see [https://stripe.com/docs/api/payment_intents/list](https://stripe.com/docs/api/payment_intents/list)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentIntentListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl PaymentIntent {
    /// Creates a new payment_intent.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/create](https://stripe.com/docs/api/payment_intents/create).
    pub fn create(
        client: &Client,
        params: PaymentIntentCreateParams,
    ) -> Result<PaymentIntent, Error> {
        client.post("/payment_intents", params)
    }

    /// Retrieves the details of a payment_intent.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/retrieve](https://stripe.com/docs/api/payment_intents/retrieve).
    pub fn retrieve(client: &Client, payment_intent_id: &str) -> Result<PaymentIntent, Error> {
        client.get(&format!("/payment_intents/{}", payment_intent_id))
    }

    /// Updates a payment_intent's properties.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/update](https://stripe.com/docs/api/payment_intents/update).
    pub fn update(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentUpdateParams,
    ) -> Result<PaymentIntent, Error> {
        client.post(&format!("/payment_intents/{}", payment_intent_id), params)
    }

    /// Confirm that customer intends to pay with current or provided source. Upon confirmation, the PaymentIntent will attempt to initiate a payment.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/confirm](https://stripe.com/docs/api/payment_intents/confirm).
    pub fn confirm(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentConfirmParams,
    ) -> Result<PaymentIntent, Error> {
        client.post(
            &format!("/payment_intents/{}/confirm", payment_intent_id),
            params,
        )
    }

    /// Capture the funds of an existing uncaptured PaymentIntent where required_action="requires_capture".
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/capture](https://stripe.com/docs/api/payment_intents/capture).
    pub fn capture(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentCaptureParams,
    ) -> Result<PaymentIntent, Error> {
        client.post(
            &format!("/payment_intents/{}/capture", payment_intent_id),
            params,
        )
    }

    /// A PaymentIntent object can be canceled when it is in one of these statuses: requires_source, requires_capture, requires_confirmation, requires_source_action.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/cancel](https://stripe.com/docs/api/payment_intents/cancel).
    pub fn cancel(
        client: &Client,
        payment_intent_id: &str,
        params: PaymentIntentCancelParams,
    ) -> Result<PaymentIntent, Error> {
        client.post(
            &format!("/payment_intents/{}/cancel", payment_intent_id),
            params,
        )
    }

    /// List all payment_intents.
    ///
    /// For more details see [https://stripe.com/docs/api/payment_intents/list](https://stripe.com/docs/api/payment_intents/list).
    pub fn list(
        client: &Client,
        params: PaymentIntentListParams,
    ) -> Result<List<PaymentIntent>, Error> {
        client.get(&format!("/payment_intents?{}", qs::to_string(&params)?))
    }
}
