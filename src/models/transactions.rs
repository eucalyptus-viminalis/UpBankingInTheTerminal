use serde::{Deserialize, Serialize};

use super::common::{MoneyObject, Relationship, RelationshipList, SelfLink};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Held,
    Settled,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Held => write!(f, "HELD"),
            TransactionStatus::Settled => write!(f, "SETTLED"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPurchaseMethodType {
    BarCode,
    Ocr,
    CardPin,
    CardDetails,
    CardOnFile,
    Ecommerce,
    MagneticStripe,
    Contactless,
}

impl std::fmt::Display for CardPurchaseMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardPurchaseMethodType::BarCode => write!(f, "BAR_CODE"),
            CardPurchaseMethodType::Ocr => write!(f, "OCR"),
            CardPurchaseMethodType::CardPin => write!(f, "CARD_PIN"),
            CardPurchaseMethodType::CardDetails => write!(f, "CARD_DETAILS"),
            CardPurchaseMethodType::CardOnFile => write!(f, "CARD_ON_FILE"),
            CardPurchaseMethodType::Ecommerce => write!(f, "ECOMMERCE"),
            CardPurchaseMethodType::MagneticStripe => write!(f, "MAGNETIC_STRIPE"),
            CardPurchaseMethodType::Contactless => write!(f, "CONTACTLESS"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldInfo {
    pub amount: MoneyObject,
    pub foreign_amount: Option<MoneyObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundUp {
    pub amount: MoneyObject,
    pub boost_portion: Option<MoneyObject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cashback {
    pub description: String,
    pub amount: MoneyObject,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPurchaseMethod {
    pub method: CardPurchaseMethodType,
    pub card_number_suffix: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformingCustomer {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAttributes {
    pub status: TransactionStatus,
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    pub is_categorizable: bool,
    pub hold_info: Option<HoldInfo>,
    pub round_up: Option<RoundUp>,
    pub cashback: Option<Cashback>,
    pub amount: MoneyObject,
    pub foreign_amount: Option<MoneyObject>,
    pub card_purchase_method: Option<CardPurchaseMethod>,
    pub settled_at: Option<String>,
    pub created_at: String,
    pub transaction_type: Option<String>,
    pub note: Option<Note>,
    pub performing_customer: Option<PerformingCustomer>,
    pub deep_link_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRelationships {
    pub account: Option<Relationship>,
    pub transfer_account: Option<Relationship>,
    pub category: Option<Relationship>,
    pub parent_category: Option<Relationship>,
    pub tags: Option<RelationshipList>,
    pub attachment: Option<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: TransactionAttributes,
    pub relationships: Option<TransactionRelationships>,
    pub links: Option<SelfLink>,
}
