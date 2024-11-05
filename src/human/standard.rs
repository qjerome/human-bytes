use thiserror::Error;

use super::*;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};

#[cfg(feature = "serde")]
impl Serialize for ByteSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string_with_prec(2))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ByteSize {
    fn deserialize<D>(deserializer: D) -> Result<ByteSize, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ByteSizeVisitor;

        impl<'de> Visitor<'de> for ByteSizeVisitor {
            type Value = ByteSize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting a human readable byte size")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse::<ByteSize>().map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_string(ByteSizeVisitor)
    }
}

impl Hash for ByteSize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.in_bytes().hash(state)
    }
}

impl Debug for ByteSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if matches!(self.0, Size::Bytes(_)) {
            write!(f, "{:.1}{}", self.in_unit(), self.unit_str())
        } else {
            write!(
                f,
                "{:.1}{} ({}B)",
                self.in_unit(),
                self.unit_str(),
                self.in_bytes()
            )
        }
    }
}

impl Display for ByteSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bs = {
            // if we are below 1.0 it means we are lacking precision
            // so we must somehow normalize the byte value
            if self.in_unit() < 1.0 {
                self.normalize()
            } else {
                *self
            }
        };
        write!(f, "{:.1}{}", bs.in_unit(), bs.unit_str())
    }
}

impl ByteSize {
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    #[cfg(feature = "serde")]
    fn to_string_with_prec(&self, prec: usize) -> String {
        let bs = {
            // if we are below 1.0 it means we are lacking precision
            // so we must somehow normalize the byte value
            if self.in_unit() < 1.0 {
                self.normalize()
            } else {
                *self
            }
        };
        let s = format!("{:.1$}", bs.in_unit(), prec);
        // we remove trailing zeros
        let f = s.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", f, bs.unit_str())
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unknown unit {0}")]
    UnkUnit(String),
    #[error("parse int: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

impl FromStr for ByteSize {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("KB") {
            Ok(Self::from_kb_f64(
                s.trim_end_matches("KB").trim().parse::<f64>()?,
            ))
        } else if s.ends_with("MB") {
            Ok(Self::from_mb_f64(
                s.trim_end_matches("MB").trim().parse::<f64>()?,
            ))
        } else if s.ends_with("GB") {
            Ok(Self::from_gb_f64(
                s.trim_end_matches("GB").trim().parse::<f64>()?,
            ))
        } else if s.ends_with('B') {
            Ok(Self::from_bytes(
                s.trim_end_matches('B').trim().parse::<u64>()?,
            ))
        } else {
            Err(ParseError::UnkUnit(s.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::ByteSize;

    #[test]
    fn test_from_str() {
        assert_eq!("10B".parse::<ByteSize>().unwrap(), ByteSize::from_bytes(10));
        assert_eq!(
            "10.9KB".parse::<ByteSize>().unwrap(),
            ByteSize::from_kb_f64(10.9)
        );
        assert_eq!(
            "10.1MB".parse::<ByteSize>().unwrap(),
            ByteSize::from_mb_f64(10.1)
        );
        assert_eq!(
            "10.42GB".parse::<ByteSize>().unwrap(),
            ByteSize::from_gb_f64(10.42)
        );
    }

    #[test]
    fn test_to_string_prec() {
        // we try to print some KB as TB. Since resolution is very low
        // to_string_with_prec must make the value KB again for display
        assert_eq!(ByteSize::from_kb(1).into_tb().to_string_with_prec(2), "1KB");
    }

    #[test]
    fn test_serde() {
        #[derive(Serialize, Deserialize)]
        struct T {
            a: ByteSize,
        }

        let t = T {
            a: ByteSize::from_kb_f64(10.42),
        };

        let ser = serde_json::to_string(&t).unwrap();
        assert_eq!(ser, r#"{"a":"10.42KB"}"#);

        let de: T = serde_json::from_str(&ser).unwrap();
        assert_eq!(de.a, ByteSize::from_kb_f64(10.42))
    }
}
