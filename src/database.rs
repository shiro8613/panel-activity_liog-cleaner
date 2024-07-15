use std::process::exit;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySql, MySqlPool, Pool};
use crate::config::DatabaseConfig;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct ActivityLog {
    pub id: u64,
    pub batch: Option<String>,
    pub event: String,
    pub ip: String,
    pub description: Option<String>,
    pub actor_type: Option<String>,
    pub actor_id: Option<u64>,
    pub api_key_id: Option<u64>,
    pub properties: String,
    pub timestamp: DateTime<Local>,
}

pub struct DatabaseC {
    db_config: DatabaseConfig,
    pool: Option<Pool<MySql>>
}

impl DatabaseC {

    pub fn new(conf :DatabaseConfig) -> Self {
        Self {
            db_config: conf,
            pool: None
        }
    }

    pub async fn connection(&mut self){
        let conf = &self.db_config;
        let database_connection = format!("mysql://{}:{}@{}:{}/{}",
                                          conf.username,
                                          conf.password,
                                          conf.host,
                                          conf.port,
                                          conf.db_name);

        let p :Pool<MySql> = match MySqlPool::connect(&database_connection).await {
            Ok(pool) => pool,
            Err(e) => {
                println!("{:?}", e.as_database_error().unwrap().message());
                exit(1);
            }
        };
        self.pool = Some(p);
    }

    fn get_pool(&self) -> &Pool<MySql> {
        let p = match &self.pool {
            Some(p) => p,
            None => {
                println!("database error");
                exit(1);
            }
        };
        p
    }

    pub async fn count_activities_from_datetime(&self, to :&DateTime<Local>) -> i64 {
        let pool = self.get_pool();
        let sql = r#"select count(*) from activity_logs where timestamp <= ?;"#;
        let count :(i64,) = match sqlx::query_as(sql)
            .bind(to)
            .fetch_one(pool)
            .await {
            Ok(l) => l,
            Err(e) => {
                println!("{:?}", e.to_string());
                exit(1);
            }
        };

        count.0

    }

    pub async fn get_activities_from_datetime(&self, to :&DateTime<Local>, offset :i64, limit :i64) -> Vec<ActivityLog> {
        let pool = self.get_pool();
        let sql = r#"select * from activity_logs where timestamp <= ? limit ? offset ?;"#;
        let logs :Vec<ActivityLog> = match sqlx::query_as(sql)
            .bind(to)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await {
                Ok(l) => l,
                Err(e) => {
                    println!("{:?}", e.to_string());
                    exit(1);
                }
        };

        logs
    }

    pub async fn delete_activities(&self, start_id :u64, end_id :u64) ->Result<(), Error> {
        let pool = self.get_pool();
        let sql = r#"delete from activity_logs where id >= ? and id <= ?;"#;
        match sqlx::query(sql)
            .bind(start_id)
            .bind(end_id)
            .execute(pool)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}