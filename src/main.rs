use std::path::PathBuf;

use clap::Parser;
use regex::{Captures, Regex};

const PREGAP: i32 = 150;

fn log_input_to_entries(input: &str) -> Vec<Captures> {
    let toc_entry_matcher =
        Regex::new(r"^\s*(\d+)\s*\|\s*([0-9:.]+)\s*\|\s*([0-9:.]+)\s*\|\s*(\d+)\s*\|\s*(\d+) \s*$")
            .unwrap();

    input
        .split('\n')
        .filter_map(|line| toc_entry_matcher.captures(line))
        .collect()
}

fn calculate_mb_toc_numbers(entries: &[Captures]) -> Vec<String> {
    if entries.len() == 0 {
        panic!("No entries provided");
    }

    let leadout_offset = entries[entries.len() - 1][5].parse::<i32>().unwrap() + PREGAP + 1;

    let offsets = entries
        .iter()
        .map(|entry| entry[4].parse::<i32>().unwrap() + PREGAP);

    [1, entries.len() as i32, leadout_offset]
        .into_iter()
        .chain(offsets)
        .map(|x| x.to_string())
        .collect()
}

fn get_mb_url(input: &str) -> String {
    let entries = log_input_to_entries(&input);
    let mb_toc_numbers = calculate_mb_toc_numbers(&entries);
    format!(
        "http://musicbrainz.org/cdtoc/attach?toc={}",
        mb_toc_numbers.join("%20")
    )
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.path).expect("Failed to read input file");
    let mb_url = get_mb_url(&input);
    println!("{}", mb_url);
}

#[derive(Debug, Parser)]
/// Calculate MusicBrainz TOC URL from a log file
struct Args {
    /// Path to the log file
    path: PathBuf,
}

#[test]
fn works() {
    let input = include_str!("../test.log");
    let expected =  "http://musicbrainz.org/cdtoc/attach?toc=1%2016%20317070%20150%2020105%2039987%2057487%2081912%2098392%20108470%20129100%20144197%20165207%20182145%20201872%20224950%20244800%20269430%20293667";
    assert_eq!(expected, get_mb_url(input));
}
