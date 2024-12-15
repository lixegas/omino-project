use axum::{extract::Path, Json};
use rusqlite::{params, Connection};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct VisitCountResponse {
    count: i64,
    row_count: i64, 
}

pub async fn visit_count(
    Path(device_id): Path<String>,
    db: Arc<Mutex<Connection>>,  
) -> Result<Json<VisitCountResponse>, axum::http::StatusCode> {
    let since = Utc::now() - chrono::Duration::hours(24); 
    let since_timestamp = since.to_rfc3339();

    let mut count: i64 = 0; 
    let mut row_count: i64 = 0; 

    
    let insert_query = r#"
        INSERT INTO visits (device_id)
        VALUES (?1)
    "#;

    let db = db.lock().await;

    match db.execute(insert_query, params![device_id]) {
        Ok(_) => {
            
            let query = r#"
                SELECT COUNT(*) 
                FROM visits
                WHERE created_at > ?1 AND device_id = ?2
            "#;

            match db.prepare(query) {
                Ok(mut stmt) => {
                    if let Ok(mut rows) = stmt.query(params![since_timestamp, device_id]) {
                        while let Ok(Some(row)) = rows.next() {
                            count = row.get(0).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
                            row_count += 1;
                        }
                    }
                }
                Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            }

            
            Ok(Json(VisitCountResponse {
                count,
                row_count,
            }))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}


pub async fn get_count(
    Path(device_id): Path<String>, 
    db: Arc<Mutex<Connection>>,
) -> Result<Json<VisitCountResponse>, axum::http::StatusCode> {
    let mut count: i64 = 0;

    let query = r#"
        SELECT COUNT(visit_id) 
        FROM visits 
        WHERE device_id = ?1
    "#;

    let db = db.lock().await;

    match db.prepare(query) {
        Ok(mut stmt) => {
            if let Ok(mut rows) = stmt.query(params![device_id]) {
                if let Ok(Some(row)) = rows.next() {
                    count = row.get(0).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
                }
            }
        }
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }

    Ok(Json(VisitCountResponse { count, row_count: 0 })) 
}
