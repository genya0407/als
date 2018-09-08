extern crate als;
extern crate getopts;
extern crate regex;

use std::io::{self, Write};
use getopts::Options;
use std::env;
use regex::Regex;

use als::{access_log, access_log_filter, access_log_aggregator};

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

    opts.optopt("f", "input-file", "set nginx log file", "FILE");

    return opts;
}

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

    // build aggregator
    let aggregation_mode = if matches.opt_present("sum") {
        access_log_aggregator::AggregationMode::Sum
    } else if matches.opt_present("cnt") {
        access_log_aggregator::AggregationMode::Cnt
    } else if matches.opt_present("avg") {
        access_log_aggregator::AggregationMode::Avg
    } else if matches.opt_present("max") {
        access_log_aggregator::AggregationMode::Max
    } else if matches.opt_present("min") {
        access_log_aggregator::AggregationMode::Min
    } else {
        access_log_aggregator::AggregationMode::Cnt // default
    };
    let aggregator = access_log_aggregator::AccessLogAggregator::new(aggregation_mode);

    // build filter
    let uri = matches.opt_str("uri").map(|u| Regex::new(&u).expect("Failed to parse uri as regexp."));
    let method = matches.opt_str("method").map(|s| access_log::Method::from_str(&s));
    let status = matches.opt_str("status").map(|s| s.parse().expect("Failed to parse status as integer."));
    let filter = access_log_filter::AccessLogFilter::new(uri, method, status);

    // read & parse access log
    let access_logs = if let Some(input_filename) = matches.opt_str("f") {
        let file = std::fs::File::open(input_filename.clone()).expect(&format!("Failed to read file: {}", input_filename));
        let file = io::BufReader::new(file);
        als::access_log::from_reader(file)
    } else {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        als::access_log::from_reader(stdin)
    };

    // filter & aggregate
    let filtered_logs = filter.filter(access_logs);
    let aggregated_logs = aggregator.aggregate(filtered_logs);

    // write tsv
    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());
    for (t, val) in aggregated_logs {
        stdout.write(format!("{}\t{}\n", t.timestamp(), val).as_bytes()).expect("Failed to write result.");
    }
}
