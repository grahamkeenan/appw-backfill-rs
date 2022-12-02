use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub environment: String,
    pub appw_environment: String,
    pub api_environment: String,
    pub event_type: String,
    pub tleo_pids: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct SnsPayload {
    #[serde(rename = "Records")]
    records: Vec<SnsRecord>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct SnsRecord {
    #[serde(rename = "eventName")]
    event_name: String,
    s3: SnsS3Information
}

#[derive(Serialize, Debug)]
struct SnsS3Information {
    #[serde(rename = "s3SchemaVersion")]
    s3_schema_version: String,
    #[serde(rename = "configurationId")]
    configuration_id: String,
    bucket: S3BucketInformation,
    object: S3ObjectInformation
}

#[derive(Serialize, Debug)]
struct S3BucketInformation {
    name: String,
    arn: String
}

#[derive(Serialize, Debug)]
struct S3ObjectInformation {
    key: String
}

pub fn parse_config() -> Result<Config> {
    let contents = std::fs::read_to_string("./config.json")?;
    let config = serde_json::from_str(&contents)?;

    Ok(config)
}