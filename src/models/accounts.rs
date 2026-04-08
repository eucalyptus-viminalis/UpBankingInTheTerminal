use serde::Deserialize;

use super::common::{MoneyObject, Relationship, SelfLink};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Saver,
    Transactional,
    HomeLoan,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Saver => write!(f, "SAVER"),
            AccountType::Transactional => write!(f, "TRANSACTIONAL"),
            AccountType::HomeLoan => write!(f, "HOME_LOAN"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OwnershipType {
    Individual,
    Joint,
}

impl std::fmt::Display for OwnershipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnershipType::Individual => write!(f, "INDIVIDUAL"),
            OwnershipType::Joint => write!(f, "JOINT"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAttributes {
    pub display_name: String,
    pub account_type: AccountType,
    pub ownership_type: OwnershipType,
    pub balance: MoneyObject,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountRelationships {
    pub transactions: Option<Relationship>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub id: String,
    pub attributes: AccountAttributes,
    pub relationships: Option<AccountRelationships>,
    pub links: Option<SelfLink>,
}
