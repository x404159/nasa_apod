use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub title: String,
    pub url: String,
    pub hdurl: Option<String>,
    pub explanation: String,
    pub media_type: String,
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct E {
    pub code: u32,
    pub msg: String,
}

pub fn construct_url() -> String {
    let base_url = "https://api.nasa.gov/planetary/apod?api_key=";
    let api_key = env!("API_KEY", "set or export the environment variable API_KEY");
    let matches = App::new("Nasa daily picture")
        .version("0.1.0")
        .author("Abhinav y. <abhinavy14@gmail.com>")
        .about("get astronomy picture of the day from nasa ")
        .arg(
            Arg::new("start_date")
                .about("start date / from date (used with end_date)")
                .short('s')
                .long("start_date")
                .value_name("date <yyy-mm-dd>")
                .takes_value(true),
        )
        .arg(
            Arg::new("end_date")
                .about("end date / to date (used with start_date)")
                .short('e')
                .long("end_date")
                .value_name("date <yyyy-mm-dd>")
                .takes_value(true),
        )
        .arg(
            Arg::new("date")
                .about("specific date (used without any other argument)")
                .short('d')
                .long("date")
                .value_name("date <yyyy-mm-dd>")
                .takes_value(true),
        )
        .get_matches();

    if let Some(v) = matches.value_of("date") {
        return format!("{}{}&date={}", base_url, api_key, v);
    } else if let Some(v) = matches.value_of("start_date") {
        if let Some(y) = matches.value_of("end_date") {
            return format!("{}{}&start_date={}&end_date={}", base_url, api_key, v, y);
        } else {
            print_err("start_date is used with end_date");
            std::process::exit(1);
        }
    } else if let Some(v) = matches.value_of("end_date") {
        if let Some(y) = matches.value_of("start_date") {
            return format!("{}{}&start_date={}&end_date={}", base_url, api_key, y, v);
        } else {
            print_err("start_date and end_date are used together");
            std::process::exit(1);
        }
    } else {
        return format!("{}{}", base_url, api_key);
    }
}

pub fn print_to_stdout(json_data: Vec<Body>) {
    for data in json_data {
        println!("{}\n", Yellow.paint("###############################"));
        println!(
            "{}:  {}\n",
            Green.bold().paint("Title"),
            Purple.paint(data.title)
        );
        println!(
            "{}:  {}\n",
            Green.bold().paint("Date"),
            Purple.paint(data.date)
        );
        println!(
            "{}:  {}\n",
            Green.bold().paint(format!("URL for {}", data.media_type)),
            Blue.underline().paint(data.url)
        );
        println!(
            "{}:  {}\n",
            Green
                .bold()
                .paint(format!("HD_URL for {}", data.media_type)),
            Blue.underline().paint(match data.hdurl {
                Some(u) => u,
                None => "not available".to_owned(),
            })
        );
        println!(
            "{}:  {}\n",
            Green.bold().paint("Explanation"),
            Purple.paint(data.explanation)
        );
    }
}

pub fn print_err(msg: &str) {
    eprintln!("{}", Red.paint(msg));
}
