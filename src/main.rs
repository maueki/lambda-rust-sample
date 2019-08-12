use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use std::collections::HashMap;
use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use serde_dynamodb;
use env_logger;
use log::{error, warn, info, debug};
use std::env;

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

fn main() -> Result<(), Box<dyn Error>>{
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda!(handler);
    Ok(())
}

fn handler(_: Value, c: Context) -> Result<Item, HandlerError> {
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

    let client = DynamoDbClient::new(Region::Custom {name: "localhost".to_owned(),
                                                     endpoint: "http://localhost:8000".to_owned()});
    match client.get_item(input).sync() {
        Ok(result) => {
            match result.item {
                Some(item) => {
                    Ok(serde_dynamodb::from_hashmap(item).unwrap())
                }
                None => {
                    error!("{}", "no item was found.");
                    Ok(Default::default())
                }
            }
        },
        Err(error) => {
            Err(c.new_error(error.description()))
        }
    }
}

