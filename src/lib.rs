use std::collections::HashMap;
use std::str::FromStr;

use chrono::Utc;
use reqwest::{Client, Method, RequestBuilder, Url};
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, DATE, HOST, USER_AGENT};

use crate::cls_log::LogGroupList;
use crate::consts::headers::{LOG_COMPRESS_TYPE, USER_AGENT_VALUE};
use crate::error::Error;
use crate::sign::signature;

pub mod cls_log;
mod consts;
pub mod error;
pub mod sign;

pub struct LogProducer<'a> {
    access_key: &'a str,
    access_secret: &'a str,
    host: &'a str,
    client: Client,
}

impl<'a> LogProducer<'a> {
    pub fn new(access_key: &'a str, access_secret: &'a str, host: &'a str) -> Result<Self, Error> {
        Ok(Self {
            access_key,
            access_secret,
            host,
            client: reqwest::ClientBuilder::new().build()?,
        })
    }

    pub async fn put_logs(
        &self,
        topic_id: String,
        log_group: &LogGroupList<'_>,
    ) -> Result<reqwest::Response, Error> {
        let buf = log_group.encode()?;
        let compressed = zstd::encode_all(buf.as_ref(), 3).unwrap();
        let request = self
            .new_request(Method::POST, "/structuredlog".to_string())?
            .query(&[("topic_id", topic_id)])
            .header(CONTENT_LENGTH, compressed.len())
            .header(CONTENT_TYPE, "application/x-protobuf")
            .header(LOG_COMPRESS_TYPE, "zstd")
            .body(compressed);

        Ok(self.send(request).await?)
    }

    fn new_request(&self, method: Method, path: String) -> Result<RequestBuilder, Error> {
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

    async fn send(&self, request: RequestBuilder) -> Result<reqwest::Response, Error> {
        let mut request = request.build()?;
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut params: HashMap<String, String> = HashMap::new();
        let _ = request.headers().iter().map(|(key, value)| {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string())
        });
        let pairs = request.url().query_pairs();
        let _ = pairs.map(|(key, value)| params.insert(key.to_string(), value.to_string()));

        let sign_str = signature(
            self.access_key,
            self.access_secret,
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
        let producer = LogProducer::new("", "", "ap-guangzhou-open.cls.tencentcs.com").unwrap();

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
}
