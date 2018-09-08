extern crate alta;
extern crate getopts;
extern crate regex;
extern crate nix;

use std::io::{self, Write};
use getopts::Options;
use std::env;
use regex::Regex;
use nix::sys::stat::fstat;
use std::os::unix::io::AsRawFd;

use alta::{access_log, access_log_filter, access_log_aggregator, plot};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
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

    opts.optopt("", "width", "set graph width (default: 1.0)", "WIDTH");

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
        alta::access_log::from_reader(file)
    } else {
        let stdin = io::stdin();
        let file_stat = fstat(stdin.as_raw_fd()).expect("Failed to call fstat (2).");
        if (file_stat.st_mode & nix::sys::stat::SFlag::S_IFIFO.bits()) == 0 {
            print_usage(&program, opts);
            return;
        }
        let stdin = stdin.lock();
        alta::access_log::from_reader(stdin)
    };

    // filter & aggregate
    let filtered_logs = filter.filter(access_logs);
    let aggregated_logs = aggregator.aggregate(filtered_logs);

    // generate tsv
    let mut tsv = String::new();
    for (t, val) in aggregated_logs {
        tsv += &format!("{}\t{}\n", t.timestamp(), val);
    }

    let width_ratio: f32 = matches.opt_str("width").and_then(|ratio| ratio.parse().ok()).unwrap_or(1.0);
    let width = (100.0 * width_ratio) as u32;

    let plot_string = plot::plot_ascii(tsv, width);

    let stdout = io::stdout();
    let mut stdout = io::BufWriter::new(stdout.lock());
    stdout.write_all(plot_string.as_bytes()).unwrap();
}
