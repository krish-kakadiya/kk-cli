use std::{fs::File, io, path::PathBuf};

use clap::{Parser, Subcommand};
use zip::{ZipWriter, write::SimpleFileOptions};

#[derive(Parser)]
#[command(name = "kk", version, about = "KK's CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello,
    Compress {
        file: PathBuf
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => {
            hello();
        },
        Commands::Compress { file } => {
            if !file.exists() {
                eprintln!("File Does Not Exist");
                return;
            }
            println!("Compressing {}",file.display());
            compress_file(&file);
        }
    }
}

fn hello() {
    println!("Hello KK!");
}

fn compress_file(file: &PathBuf) {
    let output = file.with_extension("zip");
    let input = File::open(file).expect("OPen error");
    let mut zip = ZipWriter::new(
        File::create(&output).expect("Failed to Create output fail.")
    );
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let filename = file.file_name().expect("invalid file").to_string_lossy();

    zip.start_file(filename, options).expect("Failed to start Zip file");
    let mut reader = io::BufReader::new(input);
    let _ = io::copy(&mut reader, &mut zip);

    zip.finish().expect("failed to finish zip");
    println!("Compressed: {}", output.display());
}