use serde::{Deserialize, Deserializer};
/// A Utility function to represent the difference between undefined/null in serde
///
/// usage with
/// ```
/// struct Example{
///     #[serde(default, deserialize_with = "deserialize_some")]
///    example:Option<Option<String>>
///}
/// ```
/// Any value that is present is considered Some value, including null.
pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}
pub mod crypto;
pub mod error;
pub mod middleware;
pub mod user;
