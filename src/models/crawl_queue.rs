#![allow(dead_code)]
use sea_orm::entity::prelude::*;
use serde::Serialize;

use std::fmt;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum CrawlStatus {
    #[sea_orm(string_value = "Queued")]
    Queued,
    #[sea_orm(string_value = "Processing")]
    Processing,
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "Failed")]
    Failed,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "indexed_document")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// URL to crawl.
    pub url: String,
    /// Task status.
    pub status: CrawlStatus,
    /// Number of retries for this task.
    pub num_retries: u8,
    /// Ignore crawl settings for this URL/domain and push to crawler.
    pub force_crawl: bool,
    /// When this was first added to the crawl queue.
    pub created_at: DateTimeUtc,
    /// When this task was last updated.
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl fmt::Display for CrawlStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrawlStatus::Queued => write!(f, "Queued"),
            CrawlStatus::Processing => write!(f, "Processing"),
            CrawlStatus::Completed => write!(f, "Completed"),
            CrawlStatus::Failed => write!(f, "Failed"),
        }
    }
}

// #[derive(Serialize)]
// pub struct CrawlQueue {
//     pub id: Option<i64>,
//     /// URL to crawl.
//     pub url: String,
//     /// Task status.
//     pub status: CrawlStatus,
//     /// Number of retries for this task.
//     pub num_retries: u8,
//     /// Ignore crawl settings for this URL/domain and push to crawler.
//     pub force_crawl: bool,
//     /// When this was first added to the crawl queue.
//     pub created_at: DateTime,
//     /// When this task was last updated.
//     pub updated_at: DateTime,
// }

// impl CrawlQueue {
//     pub async fn init_table(db: &DbPool) -> anyhow::Result<(), sqlx::Error> {
//         let mut conn = db.acquire().await?;

//         sqlx::query(
//             "CREATE TABLE IF NOT EXISTS
//             crawl_queue (
//                 id INTEGER PRIMARY KEY,
//                 url TEXT UNIQUE,
//                 status TEXT,
//                 num_retries INTEGER,
//                 force_crawl BOOL,
//                 created_at DATETIME default CURRENT_TIMESTAMP,
//                 updated_at DATETIME default CURRENT_TIMESTAMP
//             )",
//         )
//         .execute(&mut conn)
//         .await?;

//         Ok(())
//     }

//     pub async fn insert(
//         db: &DbPool,
//         url: &str,
//         force_crawl: bool,
//     ) -> anyhow::Result<(), sqlx::Error> {
//         let mut conn = db.acquire().await?;

//         sqlx::query(
//             "INSERT INTO crawl_queue (
//                 url,
//                 status,
//                 num_retries,
//                 force_crawl
//             )
//             VALUES (?, ?, 0, ?)
//             ON CONFLICT(url) DO UPDATE SET
//                 updated_at = CURRENT_TIMESTAMP,
//                 status = ?",
//         )
//         .bind(url)
//         .bind(CrawlStatus::Queued)
//         .bind(force_crawl)
//         .bind(CrawlStatus::Queued)
//         .execute(&mut conn)
//         .await?;

//         Ok(())
//     }

//     pub async fn get(db: &DbPool, id: i64) -> anyhow::Result<CrawlQueue, sqlx::Error> {
//         let mut conn = db.acquire().await?;

//         let row = sqlx::query(
//             "SELECT
//                 id,
//                 url,
//                 status,
//                 num_retries,
//                 force_crawl,
//                 created_at,
//                 updated_at
//             FROM crawl_queue WHERE id = ?",
//         )
//         .bind(id)
//         .fetch_one(&mut conn)
//         .await?;

//         Ok(CrawlQueue {
//             id: row.get(0),
//             url: row.get(1),
//             status: row.get(2),
//             num_retries: row.get(3),
//             force_crawl: row.get(4),
//             created_at: row.get(5),
//             updated_at: row.get(6),
//         })
//     }

//     pub async fn list(
//         db: &DbPool,
//         status: Option<CrawlStatus>,
//     ) -> anyhow::Result<Vec<CrawlQueue>, sqlx::Error> {
//         let mut conn = db.acquire().await?;

//         let mut filter_status: Vec<String> = Vec::new();
//         if status.is_none() {
//             filter_status.push(CrawlStatus::Queued.to_string());
//         } else {
//             filter_status.push(CrawlStatus::Completed.to_string());
//             filter_status.push(CrawlStatus::Failed.to_string());
//             filter_status.push(CrawlStatus::Processing.to_string());
//             filter_status.push(CrawlStatus::Queued.to_string());
//         }

//         let results = sqlx::query(
//             "SELECT
//                 id, url, status, force_crawl, created_at
//             FROM crawl_queue
//             WHERE status IN (?)
//             ORDER BY created_at ASC
//             LIMIT 100",
//         )
//         .bind(filter_status.join(","))
//         .fetch_all(&mut conn)
//         .await?;

//         let parsed = results
//             .iter()
//             .map(|row| CrawlQueue {
//                 id: row.get(0),
//                 url: row.get::<String, _>(1),
//                 status: row.get(2),
//                 num_retries: row.get(3),
//                 force_crawl: row.get(4),
//                 created_at: row.get(5),
//                 updated_at: row.get(6),
//             })
//             .collect();

//         Ok(parsed)
//     }

//     pub async fn next(db: &DbPool) -> anyhow::Result<Option<CrawlTask>, sqlx::Error> {
//         let mut conn = db.begin().await?;
//         let row: Option<SqliteRow> = sqlx::query(
//             "
//                 SELECT id FROM crawl_queue
//                 WHERE status = ?
//                 ORDER BY created_at ASC LIMIT 1",
//         )
//         .bind(CrawlStatus::Queued)
//         .fetch_optional(&mut conn)
//         .await?;

//         if let Some(row) = row {
//             let id: i64 = row.get(0);
//             sqlx::query("UPDATE crawl_queue SET status = ? WHERE id = ?")
//                 .bind(CrawlStatus::Processing)
//                 .bind(id)
//                 .execute(&mut conn)
//                 .await?;

//             conn.commit().await?;
//             return Ok(Some(CrawlTask { id }));
//         }

//         conn.commit().await?;
//         Ok(None)
//     }

//     /// Find tasks that have been processing for a while and retry
//     pub async fn clean_stale(_db: &DbPool) {
//         todo!();
//     }

//     /// Mark job as done
//     pub async fn mark_done(db: &DbPool, id: i64) -> anyhow::Result<(), sqlx::Error> {
//         let mut conn = db.acquire().await?;
//         sqlx::query("UPDATE crawl_queue SET status = ? WHERE id = ?")
//             .bind(CrawlStatus::Completed)
//             .bind(id)
//             .execute(&mut conn)
//             .await?;

//         Ok(())
//     }
// }
