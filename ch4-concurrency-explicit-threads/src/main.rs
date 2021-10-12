use error_chain::error_chain;

use std::path::Path;
use std::fs::create_dir_all;

use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{imageops::FilterType, ImageError};
use rayon::prelude::*;

error_chain! {
    foreign_links {
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        error_chain::bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}



// use rayon::prelude::*;

// struct Person {
//     age: u32,
// }

// fn main() {
//     let v: Vec<Person> = vec![
//         Person { age: 23 },
//         Person { age: 19 },
//         Person { age: 42 },
//         Person { age: 17 },
//         Person { age: 17 },
//         Person { age: 31 },
//         Person { age: 30 },
//     ];

//     let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
//     let sum_over_30 = v.par_iter()
//         .map(|x| x.age)
//         .filter(|&x| x > 30)
//         .reduce(|| 0, |x, y| x + y);

//     let alt_sum_30: u32 = v.par_iter()
//         .map(|x| x.age)
//         .filter(|&x| x > 30)
//         .sum();

//     let avg_over_30 = sum_over_30 as f32 / num_over_30;
//     let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30;

//     assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
//     println!("The average age of people older than 30 is {}", avg_over_30);
// }




// use rand::{Rng, thread_rng};
// use rand::distributions::Alphanumeric;
// use rayon::prelude::*;

// fn main() {
//   let mut vec = vec![String::new(); 100_000];
//   vec.par_iter_mut().for_each(|p| {
//     let mut rng = thread_rng();
//     *p = (0..5).map(|_| rng.sample(&Alphanumeric)).map(|c|c as char).collect()
//   });
//   vec.par_sort_unstable();
// }

// use rayon::prelude::*;

// fn main() {
//     let v = vec![6, 2, 1, 9, 3, 8, 11];

//     let f1 = v.par_iter().find_any(|&&x| x == 9);
//     let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
//     let f3 = v.par_iter().find_any(|&&x| x > 8);

//     assert_eq!(f1, Some(&9));
//     assert_eq!(f2, Some(&8));
//     assert!(f3 > Some(&8));
// }

// use rayon::prelude::*;

// fn main() {
//     let mut vec = vec![2, 4, 6, 8];

//     assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
//     assert!(vec.par_iter().all(|n| (*n % 2) == 0));
//     assert!(!vec.par_iter().any(|n| *n > 8 ));
//     assert!(vec.par_iter().all(|n| *n <= 8 ));

//     vec.push(9);

//     assert!(vec.par_iter().any(|n| (*n % 2) != 0));
//     assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
//     assert!(vec.par_iter().any(|n| *n > 8 ));
//     assert!(!vec.par_iter().all(|n| *n <= 8 )); 
// }

// use rayon::prelude::*;

// fn main() {
//     let mut arr = [0, 7, 9, 11];
//     arr.par_iter_mut().for_each(|p| *p -= 1);
//     println!("{:?}", arr);
// }



// use error_chain::error_chain;
// use std::sync::mpsc::{channel, RecvError};
// use threadpool::ThreadPool;
// use num::complex::Complex;
// use image::{ImageBuffer, Pixel, Rgb};

// error_chain! {
//     foreign_links {
//         MpscRecv(RecvError);
//         Io(std::io::Error);
//         ImageBuffer(image::ImageError);
//     }
// }

// // Function converting intensity values to RGB
// // Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
// fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
//     let wave = wavelength as f32;

//     let (r, g, b) = match wavelength {
//         380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
//         440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
//         490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
//         510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
//         580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
//         645..=780 => (1.0, 0.0, 0.0),
//         _ => (0.0, 0.0, 0.0),
//     };

//     let factor = match wavelength {
//         380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
//         701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
//         _ => 1.0,
//     };

//     let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
//     Rgb::from_channels(r, g, b, 0)
// }

// // Maps Julia set distance estimation to intensity values
// fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
//     let width = width as f32;
//     let height = height as f32;

//     let mut z = Complex {
//         // scale and translate the point to image coordinates
//         re: 3.0 * (x as f32 - 0.5 * width) / width,
//         im: 2.0 * (y as f32 - 0.5 * height) / height,
//     };

//     let mut i = 0;
//     for t in 0..max_iter {
//         if z.norm() >= 2.0 {
//             break;
//         }
//         z = z * z + c;
//         i = t;
//     }
//     i
// }

// // Normalizes color intensity values within RGB range
// fn normalize(color: f32, factor: f32) -> u8 {
//     ((color * factor).powf(0.8) * 255.) as u8
// }

// fn main() -> Result<()> {
//     let (width, height) = (1920, 1080);
//     let mut img = ImageBuffer::new(width, height);
//     let iterations = 300;

//     let c = Complex::new(-0.8, 0.156);

//     let pool = ThreadPool::new(num_cpus::get());
//     let (tx, rx) = channel();

//     for y in 0..height {
//         let tx = tx.clone();
//         pool.execute(move || for x in 0..width {
//                          let i = julia(c, x, y, width, height, iterations);
//                          let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
//                          tx.send((x, y, pixel)).expect("Could not send data!");
//                      });
//     }

//     for _ in 0..(width * height) {
//         let (x, y, pixel) = rx.recv()?;
//         img.put_pixel(x, y, pixel);
//     }
//     let _ = img.save("output.png")?;
//     Ok(())
// }


// use walkdir::WalkDir;
// use std::fs::File;
// use std::io::{BufReader, Read, Error};
// use std::path::Path;
// use threadpool::ThreadPool;
// use std::sync::mpsc::channel;
// use ring::digest::{Context, Digest, SHA256};

// // Verify the iso extension
// fn is_iso(entry: &Path) -> bool {
//     match entry.extension() {
//         Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
//         _ => false,
//     }
// }

// fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
//     let mut buf_reader = BufReader::new(File::open(&filepath)?);
//     let mut context = Context::new(&SHA256);
//     let mut buffer = [0; 1024];

//     loop {
//         let count = buf_reader.read(&mut buffer)?;
//         if count == 0 {
//             break;
//         }
//         context.update(&buffer[..count]);
//     }

//     Ok((context.finish(), filepath))
// }

// fn main() -> Result<(), Error> {
//     let pool = ThreadPool::new(num_cpus::get());

//     let (tx, rx) = channel();

//     for entry in WalkDir::new("/Users/lisheng/Downloads")
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
//             let path = entry.path().to_owned();
//             let tx = tx.clone();
//             pool.execute(move || {
//                 let digest = compute_digest(path);
//                 tx.send(digest).expect("Could not send data!");
//             });
//         }

//     drop(tx);
//     for t in rx.iter() {
//         let (sha, path) = t?;
//         println!("{:?} {:?}", sha, path);
//     }
//     Ok(())
// }


// use lazy_static::lazy_static;
// use std::sync::Mutex;
//  use std::io::Error;
// lazy_static! {
//     static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
// }

// fn insert(fruit: &str) -> Result<(),std::io::Error> {
//     let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();
//     db.push(fruit.to_string());
//     Ok(())
// }

// fn main() -> Result<(),std::io::Error> {
//     insert("apple")?;
//     insert("orange")?;
//     insert("peach")?;
//     {
//         let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard").unwrap();

//         db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
//     }
//     insert("grape")?;
//     Ok(())
// }



// use std::{thread, time};
// use crossbeam_channel::unbounded;

// fn main() {
//     let (snd, rcv) = unbounded();
//     let n_msgs = 5;
//     crossbeam::scope(|s| {
//         s.spawn(|_| {
//             for i in 0..n_msgs {
//                 snd.send(i).unwrap();
//                 thread::sleep(time::Duration::from_millis(100));
//             }
//         });
//     }).unwrap();
//     for _ in 0..n_msgs {
//         let msg = rcv.recv().unwrap();
//         println!("Received {}", msg);
//     }
// }


// extern crate crossbeam;
// extern crate crossbeam_channel;

// use std::thread;
// use std::time::Duration;
// use crossbeam_channel::bounded;

// fn main() {
//     let (snd1, rcv1) = bounded(1);
//     let (snd2, rcv2) = bounded(1);
//     let n_msgs = 4;
//     let n_workers = 2;

//     crossbeam::scope(|s| {
//         // Producer thread
//         s.spawn(|_| {
//             for i in 0..n_msgs {
//                 snd1.send(i).unwrap();
//                 println!("Source sent {}", i);
//             }
//             // Close the channel - this is necessary to exit
//             // the for-loop in the worker
//             drop(snd1);
//         });

//         // Parallel processing by 2 threads
//         for _ in 0..n_workers {
//             // Send to sink, receive from source
//             let (sendr, recvr) = (snd2.clone(), rcv1.clone());
//             // Spawn workers in separate threads
//             s.spawn(move |_| {
//             thread::sleep(Duration::from_millis(500));
//                 // Receive until channel closes
//                 for msg in recvr.iter() {
//                     println!("Worker {:?} received {}.",
//                              thread::current().id(), msg);
//                     sendr.send(msg * 2).unwrap();
//                 }
//             });
//         }
//         // Close the channel, otherwise sink will never
//         // exit the for-loop
//         drop(snd2);

//         // Sink
//         for msg in rcv2.iter() {
//             println!("Sink received {}", msg);
//         }
//     }).unwrap();
// }


// fn main() {
//     let arr = &[1, 25, -4, 10];
//     let max = find_max(arr);
//     assert_eq!(max, Some(25));
// }

// fn find_max(arr: &[i32]) -> Option<i32> {
//     const THRESHOLD: usize = 2;
  
//     if arr.len() <= THRESHOLD {
//         return arr.iter().cloned().max();
//     }

//     let mid = arr.len() / 2;
//     let (left, right) = arr.split_at(mid);
  
//     crossbeam::scope(|s| {
//         let thread_l = s.spawn(|_| find_max(left));
//         let thread_r = s.spawn(|_| find_max(right));
  
//         let max_l = thread_l.join().unwrap()?;
//         let max_r = thread_r.join().unwrap()?;
  
//         Some(max_l.max(max_r))
//     }).unwrap()
// }

