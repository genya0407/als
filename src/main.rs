/* Spec
    指定できるべきもの：
        - aggregation
            - sum
            - cnt
            - avg
        - filter
            - uri
                - can be regexp or direct path
            - method
            - status
*/

extern crate als;
extern crate getopts;
extern crate regex;

use std::io;
use getopts::Options;
use std::env;
use regex::Regex;

fn main() {
    // parse options
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = opts();
    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // construct aggregator
    let mode = if matches.opt_present("sum") {
        AggregationMode::Sum
    } else if matches.opt_present("cnt") {
        AggregationMode::Cnt
    } else if matches.opt_present("avg") {
        AggregationMode::Avg
    } else {
        AggregationMode::Cnt
    };
    let access_log_aggregator = AccessLogAggregator { mode: mode };

    // construct filter
    let access_log_filter = AccessLogFilter {
        uri: matches.opt_str("uri").map(|u| Regex::new(&u).expect("Failed to parse uri as regexp.")),
        method: matches.opt_str("method").map(|s| als::Method::from_str(&s)),
        status: matches.opt_str("status").map(|s| s.parse().expect("Failed to parse status as integer.")),
    };

    // parse access log
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let access_logs = als::read_access_log(stdin);
    let filtered = access_log_filter.filter(access_logs);
    println!("{}", filtered.len());
}

enum AggregationMode {
    Sum,
    Cnt,
    Avg
}

struct AccessLogAggregator {
    pub mode: AggregationMode
}

struct AccessLogFilter {
    pub uri: Option<Regex>,
    pub method: Option<als::Method>,
    pub status: Option<i32>,
}

impl AccessLogFilter {
    fn filter(&self, access_logs: Vec<als::AccessLog>) -> Vec<als::AccessLog> {
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn opts() -> Options {
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

    opts.optflag("", "sum", "show the sum of the response times");
    opts.optflag("", "cnt", "show the count of the requests");
    opts.optflag("", "avg", "show the average response time");

    opts.optopt("", "uri",    "set target uri pattern", "PATTERN");
    opts.optopt("", "method", "set target http method", "METHOD");
    opts.optopt("", "status", "set target http status", "STATUS");

    return opts;
}