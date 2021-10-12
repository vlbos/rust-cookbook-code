// use std::fs::File;
// use flate2::read::GzDecoder;
// use tar::Archive;

// fn main() -> Result<(), std::io::Error> {
//     let path = "archive.tar.gz";

//     let tar_gz = File::open(path)?;
//     let tar = GzDecoder::new(tar_gz);
//     let mut archive = Archive::new(tar);
//     archive.unpack(".")?;

//     Ok(())
// }


// use std::fs::File;
// use flate2::Compression;
// use flate2::write::GzEncoder;

// fn main() -> Result<(), std::io::Error> {
//     let tar_gz = File::create("archive.tar.gz")?;
//     let enc = GzEncoder::new(tar_gz, Compression::default());
//     let mut tar = tar::Builder::new(enc);
//     tar.append_dir_all(".", "/Users/lisheng/Downloads/misclog")?;
//     Ok(())
// }

use std::fs::File;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;



fn main() -> Result<(), std::io::Error> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, std::io::Error> {
            let path = entry.path()?.strip_prefix(prefix).unwrap().to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}

// use error_chain::error_chain;

// use url::{Url, Position};

// error_chain! {
//     foreign_links {
//         UrlParse(url::ParseError);
//     }
// }

// fn main() -> Result<()> {
//     let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;
//     let cleaned: &str = &parsed[..Position::AfterPath];
//     println!("cleaned: {}", cleaned);
//     Ok(())
// }