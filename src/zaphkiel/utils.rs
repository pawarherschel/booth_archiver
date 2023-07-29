use indicatif::{ProgressBar, ProgressStyle};

pub fn get_pb(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);

    let pb_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec})")
        .unwrap()
        .progress_chars("#>-");
    pb.set_style(pb_style);
    pb.tick();

    pb
}
