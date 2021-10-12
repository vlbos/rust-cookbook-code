
// use url::{Url, ParseError};

// fn main() -> Result<(), ParseError> {
//     let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

//     let parsed = Url::parse(s)?;
//     println!("The path part of the URL is: {}", parsed.path());

//     Ok(())
// }


// use error_chain::error_chain;

// use url::Url;

// error_chain! {
//     foreign_links {
//         UrlParse(url::ParseError);
//     }
//     errors {
//         CannotBeABase
//     }
// }

// fn main() -> Result<()> {
//     let full = "https://github.com/rust-lang/cargo?asdf";

//     let url = Url::parse(full)?;
//     let base = base_url(url)?;

//     assert_eq!(base.as_str(), "https://github.com/");
//     println!("The base of the URL is: {}", base);

//     Ok(())
// }

// fn base_url(mut url: Url) -> Result<Url> {
//     match url.path_segments_mut() {
//         Ok(mut path) => {
//             path.clear();
//         }
//         Err(_) => {
//             return Err(Error::from_kind(ErrorKind::CannotBeABase));
//         }
//     }

//     url.set_query(None);

//     Ok(url)
// }





// use url::{Url, ParseError};

// fn main() -> Result<(), ParseError> {
//     let path = "/rust-lang/cargo";

//     let gh = build_github_url(path)?;

//     assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
//     println!("The joined URL is: {}", gh);

//     Ok(())
// }

// fn build_github_url(path: &str) -> Result<Url, ParseError> {
//     const GITHUB: &'static str = "https://github.com";

//     let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
//     let joined = base.join(path)?;

//     Ok(joined)
// }



// use url::{Url, Host, ParseError};

// fn main() -> Result<(), ParseError> {
//     let s = "ftp://rust-lang.org/examples";

//     let url = Url::parse(s)?;

//     assert_eq!(url.scheme(), "ftp");
//     assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
//     assert_eq!(url.port_or_known_default(), Some(21));
//     println!("The origin is as expected!");

//     Ok(())
// }

// use error_chain::error_chain;

// use url::{Url, Origin, Host};

// error_chain! {
//     foreign_links {
//         UrlParse(url::ParseError);
//     }
// }

// fn main() -> Result<()> {
//     let s = "ftp://rust-lang.org/examples";

//     let url = Url::parse(s)?;

//     let expected_scheme = "ftp".to_owned();
//     let expected_host = Host::Domain("rust-lang.org".to_owned());
//     let expected_port = 21;
//     let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

//     let origin = url.origin();
//     assert_eq!(origin, expected);
//     println!("The origin is as expected!");

//     Ok(())
// }





use url::{Url, Position, ParseError};

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}

