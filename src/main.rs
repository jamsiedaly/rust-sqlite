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
            let _other = &data[100..];

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            let page_size = u16::from_be_bytes([header[16], header[17]]);

            let encoding = match u32::from_be_bytes(header[56..60].try_into().expect("Invalid encoding")) {
                1 => "UTF-8",
                _ => bail!("Unsupported encoding"),
            };

            // You can use print statements as follows for debugging, they'll be visible when running tests.
            println!("Logs from your program will appear here!");

            // Uncomment this block to pass the first stage
            println!("database page size: {}", page_size);
            println!("encoding: {}", encoding);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
