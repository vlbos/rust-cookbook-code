// use error_chain::error_chain;
// use std::io::copy;
// use std::fs::File;
// use tempfile::Builder;

// error_chain! {
//      foreign_links {
//          Io(std::io::Error);
//          HttpRequest(reqwest::Error);
//      }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let tmp_dir = Builder::new().prefix("example").tempdir()?;
//     let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//     let response = reqwest::get(target).await?;

//     let mut dest = {
//         let fname = response
//             .url()
//             .path_segments()
//             .and_then(|segments| segments.last())
//             .and_then(|name| if name.is_empty() { None } else { Some(name) })
//             .unwrap_or("tmp.bin");

//         println!("file to download: '{}'", fname);
//         let fname = tmp_dir.path().join(fname);
//         println!("will be located under: '{:?}'", fname);
//         File::create(fname)?
//     };
//     let content =  response.text().await?;
//     copy(&mut content.as_bytes(), &mut dest)?;
//     Ok(())
// }


// use error_chain::error_chain;
// use std::fs::File;
// use std::io::Read;

//  error_chain! {
//      foreign_links {
//          HttpRequest(reqwest::Error);
//          IoError(::std::io::Error);
//      }
//  }
//  #[tokio::main]

// async fn main() -> Result<()> {
//     let paste_api = "https://paste.rs";
//     let mut file = File::open("message")?;

//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let client = reqwest::Client::new();
//     let res = client.post(paste_api)
//         .body(contents)
//         .send()
//         .await?;
//     let response_text = res.text().await?;
//     println!("Your paste is located at: {}",response_text );
//     Ok(())
// }



// use error_chain::error_chain;
// use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
// use reqwest::StatusCode;
// use std::fs::File;
// use std::str::FromStr;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         Reqwest(reqwest::Error);
//         Header(reqwest::header::ToStrError);
//     }
// }

// struct PartialRangeIter {
//   start: u64,
//   end: u64,
//   buffer_size: u32,
// }

// impl PartialRangeIter {
//   pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
//     if buffer_size == 0 {
//       Err("invalid buffer_size, give a value greater than zero.")?;
//     }
//     Ok(PartialRangeIter {
//       start,
//       end,
//       buffer_size,
//     })
//   }
// }

// impl Iterator for PartialRangeIter {
//   type Item = HeaderValue;
//   fn next(&mut self) -> Option<Self::Item> {
//     if self.start > self.end {
//       None
//     } else {
//       let prev_start = self.start;
//       self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
//       Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
//     }
//   }
// }

// fn main() -> Result<()> {
//   let url = "https://httpbin.org/range/102400?duration=2";
//   const CHUNK_SIZE: u32 = 10240;
    
//   let client = reqwest::blocking::Client::new();
//   let response = client.head(url).send()?;
//   let length = response
//     .headers()
//     .get(CONTENT_LENGTH)
//     .ok_or("response doesn't include the content length")?;
//   let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
    
//   let mut output_file = File::create("download.bin")?;
    
//   println!("starting download...");
//   for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
//     println!("range {:?}", range);
//     let mut response = client.get(url).header(RANGE, range).send()?;
    
//     let status = response.status();
//     if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
//       error_chain::bail!("Unexpected server response: {}", status)
//     }
//     std::io::copy(&mut response, &mut output_file)?;
//   }
    
//   let content = response.text()?;
//   std::io::copy(&mut content.as_bytes(), &mut output_file)?;

//   println!("Finished with success!");
//   Ok(())
// }



use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = "testuser".to_string();
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user_name, password)
        .send();

    println!("=={:?}", response);

    Ok(())
}
