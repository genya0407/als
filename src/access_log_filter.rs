use regex::Regex;
use access_log::{AccessLog, Method};

pub struct AccessLogFilter {
    uri: Option<Regex>,
    method: Option<Method>,
    status: Option<i32>,
}

impl AccessLogFilter {
    pub fn new(uri: Option<Regex>, method: Option<Method>, status: Option<i32>) -> Self {
        Self { uri: uri, method: method, status: status }
    }

    pub fn filter(&self, access_logs: Vec<AccessLog>) -> Vec<AccessLog> {
        access_logs.into_iter().filter(|al| {
            if let Some(ref re) = self.uri {
                if !re.is_match(&al.uri) {
                    return false;
                }
            }

            if let Some(m) = self.method {
                if al.method != m {
                    return false;
                }
            }

            if let Some(st) = self.status {
                if al.status != st {
                    return false;
                }
            }

            return true;
        }).collect()
    }
}
