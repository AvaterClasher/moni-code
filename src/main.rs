use moni_code::models::problem::ProblemSetQuestionListRoot;
use reqwest::{self, cookie::Jar, Url};
use sea_orm::{Database, DatabaseConnection};
use serde_json::{json, Value};
use tracing;
use tracing_subscriber;

use once_cell::sync::Lazy;

fn main() {
    println!("Hello, world!");
}
