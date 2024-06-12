extern crate chrono;
extern crate rustc_serialize;
extern crate sha1;
extern crate url;

use std::collections::HashMap;

use rustc_serialize::hex::ToHex;

pub const SHA1_DIGEST_BYTES: usize = 20;
const SHA1_KEY_BYTES: usize = 64;

pub fn cal_sha1_sum(msg: &str) -> String {
    sha1::Sha1::from(msg).digest().to_string()
}

pub fn cal_sha1_hmac_digest(key: &[u8], message: &[u8]) -> [u8; SHA1_DIGEST_BYTES] {
    let inner_pad_byte: u8 = 0x36;
    let outer_pad_byte: u8 = 0x5c;
    let key_pad_byte: u8 = 0x00;

    let mut sha1_ctx = sha1::Sha1::new();
    let auth_key: &mut [u8; SHA1_KEY_BYTES] = &mut [key_pad_byte; SHA1_KEY_BYTES];

    if key.len() > SHA1_KEY_BYTES {
        sha1_ctx.update(key);
        auth_key[..SHA1_DIGEST_BYTES].copy_from_slice(&(sha1_ctx.digest().bytes()));
        sha1_ctx.reset();
    } else {
        auth_key[..key.len()].copy_from_slice(key);
    }

    let mut inner_padding: [u8; SHA1_KEY_BYTES] = [inner_pad_byte; SHA1_KEY_BYTES];
    let mut outer_padding: [u8; SHA1_KEY_BYTES] = [outer_pad_byte; SHA1_KEY_BYTES];

    for offset in 0..auth_key.len() {
        inner_padding[offset] ^= auth_key[offset];
        outer_padding[offset] ^= auth_key[offset];
    }
    sha1_ctx.update(&inner_padding);
    sha1_ctx.update(message);
    let inner_hash = sha1_ctx.digest().bytes();
    sha1_ctx.reset();
    sha1_ctx.update(&outer_padding);
    sha1_ctx.update(&inner_hash);
    sha1_ctx.digest().bytes()
}

pub fn signature(
    secret_id: &str,
    secret_key: &str,
    method: &str,
    path: &str,
    params: &HashMap<String, String>,
    headers: &HashMap<String, String>,
    expire: i64,
) -> String {
    let mut signed_header_list: Vec<String> = vec![];
    let mut signed_parameter_list: Vec<String> = vec![];

    let mut hs = url::form_urlencoded::Serializer::new(String::new());
    for (key, val) in headers {
        let lower_key = key.to_lowercase();
        if lower_key == "content-type"
            || lower_key == "content-md5"
            || lower_key == "host"
            || lower_key.starts_with('x')
        {
            hs.append_pair(lower_key.as_str(), val.as_str());
            signed_header_list.push(lower_key.clone());
        }
    }
    signed_header_list.sort();
    let format_headers = hs.finish().as_str().to_string();

    let mut ps = url::form_urlencoded::Serializer::new(String::new());
    for (key, val) in params {
        let lower_key = key.to_lowercase();
        ps.append_pair(lower_key.as_str(), val.as_str());
        signed_parameter_list.push(lower_key.clone());
    }
    let format_parameters = ps.finish().as_str().to_string();
    signed_parameter_list.sort();
    let format_string = format!(
        "{}\n{}\n{}\n{}\n",
        method.to_lowercase(),
        path,
        format_parameters,
        format_headers
    );

    let sign_time = format!(
        "{};{}",
        chrono::Local::now().timestamp() - 60,
        chrono::Local::now().timestamp() + expire
    );
    let string_to_sign = format!(
        "sha1\n{}\n{}\n",
        sign_time.clone(),
        cal_sha1_sum(format_string.as_str())
    );

    let sign_key = cal_sha1_hmac_digest(secret_key.as_bytes(), sign_time.as_bytes()).to_hex();
    let signature = cal_sha1_hmac_digest(sign_key.as_bytes(), string_to_sign.as_bytes()).to_hex();

    vec![
        "q-sign-algorithm=sha1".to_string(),
        format!("q-ak={}", secret_id),
        format!("q-sign-time={}", sign_time),
        format!("q-key-time={}", sign_time),
        format!("q-header-list={}", signed_header_list.join(";")),
        format!("q-url-param-list={}", signed_parameter_list.join(";")),
        format!("q-signature={}", signature),
    ]
    .join("&")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use crate::sign::signature;
    
    #[test]
    fn test_url_encode() {
        let mut headers: HashMap<String, String> = HashMap::new();
        let params: HashMap<String, String> = HashMap::new();
        headers.insert(
            "Host".to_string(),
            "ap-shanghai.cls.myqcloud.com".to_string(),
        );
        headers.insert("User-Agent".to_string(), "AuthSDK".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert(
            "Content-MD5".to_string(),
            "f9c7fc33c7eab68dfa8a52508d1f4659".to_string(),
        );

        // params.insert(
        //     "logset_id".to_string(),
        //     "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string(),
        // );

        let data = signature(
            "SecretIdExample_XXXXXXXXXXXXXXXXXXXXX",
            "SecretKeyExample_XXXXXXXXXXXXXXXX",
            "GET",
            "/logset",
            &params,
            &headers,
            300,
        );
        println!("{}", data)
    }
}
