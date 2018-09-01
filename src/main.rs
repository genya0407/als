/* Spec
*/

extern crate als;
extern crate getopts;
extern crate regex;
extern crate chrono;
extern crate itertools;

use std::io;
use std::io::Write;
use getopts::Options;
use std::env;
use regex::Regex;
use chrono::prelude::*;
use itertools::Itertools;

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
    } else if matches.opt_present("max") {
        AggregationMode::Max
    } else if matches.opt_present("min") {
        AggregationMode::Min
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
    let aggregated = access_log_aggregator.aggregate(filtered);

    // write tsv
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());
    for (t, val) in aggregated {
        stdout.write(format!("{}\t{}\n", t.to_rfc3339(), val).as_bytes()).unwrap();
    }
}

enum AggregationMode {
    Sum,
    Cnt,
    Avg,
    Max,
    Min,
}

struct AccessLogAggregator {
    pub mode: AggregationMode
}

impl AccessLogAggregator {
    fn aggregate(&self, access_logs: Vec<als::AccessLog>) -> Vec<(DateTime<Local>, f32)> {
        match self.mode {
            AggregationMode::Sum => self.group_by_second(access_logs, |access_logs| {
                access_logs.into_iter().map(|al| al.reqtime).sum()
            }),
            AggregationMode::Cnt => self.group_by_second(access_logs, |access_logs| {
                access_logs.len() as f32
            }),
            AggregationMode::Avg => self.group_by_second(access_logs, |access_logs| {
                let cnt = access_logs.len() as f32;
                let sum: f32 = access_logs.into_iter().map(|al| al.reqtime).sum();
                return sum / cnt;
            }),
            AggregationMode::Max => self.group_by_second(access_logs, |access_logs| {
                access_logs.into_iter().map(|al| al.reqtime).fold(0.0/0.0, f32::max)
            }),
            AggregationMode::Min => self.group_by_second(access_logs, |access_logs| {
                access_logs.into_iter().map(|al| al.reqtime).fold(0.0/0.0, f32::min)
            }),
        }
    }

    fn group_by_second<F>(&self, mut access_logs: Vec<als::AccessLog>, mut f: F) -> Vec<(DateTime<Local>, f32)>
            where F: FnMut(Vec<als::AccessLog>) -> f32 {
        let mut results = vec![];

        access_logs.sort_by_key(|al| al.time);
        for (timestamp, access_logs) in &access_logs.into_iter().group_by(|al| al.time.timestamp()) {
            let t = Local.timestamp(timestamp, 0);
            let value = f(access_logs.collect());
            results.push((t, value))
        }

        return results;
    }
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
    opts.optflag("", "max", "show the average response time");
    opts.optflag("", "min", "show the average response time");

    opts.optopt("", "uri",    "set target uri pattern", "PATTERN");
    opts.optopt("", "method", "set target http method", "METHOD");
    opts.optopt("", "status", "set target http status", "STATUS");

    return opts;
}