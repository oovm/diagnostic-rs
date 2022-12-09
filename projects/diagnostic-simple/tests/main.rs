use std::str::FromStr;

use diagnostic_quick::QResult;

#[test]
fn ready() -> QResult {
    println!("mime: {}", mime::Mime::from_str("text/plain")?);
    println!("mime: {}", semver::Version::from_str("1.0.0.0.pre-release")?);
    println!("it, works!");
    Ok(())
}
