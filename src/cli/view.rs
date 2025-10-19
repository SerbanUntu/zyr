use crate::domain::Data;
use crate::utils::time_utils;
use chrono::Local;
use std::collections::HashMap;
use std::time::Duration;

pub fn exec(data: &Data) {
    let now_dt = Local::now();

    let mut filtered: Vec<(Duration, &str)> = data
        .blocks
        .iter()
        .filter(|b| {
            time_utils::same_day(now_dt, time_utils::convert(b.start_unix))
                || (b.end_unix.is_some()
                    && time_utils::same_day(now_dt, time_utils::convert(b.end_unix.unwrap())))
                || b.end_unix.is_none()
        })
        .map(|b| {
            let start_dt = time_utils::convert(b.start_unix);
            let mut end_dt = now_dt;

            if let Some(end) = b.end_unix {
                end_dt = time_utils::convert(end);
                if end_dt > now_dt {
                    end_dt = now_dt;
                }
            }

            let millis = if !time_utils::same_day(now_dt, start_dt) {
                time_utils::millis_since_midnight(end_dt)
            } else {
                (end_dt - start_dt).num_milliseconds() as u64
            };

            (Duration::from_millis(millis), &b.category[..])
        })
        .fold(HashMap::new(), |mut acc, (d, category)| {
            acc.entry(category)
                .and_modify(|existing| *existing += d)
                .or_insert(d);
            acc
        })
        .into_iter()
        .map(|(s, d)| (d, s))
        .collect::<Vec<(Duration, &str)>>();
    filtered.sort_by(|a, b| b.cmp(a));

    let time_worked: Duration = filtered
        .iter()
        .filter(|t| t.1 != "break")
        .fold(Duration::from_millis(0), |t, c| t + c.0);

    let time_break: Duration = filtered
        .iter()
        .filter(|t| t.1 == "break")
        .fold(Duration::from_millis(0), |t, c| t + c.0);

    let breakdown: String = filtered
        .iter()
        .map(|(d, c)| format!("{}: {}", c, time_utils::prettify_duration(*d)))
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "Overview:\n\nTime worked: {}\nBreak time: {}\nTotal: {}\n",
        time_utils::prettify_duration(time_worked),
        time_utils::prettify_duration(time_break),
        time_utils::prettify_duration(time_worked + time_break)
    );

    println!("Breakdown by category:\n\n{}", breakdown);
}
