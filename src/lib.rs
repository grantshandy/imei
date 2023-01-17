#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Display;

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

/// A validated IMEI
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

    /// Get inner IMEI representation
    pub fn into_inner(self) -> I {
        self.inner
    }

    /// Validate an IMEI
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
            n = n * 2;

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
    if sum % 10 == 0 {
        return true;
    } else {
        return false;
    }
}
