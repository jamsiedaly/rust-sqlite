use anyhow::{bail, Result};
use std::fs::read;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let data = read(&args[1])?;
            let header = &data[0..100];
            let other = &data[100..];

            let page_size = u16::from_be_bytes([header[16], header[17]]);

            let encoding = match u32::from_be_bytes(header[56..60].try_into().expect("Invalid encoding")) {
                1 => "UTF-8",
                _ => bail!("Unsupported encoding"),
            };

            let table_count = u16::from_be_bytes([other[3], other[4]]);

            println!("database page size: {}", page_size);
            println!("encoding: {}", encoding);
            println!("number of tables: {}", table_count);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
