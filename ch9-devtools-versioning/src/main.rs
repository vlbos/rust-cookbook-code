// use semver::{BuildMetadata, Prerelease,Version, Error};

// fn main() -> Result<(), Error> {
//     let mut parsed_version = Version::parse("0.2.6")?;

//     assert_eq!(
//         parsed_version,
//         Version {
//             major: 0,
//             minor: 2,
//             patch: 6,
//             pre: Prerelease::new("alpha.1").unwrap(),
//             build:  BuildMetadata::EMPTY,
//         }
//     );

//     // parsed_version.increment_patch();
//     // assert_eq!(parsed_version.to_string(), "0.2.7");
//     // println!("New patch release: v{}", parsed_version);

//     // parsed_version.increment_minor();
//     // assert_eq!(parsed_version.to_string(), "0.3.0");
//     // println!("New minor release: v{}", parsed_version);

//     // parsed_version.increment_major();
//     // assert_eq!(parsed_version.to_string(), "1.0.0");
//     // println!("New major release: v{}", parsed_version);

//     Ok(())
// }



// use semver::{BuildMetadata, Prerelease, Version, Error};

// fn main() -> Result<(), Error> {
//     let version_str = "1.0.49-125+g72ee7853";
//     let parsed_version = Version::parse(version_str)?;

//     assert_eq!(
//         parsed_version,
//         Version {
//             major: 1,
//             minor: 0,
//             patch: 49,
//             pre: Prerelease::new("125").unwrap(),
//             build:BuildMetadata::new(&String::from("g72ee7853")).unwrap(),
//         }
//     );
//     assert_eq!(
//         parsed_version.build,
//         BuildMetadata::new(&String::from("g72ee7853")).unwrap()
//     );

//     let serialized_version = parsed_version.to_string();
//     assert_eq!(&serialized_version, version_str);

//     Ok(())
// }



// use semver::{Version, Error};

// fn main() -> Result<(), Error> {
//     let version_1 = Version::parse("1.0.0-alpha")?;
//     let version_2 = Version::parse("1.0.0")?;

//     assert!(!version_1.pre.is_empty());
//     assert!(version_2.pre.is_empty());

//     Ok(())
// }


// use error_chain::error_chain;

// use semver::{Version, VersionReq};

// error_chain! {
//     foreign_links {
//         SemVer(semver::Error);
//     }
// }

// fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I) -> Result<Option<Version>>
// where
//     I: IntoIterator<Item = &'a str>,
// {
//     let vreq = VersionReq::parse(version_req_str)?;

//     Ok(
//         iterable
//             .into_iter()
//             .filter_map(|s| Version::parse(s).ok())
//             .filter(|s| vreq.matches(s))
//             .max(),
//     )
// }

// fn main() -> Result<()> {
//     assert_eq!(
//         find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
//         Some(Version::parse("1.0.0")?)
//     );

//     assert_eq!(
//         find_max_matching_version(
//             ">1.2.3-alpha.3",
//             vec![
//                 "1.2.3-alpha.3",
//                 "1.2.3-alpha.4",
//                 "1.2.3-alpha.10",
//                 "1.2.3-beta.4",
//                 "3.4.5-alpha.9",
//             ]
//         )?,
//         Some(Version::parse("1.2.3-beta.4")?)
//     );

//     Ok(())
// }


use error_chain::error_chain;

use std::process::Command;
use semver::{Version, VersionReq};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
        SemVer(semver::Error);
    }
}

fn main() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;
    let output = Command::new("git").arg("--version").output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout.split(" ").collect::<Vec<&str>>()[2];
// .last().ok_or_else(|| {
//         "Invalid command output"
//     })?;
    println!("{:?}",version);
    let parsed_version = Version::parse(version)?;
    
    if !version_test.matches(&parsed_version) {
        error_chain::bail!("Command version lower than minimum supported version (found {}, need {})",
            parsed_version, version_constraint);
    }

    Ok(())
}
