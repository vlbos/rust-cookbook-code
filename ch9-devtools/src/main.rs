
// fn execute_query(query: &str) {
//     log::debug!("Executing query: {}", query);
// }

// fn main() {
//     env_logger::init();

//     execute_query("DROP TABLE students");
// }



// fn execute_query(_query: &str) -> Result<(), &'static str> {
//     Err("I'm afraid I can't do that")
// }

// fn main() {
//     env_logger::init();

//     let response = execute_query("DROP TABLE students");
//     if let Err(err) = response {
//         log::error!("Failed to execute query: {}", err);
//     }
// }



// use env_logger::{Builder, Target};

// fn main() {
//     Builder::new()
//         .target(Target::Stdout)
//         .init();

//     log::error!("This error has been printed to Stdout");
// }


// use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

// static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

// struct ConsoleLogger;

// impl log::Log for ConsoleLogger {
//   fn enabled(&self, metadata: &Metadata) -> bool {
//      metadata.level() <= Level::Info
//     }

//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             println!("Rust says: {} - {}", record.level(), record.args());
//         }
//     }

//     fn flush(&self) {}
// }

// fn main() -> Result<(), SetLoggerError> {
//     log::set_logger(&CONSOLE_LOGGER)?;
//     log::set_max_level(LevelFilter::Info);

//     log::info!("hello log");
//     log::warn!("warning");
//     log::error!("oops");
//     Ok(())
// }



// #[cfg(target_os = "linux")]
// #[cfg(target_os = "linux")]
// use syslog::{Facility, Error};

// #[cfg(target_os = "linux")]
// fn main() -> Result<(), Error> {
//     syslog::init(Facility::LOG_USER,
//                  log::LevelFilter::Debug,
//                  Some("My app name"))?;
//     log::debug!("this is a debug {}", "message");
//     log::error!("this is an error!");
//     Ok(())
// }

// #[cfg(not(target_os = "linux"))]
// fn main() {
//     println!("So far, only Linux systems are supported.");
// }



// mod foo {
//     mod bar {
//         pub fn run() {
//             log::warn!("[bar] warn");
//             log::info!("[bar] info");
//             log::debug!("[bar] debug");
//         }
//     }

//     pub fn run() {
//         log::warn!("[foo] warn");
//         log::info!("[foo] info");
//         log::debug!("[foo] debug");
//         bar::run();
//     }
// }

// fn main() {
//     env_logger::init();
//     log::warn!("[root] warn");
//     log::info!("[root] info");
//     log::debug!("[root] debug");
//     foo::run();
// }


// use std::env;
// use env_logger::Builder;

// fn main() {
//     Builder::from_env("MY_APP_LOG")
//         .init();

//     log::info!("informational message");
//     log::warn!("warning message");
//     log::error!("this is an error {}", "message");
// }

// use std::io::Write;
// use chrono::Local;
// use env_logger::Builder;
// use log::LevelFilter;

// fn main() {
//     Builder::new()
//         .format(|buf, record| {
//             writeln!(buf,
//                 "{} [{}] - {}",
//                 Local::now().format("%Y-%m-%dT%H:%M:%S"),
//                 record.level(),
//                 record.args()
//             )
//         })
//         .filter(None, LevelFilter::Info)
//         .init();

//     log::warn!("warn");
//     log::info!("info");
//     log::debug!("debug");
// }


use error_chain::error_chain;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        LogConfig(log4rs::config::runtime::ConfigErrors);
        SetLogger(log::SetLoggerError);
    }
}

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    Ok(())
}


