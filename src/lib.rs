use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

use chrono::Utc;
use reqwest::{Client, Method, RequestBuilder, Url};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, DATE, HOST, USER_AGENT};

use crate::cls_log::{Log, LogGroup, LogGroupList};
use crate::cls_log::mod_Log::Content;
use crate::cls_log_json::Logs;
use crate::consts::headers::{LOG_COMPRESS_TYPE, USER_AGENT_VALUE};
use crate::error::LogProducerError;
use crate::sign::signature;

pub mod cls_log;
mod cls_log_json;
mod consts;
pub mod error;
pub mod sign;

pub struct LogProducer {
    access_key: String,
    access_secret: String,
    host: String,
    client: Client,
}

impl LogProducer {
    pub fn new(
        access_key: String,
        access_secret: String,
        host: String,
    ) -> Result<Self, LogProducerError> {
        if access_key.is_empty() {
            Err(LogProducerError::InvalidParameter {
                error_message: "access_key is empty".to_string(),
            })?;
        }
        if access_secret.is_empty() {
            Err(LogProducerError::InvalidParameter {
                error_message: "access_secret is empty".to_string(),
            })?;
        }
        if host.is_empty() {
            Err(LogProducerError::InvalidParameter {
                error_message: "host is empty".to_string(),
            })?;
        }
        Ok(Self {
            access_key,
            access_secret,
            host,
            client: reqwest::ClientBuilder::new().build()?,
        })
    }

    pub async fn put_logs_json(
        &self,
        topic_id: String,
        data: &str,
    ) -> Result<reqwest::Response, LogProducerError> {
        let logs: Logs = serde_json::from_str(data).unwrap();
        let mut log_group_list = LogGroupList::default();
        let mut log_group: LogGroup = LogGroup::default();
        if logs.source.is_some() {
            log_group.source = Option::Some(Cow::from(logs.source.unwrap()));
        }

        if logs.filename.is_some() {
            log_group.filename = Option::Some(Cow::from(logs.filename.unwrap()));
        }

        if logs.hostname.is_some() {
            log_group.hostname = Option::Some(Cow::from(logs.hostname.unwrap()));
        }

        logs.logs.iter().for_each(|item| {
            let mut log: Log = Log::default();
            log.time = item.time;
            item.contents.iter().for_each(|content| {
                log.contents
                    .push(Content::new(content.key.as_str(), content.value.as_str()));
            });
            log_group.logs.push(log);
        });

        log_group_list.logGroupList.push(log_group);

        Ok(self.put_logs(topic_id, &log_group_list).await?)
    }

    pub async fn put_logs(
        &self,
        topic_id: String,
        log_group: &LogGroupList<'_>,
    ) -> Result<reqwest::Response, LogProducerError> {
        if topic_id.is_empty() {
            Err(LogProducerError::InvalidParameter {
                error_message: "topic_id is empty".to_string(),
            })?;
        }

        let buf = log_group.encode()?;
        let compressed = zstd::encode_all(buf.as_ref(), 3)?;
        let request = self
            .new_request(Method::POST, "/structuredlog".to_string())?
            .query(&[("topic_id", topic_id)])
            .header(CONTENT_LENGTH, compressed.len())
            .header(CONTENT_TYPE, "application/x-protobuf")
            .header(LOG_COMPRESS_TYPE, "zstd")
            .body(compressed);

        Ok(self.send(request).await?)
    }

    fn new_request(
        &self,
        method: Method,
        path: String,
    ) -> Result<RequestBuilder, LogProducerError> {
        let url = Url::from_str(&*format!("https://{}{}", self.host, path))?;
        let date = Utc::now().format("%a,%d%b%Y %H:%M:%S GMT").to_string();
        let request = self
            .client
            .request(method, url)
            .header(USER_AGENT, USER_AGENT_VALUE)
            .header(DATE, date)
            .header(HOST, &*self.host);
        Ok(request)
    }

    async fn send(&self, request: RequestBuilder) -> Result<reqwest::Response, LogProducerError> {
        let mut request = request.build()?;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut params: HashMap<String, String> = HashMap::new();
        let _ = request.headers().iter().map(|(key, value)| {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string())
        });
        let pairs = request.url().query_pairs();
        let _ = pairs.map(|(key, value)| params.insert(key.to_string(), value.to_string()));

        let sign_str = signature(
            self.access_key.as_str(),
            self.access_secret.as_str(),
            request.method().as_str(),
            request.url().path(),
            &params,
            &headers,
            300,
        );
        request
            .headers_mut()
            .insert("Authorization", sign_str.parse().unwrap());

        Ok(self.client.execute(request).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    
    use crate::cls_log::{Log, LogGroup};
    use crate::cls_log::mod_Log::Content;
    
    use super::*;
    
    #[test]
    fn send_logs() {
        // create a async runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        let producer = LogProducer::new(
            "".to_string(),
            "".to_string(),
            "ap-guangzhou-open.cls.tencentcs.com".to_string(),
        )
        .unwrap();

        // Create a new Log with default timestamp (now)
        let mut log: Log = Log::default();
        log.time = chrono::Local::now().timestamp_millis();
        // Push K-V pairs to Log
        log.contents.push(Content::new("level", "INFO"));
        log.contents.push(Content::new("message", "startup"));
        // Create LogGroup
        let mut log_group: LogGroup = LogGroup::default();
        log_group.source = Option::Some(Cow::from("127.0.0.1"));
        log_group.logs.push(log);
        let mut log_group_list = LogGroupList::default();
        log_group_list.logGroupList.push(log_group);

        let result = rt
            .block_on(producer.put_logs("".to_string(), &log_group_list))
            .unwrap();
        let text = rt.block_on(result.text()).unwrap();
        println!("{}", text);
    }

    #[test]
    fn send_logs_json() {
        // create a async runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        let producer = LogProducer::new(
            "".to_string(),
            "".to_string(),
            "ap-guangzhou-open.cls.tencentcs.com".to_string(),
        );

        if let Ok(producer) = producer {
            let logs = "{\"filename\":\"\",\"source\":\"127.0.0.2\",\"hostname\":\"\",\"logs\":[{\"time\":1718247083,\"contents\":[{\"value\":\"hello\",\"key\":\"world\"}]},{\"time\":1718247083,\"contents\":[{\"value\":\"hi\",\"key\":\"hey\"}]}]}";
            let result = rt
                .block_on(
                    producer
                        .put_logs_json("23eaa499-b7a9-4a60-a628-49a4239ddbba".to_string(), logs),
                )
                .unwrap();
            let text = rt.block_on(result.text()).unwrap();
            println!("{}", text);
        } else {
            println!("{}", "init producer failed");
        }
    }
}
