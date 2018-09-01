extern crate chrono;

use std::io::prelude::*;
use chrono::prelude::*;

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Other
}

impl Method {
    pub fn from_str(method_str: &str) -> Self {
        match method_str.to_uppercase().as_str() {
            "GET"    => Method::Get,
            "POST"   => Method::Post,
            "PUT"    => Method::Put,
            "DELETE" => Method::Delete,
            _        => Method::Other,
        }
    }
}

pub struct AccessLog {
    pub time: DateTime<Local>,
    pub method: Method,
    pub uri: String,
    pub status: i32,
    pub reqtime: f32,
}

impl AccessLog {
    pub fn default() -> Self {
        Self {
            time: Local.ymd(1970, 1, 1).and_hms(0, 0, 0),
            method: Method::Get,
            uri: "".to_string(),
            status: 0,
            reqtime: 0.0,
        }
    }
}

pub fn read_access_log<R>(reader: R) -> Vec<AccessLog> where R: BufRead {
    let mut results = vec![];
    for line in reader.lines() {

        let mut access_log = AccessLog::default();
        for record in line.unwrap().split("\t") {
            let mut record = record.split(":");
            let label = record.next().unwrap();
            let value = record.collect::<Vec<&str>>().join(":");
            let value = &value;

            match label {
                "time"    => {
                    access_log.time = Local.datetime_from_str(value, "%d/%b/%Y:%H:%M:%S %z").expect("time");
                }
                "method"  => {
                    access_log.method = Method::from_str(value);
                }
                "uri"     => {
                    access_log.uri = value.to_string();
                }
                "status"  => {
                    access_log.status = value.parse().expect("status");
                }
                "reqtime" => {
                    access_log.reqtime = value.parse().expect("reqtime");
                }
                _         => {
                    // do nothing
                }
            }
        }
        results.push(access_log)
    }
    return results;
}