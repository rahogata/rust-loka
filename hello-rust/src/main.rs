use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = "Ramaya Rama Bhadraya";

    let mut writer = BufWriter::new(stdout.lock());
    say(message, 10, &mut writer).unwrap();
}
