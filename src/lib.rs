use dotenvy_macro::dotenv;
use reqwest::header::{HeaderMap, COOKIE};

use std::{io::Write, path::Path, path::PathBuf};

pub async fn fetch_input(day: usize) -> anyhow::Result<()> {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");

    let fname = format!("day{day}");
    let data_dir = Path::new("data");
    if data_dir
        .read_dir()
        .unwrap()
        .any(|f| f.unwrap().path() == Path::new(&fname))
    {
        return Ok(());
    }

    let key = dotenv!("KEY");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, format!("session={key}").parse()?);

    let res = client
        .get(url)
        .header(COOKIE, format!("session={key}"))
        .send()
        .await?
        .text()
        .await?;

    let fpath: PathBuf = ["data", &fname].iter().collect();
    let mut file = std::fs::File::create(fpath)?;
    file.write_all(res.as_bytes())?;

    Ok(())
}
