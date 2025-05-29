use domain::models::SecurityEntity;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "document")]
pub struct SecurityResponse {
    #[serde(rename = "data")]
    data: Vec<DataBlock>,
}

#[derive(Debug, Deserialize)]
struct DataBlock {
    #[serde(rename = "@id")]
    id: String,
    rows: Rows,
}

#[derive(Debug, Deserialize)]
struct Rows {
    #[serde(rename = "row")]
    row: Vec<Row>,
}

#[derive(Debug, Deserialize)]
struct Row {
    #[serde(flatten)]
    attributes: std::collections::HashMap<String, String>,
}

impl From<SecurityResponse> for SecurityEntity {
    fn from(response: SecurityResponse) -> Self {
        let description = response
            .data
            .iter()
            .find(|block| block.id == "description")
            .expect("Description data block not found");

        let get_field = |field_name: &str| -> Option<String> {
            description
                .rows
                .row
                .iter()
                .find(|row| row.attributes.get("@name") == Some(&field_name.to_string()))
                .and_then(|row| row.attributes.get("@value").cloned())
        };

        Self {
            security_id: get_field("SECID").unwrap_or_default(),
            shortname: get_field("SHORTNAME").unwrap_or_default(),
            name: get_field("NAME").unwrap_or_default(),
            isin: get_field("ISIN").unwrap_or_default(),
        }
    }
}
