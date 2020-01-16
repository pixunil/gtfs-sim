use std::fmt;

use serde::Deserializer;
use serde::de::{Visitor, Error as DeserializeError, Unexpected};

struct NumericBoolVisitor;

impl<'de> Visitor<'de> for NumericBoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("either 0 or 1")
    }

    fn visit_u64<E>(self, value: u64) -> Result<bool, E>
        where E: DeserializeError
    {
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::invalid_value(Unexpected::Unsigned(value), &self)),
        }
    }
}

pub(crate) fn numeric_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_u64(NumericBoolVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::de::IntoDeserializer;
    use serde::de::value::{U64Deserializer, StrDeserializer, Error as ValueError};

    #[test]
    fn test_true() {
        let deserializer: U64Deserializer<ValueError> = 1u64.into_deserializer();
        assert_eq!(numeric_bool(deserializer), Ok(true));
    }

    #[test]
    fn test_false() {
        let deserializer: U64Deserializer<ValueError> = 0u64.into_deserializer();
        assert_eq!(numeric_bool(deserializer), Ok(false));
    }

    #[test]
    fn test_invalid_number() {
        let deserializer: U64Deserializer<ValueError> = 2u64.into_deserializer();
        let error = numeric_bool(deserializer).unwrap_err();
        assert_eq!(error.to_string(), "invalid value: integer `2`, expected either 0 or 1");
    }

    #[test]
    fn test_empty() {
        let deserializer: StrDeserializer<ValueError> = "".into_deserializer();
        let error = numeric_bool(deserializer).unwrap_err();
        assert_eq!(error.to_string(), "invalid type: string \"\", expected either 0 or 1");
    }
}
