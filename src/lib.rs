use std::io::Write;
use std::path::{Path, PathBuf};

use dotenvy_macro::dotenv;
use reqwest::header::{HeaderMap, COOKIE};

pub async fn fetch_input(day: usize) -> anyhow::Result<String> {
    let url = format!("https://adventofcode.com/2023/day/{day}/input");

    let fname = format!("day{day}");
    let data_dir = Path::new("data");
    if let Some(file) = data_dir
        .read_dir()?
        .find(|f| f.as_ref().unwrap().path() == Path::new(&fname))
    {
        return Ok(std::fs::read_to_string(file?.path())?);
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

    Ok(res.to_string())
}
