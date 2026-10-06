use hbb_common::{config::Config, log, tokio, ResultType};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

#[cfg(feature = "flutter")]
pub mod account;
pub mod downloader;
mod http_client;
pub mod record_upload;
pub mod sync;
pub use http_client::{
    create_http_client_async, create_http_client_async_with_url, create_http_client_with_url,
    get_url_for_tls,
};

/// SCTG: аудит — close-событие оператора (POST /api/audit/conn) при завершении сессии.
/// Закрывает сессию в журнале подключений, даже если цель не прислала close (обрыв у цели).
/// URL — как у клиента-цели (api-server или custom-rendezvous−2), без access_token.
pub fn audit_conn_close(target_id: &str, session_id: u64) {
    if target_id.is_empty() {
        return;
    }
    let url = crate::get_audit_server(
        Config::get_option("api-server"),
        Config::get_option("custom-rendezvous-server"),
        "conn".to_owned(),
    );
    if url.is_empty() {
        return;
    }
    let target_id = target_id.to_owned();
    let body = serde_json::json!({
        "action": "close",
        "id": target_id,
        "peer": [Config::get_id(), String::new()],
        "session_id": session_id,
    });
    tokio::spawn(async move {
        if let Err(e) = crate::post_request(url, body.to_string(), "").await {
            log::debug!("SCTG audit close post failed: {}", e);
        }
    });
}

#[derive(Debug)]
pub enum HbbHttpResponse<T> {
    ErrorFormat,
    Error(String),
    DataTypeFormat,
    Data(T),
}

impl<T: DeserializeOwned> HbbHttpResponse<T> {
    pub fn parse(body: &str) -> ResultType<Self> {
        let map = serde_json::from_str::<Map<String, Value>>(body)?;
        if let Some(error) = map.get("error") {
            if let Some(err) = error.as_str() {
                Ok(Self::Error(err.to_owned()))
            } else {
                Ok(Self::ErrorFormat)
            }
        } else {
            match serde_json::from_value(Value::Object(map)) {
                Ok(v) => Ok(Self::Data(v)),
                Err(_) => Ok(Self::DataTypeFormat),
            }
        }
    }
}
