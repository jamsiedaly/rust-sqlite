use anyhow::{bail, Result};
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::FileExt;

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
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            let page_size = u16::from_be_bytes([header[16], header[17]]);

            let encoding = match u32::from_be_bytes(header[56..60].try_into().expect("Invalid encoding")) {
                1 => "UTF-8",
                _ => bail!("Unsupported encoding"),
            };

            let mut first_page = vec![0; page_size as usize];
            file.read_exact_at(&mut first_page, 100)?;

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
