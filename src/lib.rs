#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Display;

/// A common error for [`Imei`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// IMEI was invalid
    InvalidImei,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidImei => write!(f, "IMEI is invalid"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A type representing a valid IMEI
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Imei<I> {
    inner: I,
}

impl<I: AsRef<str>> Imei<I> {
    /// Try to create a new [Imei]. Fails if the passed IMEI
    /// is invalid.
    pub fn try_new(imei_str: I) -> Result<Self, Error> {
        Imei::validate(imei_str.as_ref()).map(|_| Self { inner: imei_str })
    }

    /// Get inner IMEI representation.
    pub fn into_inner(self) -> I {
        self.inner
    }

    /// Validate an IMEI.
    pub fn validate(imei: I) -> Result<(), Error> {
        if valid(imei) {
            Ok(())
        } else {
            Err(Error::InvalidImei)
        }
    }
}

impl<I: AsRef<str>> Display for Imei<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner.as_ref())
    }
}

/// Check to see if an IMEI number is valid.
pub fn valid<A: AsRef<str>>(imei: A) -> bool {
    let s = imei.as_ref();

    // str::len is acceptable because if s is numeric (therefore valid),
    //   there will not be issues with UTF-8
    if s.len() != 15 {
        return false;
    }

    // the sum of calculated digits
    let mut sum: u8 = 0;

    // go through each character in the imei
    for (i, c) in s.chars().enumerate() {
        // convert the chars into u8
        // I precalculated these so it doesn't have to parse the char as a string
        // It also makes sure that the input is only numeric
        let mut n: u8 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            // return false if the string isn't numeric
            _ => return false,
        };

        // if we are on an odd index (starting at 1)
        if (i + 1) % 2 == 0 {
            // our number is multiplied by two
            n *= 2;

            // if that makes a double digit number make then add together the digits
            if n > 9 {
                n = match n {
                    10 => 1,
                    11 => 2,
                    12 => 3,
                    13 => 4,
                    14 => 5,
                    15 => 6,
                    16 => 7,
                    17 => 8,
                    18 => 9,
                    // we should never get a number larger then 18
                    _ => 0,
                }
            }
        }

        // add it to the sum of calculated digits
        sum += n;
    }

    // return true if we're evenly divisible by 10
    sum % 10 == 0
}

#[cfg(feature = "serde")]
mod serde {
    use crate::{valid, Error, Imei};
    use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

    impl<I> Serialize for Imei<I>
    where
        I: AsRef<str>,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.inner.as_ref())
        }
    }

    impl<'de, I> Deserialize<'de> for Imei<I>
    where
        for<'s> I: From<&'s str>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use std::fmt;
            use std::marker::PhantomData;
            struct ImeiVisitor<I> {
                _marker: PhantomData<I>,
            }

            impl<'d, I> Visitor<'d> for ImeiVisitor<I>
            where
                for<'s> I: From<&'s str>,
            {
                type Value = Imei<I>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a string representing a valid IMEI")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if valid(v) {
                        Ok(Imei { inner: I::from(v) })
                    } else {
                        Err(E::custom(Error::InvalidImei))
                    }
                }
            }

            let visitor: ImeiVisitor<I> = ImeiVisitor {
                _marker: PhantomData,
            };

            deserializer.deserialize_any(visitor)
        }
    }
}

#[cfg(feature = "openapi")]
mod openapi {
    use serde_json::json;
    use utoipa::{
        openapi::{
            response::Response, schema::Schema, ObjectBuilder, RefOr, ResponseBuilder, SchemaType,
        },
        ToResponse, ToSchema,
    };

    use crate::Imei;

    impl<'r, I: AsRef<str>> ToResponse<'r> for Imei<I> {
        fn response() -> (&'r str, RefOr<Response>) {
            (
                "Imei",
                ResponseBuilder::new()
                    .description("A valid International Mobile Equipment Identity number")
                    .build()
                    .into(),
            )
        }
    }

    impl<'s, I: AsRef<str>> ToSchema<'s> for Imei<I> {
        fn schema() -> (&'s str, RefOr<Schema>) {
            (
                "Imei",
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    // IMEI consists of 15 digits
                    .pattern(Some(r"^\d{15}$"))
                    .example(Some(json!("522872587498800")))
                    .into(),
            )
        }
    }
}
