use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE};
use reqwest::Client;
use csv::Reader;
use chrono::Local;

const LOG_PATH: &str = "log.txt";
const CSV_PATH: &str = "data.csv";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create(LOG_PATH)?;

    loop {
        let time = Local::now();
        writeln!(file, "{}", time)?;

        let mut rdr = Reader::from_path(CSV_PATH)?;

        for (i, result) in rdr.records().enumerate() {
            let record = result?;
            let host = &record[0];
            let cookie = &record[1];
            let tag = &record[2];

            let client = Client::new();

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(COOKIE, HeaderValue::from_str(cookie.trim())?);

            let res = client.post(host)
                .headers(headers)
                .body(r#"{"token": "glados.network"}"#)
                .send()
                .await?;

            println!("index = {}, name = {}", i, tag);
            writeln!(file, "index = {}, name = {}", i, tag)?;
            writeln!(file, "Status: {}", res.status())?;
            
            let body = res.text().await?;
            writeln!(file, "Body:\n{}", body)?;
            writeln!(file)?;
        }
        thread::sleep(Duration::from_secs(8 * 60 * 60));// 八小时执行一次
    }
}
