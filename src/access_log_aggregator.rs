use access_log::AccessLog;
use chrono::prelude::*;
use itertools::Itertools;

pub enum AggregationMode {
    Sum,
    Cnt,
    Avg,
    Max,
    Min,
}

pub struct AccessLogAggregator {
    mode: AggregationMode
}

impl AccessLogAggregator {
    pub fn new(mode: AggregationMode) -> Self {
        Self { mode: mode }
    }

    pub fn aggregate(&self, access_logs: Vec<AccessLog>) -> Vec<(DateTime<Local>, f32)> {
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

    fn group_by_second<F>(&self, mut access_logs: Vec<AccessLog>, mut f: F) -> Vec<(DateTime<Local>, f32)>
            where F: FnMut(Vec<AccessLog>) -> f32 {
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
