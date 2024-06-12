# 腾讯云日志服务CLS RUST SDK

example usage:

```rust
use tencentcloud-cls-sdk-rust::{LogGroupList, LogGroup, Content, Log, LogProducer};

fn main() {
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
		.block_on(producer.put_logs(
			"23eaa499-b7a9-4a60-a628-49a4239ddbba".to_string(),
			&log_group_list,
		))
		.unwrap();
	let text = rt.block_on(result.text()).unwrap();
	println!("{}", text);
}
```