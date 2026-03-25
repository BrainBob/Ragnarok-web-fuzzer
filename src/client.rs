use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Proxy,
};
use std::str::FromStr;
use std::time::Duration;

pub fn build_headers(raw: &[String]) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    for h in raw {
        let parts: Vec<&str> = h.splitn(2, ':').collect();
        if parts.len() == 2 {
            map.insert(
                HeaderName::from_str(parts[0].trim())?,
                HeaderValue::from_str(parts[1].trim())?,
            );
        }
    }
    Ok(map)
}

pub fn build_client(
    timeout:      u64,
    user_agent:   &str,
    proxy:        Option<&str>,
    cookies:      Option<&str>,
    extra_headers: HeaderMap,
) -> Result<Client> {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .user_agent(user_agent)
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(extra_headers);

    if let Some(p) = proxy {
        builder = builder.proxy(Proxy::all(p)?);
    }

    if let Some(c) = cookies {
        let mut hmap = HeaderMap::new();
        hmap.insert(
            reqwest::header::COOKIE,
            HeaderValue::from_str(c)?,
        );
        builder = builder.default_headers(hmap);
    }

    Ok(builder.build()?)
}
