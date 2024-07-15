use std::path::Path;
use std::process::exit;
use chrono::{Duration, Local, NaiveDateTime, NaiveTime, TimeZone};
use tokio::time::sleep;

use crate::config::Config;
use crate::database::{ActivityLog, DatabaseC};

mod config;
mod database;
mod webhook;

#[tokio::main]
async fn main() {
    let config_path = "./config.yml";
    if !Path::new(config_path).is_file() {
        Config::create(config_path);
        exit(1);
    }

    let conf = Config::load(config_path);

    let mut db = DatabaseC::new(conf.database);
    db.connection().await;

    let now_date0 = NaiveDateTime::new(Local::now().date_naive(), NaiveTime::from_hms_opt(0,0,0).unwrap());
    let before_duration = Duration::weeks(conf.weeks_ago);
    let before_datetime = now_date0 - before_duration;
    let to = Local.from_local_datetime(&before_datetime).unwrap();

    println!("delete up to {:?}", to.to_string());

    let discord = webhook::Webhook::new(conf.webhook.as_str());

    let count = db.count_activities_from_datetime(&to).await;

    let mut li :Vec<Vec<ActivityLog>> = vec![];
    let i = ((count as f64) / 100.0).ceil() as i64;
    let mut offset = 0;

    for _ in 0..i {
        let logs = db.get_activities_from_datetime(&to, offset, 100).await;
        li.push(logs);

        sleep(tokio::time::Duration::from_millis(500)).await;
        offset += 100
    }

    let upload_time = Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    for (index, logs) in li.iter().enumerate() {
        let js = serde_json::to_vec_pretty(&logs).unwrap();
        discord.send(format!("{} #{}/{}", upload_time, (index as i64)+1, li.len()).as_str(), js.clone()).await.expect("SendError");

        sleep(tokio::time::Duration::from_millis(4000)).await;

        let l = ((logs.len() as i64)-1 )as usize;
        let start_id = logs.get(0).unwrap().id;
        let end_id = logs.get(l).unwrap().id;
        db.delete_activities(start_id, end_id).await.unwrap()
    }
}

