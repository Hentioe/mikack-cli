use crate::errors::{Result as ErrResult, *};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    Response,
};

lazy_static! {
    static ref DEFAULT_USER_AGENT: HeaderValue =
        HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36");
}

pub fn request(url: &str) -> ErrResult<Response> {
    let mut helper = SendHelper::new();
    helper.send_get(url)?;
    Ok(helper.response.unwrap())
}

pub struct SendHelper {
    client: reqwest::Client,
    headers: HeaderMap,
    pub response: Option<Response>,
}

impl SendHelper {
    pub fn with_headers(headers: HeaderMap) -> Self {
        let client = reqwest::Client::new();
        let mut headers_n = headers.clone();
        if !headers.contains_key(USER_AGENT) {
            headers_n.insert(USER_AGENT, (*DEFAULT_USER_AGENT).clone());
        }
        Self {
            client,
            headers: headers_n,
            response: None,
        }
    }

    pub fn with_header(name: HeaderName, value: HeaderValue) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(name, value);
        Self::with_headers(headers)
    }

    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, (*DEFAULT_USER_AGENT).clone());
        Self {
            client,
            headers,
            response: None,
        }
    }

    pub fn send_get(&mut self, url: &str) -> ErrResult<()> {
        self.response = Some(self.client.get(url).headers(self.headers.clone()).send()?);
        Ok(())
    }

    pub fn done(&self) -> bool {
        self.response.is_some()
    }

    pub fn succeed(&self) -> ErrResult<bool> {
        if self.done() {
            let resp = self.response.as_ref().unwrap();
            Ok(resp.status().is_success())
        } else {
            Err(err_msg("request not sent complete"))
        }
    }

    pub fn result(&mut self) -> Result {
        match self.succeed() {
            Ok(r) => {
                if r {
                    Result::Ok(self.response.as_mut().unwrap().text().unwrap())
                } else {
                    Result::Err(err_msg("did not get the correct response"))
                }
            }
            Err(e) => Result::Err(e),
        }
    }

    pub fn result_bytes(&mut self) -> RawResult {
        match self.succeed() {
            Ok(r) => {
                if r {
                    let mut buf: Vec<u8> = vec![];
                    self.response.as_mut().unwrap().copy_to(&mut buf).unwrap();
                    RawResult::Ok(buf)
                } else {
                    RawResult::Err(err_msg("did not get the correct response"))
                }
            }
            Err(e) => RawResult::Err(e),
        }
    }
}

pub enum Result {
    Ok(String),
    Err(Error),
}

pub enum RawResult {
    Ok(Vec<u8>),
    Err(Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let url = "http://www.hhmmoo.com/comic/";
        let resp = request(url).unwrap();
        assert_eq!(true, resp.status().is_success());
    }

    #[test]
    fn test_send_helper() {
        let url = "http://www.hhmmoo.com/comic/";
        let mut helper = SendHelper::new();
        helper.send_get(url).unwrap();
        assert_eq!(true, helper.done());
        assert_eq!(true, helper.succeed().unwrap());
        match helper.result() {
            Result::Ok(html_s) => {
                assert!(html_s.len() > 9999);
            }
            Result::Err(_e) => assert!(false),
        }
    }

}
