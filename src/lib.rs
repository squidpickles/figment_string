#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

struct StringVisitor;

impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or string-convertible value")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.to_owned())
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(value.to_string())
    }
}

/// Deserialize a string or string-convertible value as a [`String`].
///
/// ### Example:
/// ```rust
/// # use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct Config{
///     #[serde(deserialize_with = "figment_string::deserialize_as_string")]
///     pub name: String,
/// }
/// ```
pub fn deserialize_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_any(StringVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use figment::Figment;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Config {
        #[serde(deserialize_with = "deserialize_as_string")]
        pub name: String,
    }

    #[test]
    fn test_int() {
        temp_env::with_var("NAME", Some("8080"), || {
            let config: Config = Figment::new()
                .merge(figment::providers::Env::raw())
                .extract()
                .unwrap();
            assert_eq!(config.name, "8080");
        });
    }

    #[test]
    fn test_negative_int() {
        temp_env::with_var("NAME", Some("-8080"), || {
            let config: Config = Figment::new()
                .merge(figment::providers::Env::raw())
                .extract()
                .unwrap();
            assert_eq!(config.name, "-8080");
        });
    }

    #[test]
    fn test_float() {
        temp_env::with_var("NAME", Some("80.822"), || {
            let config: Config = Figment::new()
                .merge(figment::providers::Env::raw())
                .extract()
                .unwrap();
            assert_eq!(config.name, "80.822");
        });
    }

    #[test]
    fn test_bool() {
        temp_env::with_var("NAME", Some("true"), || {
            let config: Config = Figment::new()
                .merge(figment::providers::Env::raw())
                .extract()
                .unwrap();
            assert_eq!(config.name, "true");
        });
    }

    #[test]
    fn test_str() {
        temp_env::with_var("NAME", Some("foo"), || {
            let config: Config = Figment::new()
                .merge(figment::providers::Env::raw())
                .extract()
                .unwrap();
            assert_eq!(config.name, "foo");
        });
    }
}
