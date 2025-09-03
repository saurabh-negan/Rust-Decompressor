use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::io::{self, copy, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <source.gz> <target>", args[0]);
        std::process::exit(1);
    }

    let source = &args[1];
    let target = &args[2];

    // Open the compressed file
    let input_file = File::open(source)?;
    let buffered = BufReader::new(input_file);

    // Wrap in GzDecoder (will decompress while reading)
    let mut decoder = GzDecoder::new(buffered);

    // Create the output file
    let mut output_file = File::create(target)?;

    let start = Instant::now();

    // Copy data from decoder (compressed input) → output file (decompressed)
    copy(&mut decoder, &mut output_file)?;

    println!("Decompressed {:?} → {:?} in {:?}", source, target, start.elapsed());

    Ok(())
}
