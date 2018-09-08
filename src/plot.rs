use std::process::{Command, Stdio};
use std::error::Error;
use std::io::prelude::*;

pub fn plot_ascii(tsv: String, width: u32) -> String {
    let gnuplot_settings = format!("set terminal dumb {} 30; set timefmt \"%s\"; set xdata time; plot \"<cat\" using 1:2 with line;", width);
    let process = match Command::new("gnuplot")
                                .arg("-e")
                                .arg(&gnuplot_settings)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn gnuplot: {}", why.description()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(tsv.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why.description()),
        _ => (),
    }

    let mut out = String::new();
    match process.stdout.unwrap().read_to_string(&mut out) {
        Err(why) => panic!("couldn't read gnuplot stdout: {}",
                           why.description()),
        _ => (),
    }

    return out;
}