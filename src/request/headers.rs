use std::collections::HashMap;

use crate::request::Headers;


impl Headers {
    pub fn new() -> Self {
        Headers {
            headers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}


impl Default for Headers {
    fn default() -> Self {
        let mut headers = Headers::new();
        headers.insert(HeadersTypes::ClientInfo.as_str(), "supabase-rs/0.3.3");
        headers.insert(HeadersTypes::ContentType.as_str(), "application/json");
        Headers {
            headers: headers.get_headers(),
        }
    }
}

pub enum HeadersTypes {
    ApiKey,
    Authorization,
    ContentType,
    Prefer,
    ClientInfo,
}

impl HeadersTypes {
    pub fn as_str(&self) -> &str {
        match self {
            HeadersTypes::ApiKey => "apikey",
            HeadersTypes::Authorization => "Authorization",
            HeadersTypes::ContentType => "Content-Type",
            HeadersTypes::Prefer => "prefer",
            HeadersTypes::ClientInfo => "x_client_info",
        }
    }
}
