use std::env;
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn main() -> anyhow::Result<()> {
    if env::args().len() != 2 {
        eprintln!("expected day number");
        exit(1);
    }

    let day_str = env::args().last().unwrap();

    if day_str.parse::<usize>().is_err() {
        eprintln!("expected day number to be a positive int");
        exit(1);
    }

    let template = std::fs::read_to_string("templ")?.replace("$$$", &day_str);

    let template = template.split("=====").collect::<Vec<_>>();
    let bin = template[0];
    let bin_info = template[1];

    let bin_fname = format!("src/day{day_str}.rs");
    let bin_fname = Path::new(&bin_fname);
    if bin_fname.exists() {
        return Ok(());
    }
    let mut bin_file = std::fs::File::create(bin_fname)?;
    bin_file.write_all(bin.as_bytes())?;

    let mut cargo_toml = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("Cargo.toml")?;
    cargo_toml.write_all(bin_info.as_bytes())?;

    Ok(())
}
