use env_logger;
use failure::format_err;
use lambda_http::{lambda, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{debug, error, info, warn};
use rusoto_core::Region;
use serde_derive::{Deserialize, Serialize};
use serde_dynamodb;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::error::Error;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Item {
    year: i64,
    title: String,
    info: ItemInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ItemInfo {
    plot: String,
    rating: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda!(handler);
    Ok(())
}

fn handler(_: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();

    query_key.insert(
        "year".to_owned(),
        AttributeValue {
            n: Some("2013".to_owned()),
            ..Default::default()
        },
    );

    query_key.insert(
        "title".to_owned(),
        AttributeValue {
            s: Some("Rush".to_owned()),
            ..Default::default()
        },
    );

    let input: GetItemInput = GetItemInput {
        table_name: "Movies".to_owned(),
        key: query_key,
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::Custom {
        name: "ap-northeast-1".to_owned(),
        endpoint: "http://192.168.11.201:8000".to_owned(),
    });
    match client.get_item(input).sync() {
        Ok(result) => match result.item {
            Some(item) => {
                let item: Item = serde_dynamodb::from_hashmap(item).unwrap();
                Ok(json!(item))
            }
            None => {
                error!("{}", "no item was found.");
                Ok(json!(""))
            }
        },
        Err(error) => Err(format_err!("{}", error.description()).into()),
    }
}
