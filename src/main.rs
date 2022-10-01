use clap::Parser;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[clap(author,version,about,long_about=None)]
struct Cli {
    path: Vec<PathBuf>,

    #[clap(value_parser)]
    #[clap(short, long)]
    output: std::path::PathBuf,
}

fn main() {
    let cli = &Cli::parse();
    let paths = cli.path.to_vec();
    let mut output_lines: Vec<String> = Vec::new();

    for input_path in paths {
        let path = input_path.as_path();
        let mut file = File::open(&path).expect("[ ERROR ] Couldn't open that file!");
        let mut hasher = Sha256::new();
        let chunk_size = 0x0010_0000; // 1 MiB
                                      // let chunk_size = 1_048_576;
        loop {
            let mut chunk = Vec::with_capacity(chunk_size);
            let n = std::io::Read::by_ref(&mut file)
                .take(chunk_size as u64)
                .read_to_end(&mut chunk)
                .expect("[ ERROR ] Couldn't process that file!");
            if n == 0 {
                break;
            }
            hasher.update(chunk);
        }

        let result = hasher.finalize();
        if let Some(path_str) = path.to_str() {
            output_lines.push(format!("SHA256 ({path_str}) = {:#01x}", result));
        }
    }

    let output_path = cli.output.as_path();
    let mut outfile = match File::create(&output_path) {
        Ok(value) => value,
        Err(_) => panic!(
            "[ ERROR ] Was not able to create the output file: {:?}!",
            output_path
        ),
    };

    for line in output_lines {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Was not able to write to the output file!");
        outfile
            .write_all(b"\n")
            .expect("[ ERROR ] Was not able to write to the output file!");
    }
}
