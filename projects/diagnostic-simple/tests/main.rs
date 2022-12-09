use std::str::FromStr;

use diagnostic_quick::QResult;

#[test]
fn ready() -> QResult {
    println!("mime: {}", mime::Mime::from_str("text/plain")?);
    println!("semver: {}", semver::Version::from_str("1.0.0")?);
    println!("toml: {}", toml::Value::from_str("key = true")?);
    println!("url: {}", url::Url::from_str("https://www.google.com")?);
    println!("email_address: {}", email_address::EmailAddress::from_str("mailto:johnstonsk@gmail.com")?);
    println!("it, works!");
    Ok(())
}
