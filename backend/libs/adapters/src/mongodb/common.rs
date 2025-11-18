use std::fmt::Display;

use chrono::{
    DateTime,
    NaiveDateTime,
    Utc,
};
use domain::{
    errors::DomainError,
    value_objects::Price,
};
use mongodb::{
    Cursor,
    Database,
    bson::{
        DateTime as BsonDateTime,
        doc,
    },
    options::{
        FindOneAndUpdateOptions,
        ReturnDocument,
    },
};
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceDocument {
    pub amount: f64,
    pub currency: String,
}

impl From<&Price> for PriceDocument {
    fn from(value: &Price) -> Self {
        Self {
            amount: value.amount,
            currency: value.currency.value.clone(),
        }
    }
}

impl From<PriceDocument> for Price {
    fn from(value: PriceDocument) -> Self {
        Price::new(value.amount, value.currency)
    }
}

pub async fn next_sequence(database: &Database, counter: &str) -> Result<u32, DomainError> {
    let collection = database.collection::<CounterDocument>("counters");
    let update = doc! { "$inc": { "value": 1 } };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .upsert(true)
        .build();

    let document = collection
        .find_one_and_update(doc! { "_id": counter }, update, options)
        .await
        .map_err(repo_error)?;

    document
        .map(|counter| counter.value)
        .ok_or_else(|| DomainError::RepositoryError("Failed to increment counter".into()))
}

pub fn repo_error(err: impl Display) -> DomainError {
    DomainError::RepositoryError(err.to_string())
}

pub fn id_to_u32(value: i64, field: &str) -> Result<u32, DomainError> {
    u32::try_from(value).map_err(|_| {
        DomainError::RepositoryError(format!("{field} value {value} does not fit into u32"))
    })
}

pub fn bson_to_naive(datetime: BsonDateTime) -> NaiveDateTime {
    let millis = datetime.timestamp_millis();
    let seconds = millis.div_euclid(1_000);
    let nanos = (millis.rem_euclid(1_000) * 1_000_000) as u32;
    DateTime::<Utc>::from_timestamp(seconds, nanos)
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| DateTime::<Utc>::UNIX_EPOCH.naive_utc())
}

pub fn naive_to_bson(datetime: NaiveDateTime) -> BsonDateTime {
    BsonDateTime::from_millis(datetime.and_utc().timestamp_millis())
}

pub async fn collect_cursor<T>(mut cursor: Cursor<T>) -> Result<Vec<T>, DomainError>
where
    T: Unpin + DeserializeOwned,
{
    let mut items = Vec::new();
    while cursor.advance().await.map_err(repo_error)? {
        items.push(cursor.deserialize_current().map_err(repo_error)?);
    }
    Ok(items)
}
