//! Representations of TIFF field data types.
//!
//! Each type comes with convenience functions in order
//! to facilitate its use.
//!
//! Every TIFF data type has to implement [`TiffType`] in order to be
//! usable in the crate.
//!
//! [`TiffType`]: trait.TiffType.html

use std::convert::AsRef;
use std::io;

use crate::ifd::values::TiffTypeValues;
use crate::write::EndianFile;

/// A type of data for TIFF fields.
///
/// Other types that might come to exist can be easily implemented by
/// implementing this trait.
pub trait TiffType {
    /// The TIFF 16-bit code that identifies the type.
    fn id() -> u16;

    /// The number of bytes occupied by a single value of this type.
    fn size() -> u32;

    /// The function that writes this type to a given [`EndianFile`].
    ///
    /// # Panics
    ///
    /// Will `panic` if the number of bytes written to the file is
    /// different than the number of bytes specified in [`size()`].
    ///
    /// [`EndianFile`]: ../../struct.EndianFile.html
    /// [`size()`]: #tymethod.size
    fn write_to(self, file: &mut EndianFile) -> io::Result<()>;
}

/// 8-bit unsigned integer.
#[derive(Debug, PartialEq)]
pub struct BYTE(pub u8);
impl BYTE {
    /// Constructs a [`TiffTypeValues`] of `BYTE`s from a vector of
    /// bytes.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[u8]>>(values: T) -> TiffTypeValues<BYTE> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| BYTE(value)).collect())
    }
    /// Constructs a [`TiffTypeValues`] consisting of a single `BYTE`.
    ///
    /// In other words, marks this `BYTE` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: u8) -> TiffTypeValues<BYTE> {
        TiffTypeValues::new(vec![BYTE(value)])
    }
}
impl TiffType for BYTE {
    fn id() -> u16 {
        1
    }
    fn size() -> u32 {
        1
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u8(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`BYTE`] values.
///
/// [`BYTE`]: ifd/types/struct.BYTE.html
#[macro_export]
macro_rules! BYTE {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::BYTE($values)),+])
    };
}

/// 8-bit byte that contains a 7-bit ASCII code.
///
/// According the TIFF specification, the last byte
/// of a field of `ASCII`s must be `NUL` (binary zero, '\0').
#[derive(Debug, PartialEq)]
pub struct ASCII(u8);
impl ASCII {
    /// Constructs a [`TiffTypeValues`] of `ASCII`s from a `&str`.
    ///
    /// If the string doesn't already end with a `NUL` value, it will
    /// be added automatically.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn from_str(s: &str) -> TiffTypeValues<ASCII> {
        let mut values = Vec::with_capacity(s.chars().count());
        for c in s.chars() {
            if c >= (128 as char) {
                panic!("String contains non-ASCII character {}.", c)
            }
            values.push(c as u8);
        }
        Self::values(values)
    }
    /// Constructs a [`TiffTypeValues`] of `ASCII`s from a vector of
    /// bytes.
    ///
    /// If last value isn't already a `NUL` value, a `NUL` value will
    /// be added automatically after the last value.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[u8]>>(values: T) -> TiffTypeValues<ASCII> {
        let values = values.as_ref();
        if values.len() == 0 {
            panic!("Cannot create an empty instance of TiffTypeValues.")
        }

        // TIFF ASCIIs must end with a NUL character.
        // If the user doesn't add it, add it automatically.
        let add_nul = *values.last().unwrap() != 0;
        let mut values: Vec<_> = values.iter().map(|&value| ASCII::new(value)).collect();
        if add_nul {
            values.push(ASCII::new(0))
        }
        TiffTypeValues::new(values)
    }
    /// Creates an `ASCII`s value from a byte.
    ///
    /// # Panics
    ///
    /// An ASCII value only uses 7 bytes. Trying to create an
    /// `ASCII` from values bigger than 127 will `panic`.
    pub fn new(value: u8) -> ASCII {
        if value >= 128 {
            panic!("Tried to create an ASCII encoded by the value {}.\n An ASCII value can only range from 0 to 127.", value);
        }
        ASCII(value)
    }
}
impl TiffType for ASCII {
    fn id() -> u16 {
        2
    }
    fn size() -> u32 {
        1
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u8(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`ASCII`] values.
///
/// [`ASCII`]: ifd/types/struct.ASCII.html
#[macro_export]
macro_rules! ASCII {
    ($string: expr) => {
        ::tiff_encoder::ifd::types::ASCII::from_str($string)
    };
}

/// 16-bit (2-byte) unsigned integer.
#[derive(Debug, PartialEq)]
pub struct SHORT(pub u16);
impl SHORT {
    /// Constructs a [`TiffTypeValues`] of `SHORTS`s from a vector of
    /// `u16`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[u16]>>(values: T) -> TiffTypeValues<SHORT> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| SHORT(value)).collect())
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `SHORT`.
    ///
    /// In other words, marks this `SHORT` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: u16) -> TiffTypeValues<SHORT> {
        TiffTypeValues::new(vec![SHORT(value)])
    }
}
impl TiffType for SHORT {
    fn id() -> u16 {
        3
    }
    fn size() -> u32 {
        2
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u16(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`SHORT`] values.
///
/// [`SHORT`]: ifd/types/struct.SHORT.html
#[macro_export]
macro_rules! SHORT {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::SHORT($values)),+])
    };
}

/// 32-bit (4-byte) unsigned integer.
#[derive(Debug, PartialEq)]
pub struct LONG(pub u32);
impl LONG {
    /// Constructs a [`TiffTypeValues`] of `LONG`s from a vector of
    /// `u32`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[u32]>>(values: T) -> TiffTypeValues<LONG> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| LONG(value)).collect())
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `LONG`.
    ///
    /// In other words, marks this `LONG` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: u32) -> TiffTypeValues<LONG> {
        TiffTypeValues::new(vec![LONG(value)])
    }
}
impl TiffType for LONG {
    fn id() -> u16 {
        4
    }
    fn size() -> u32 {
        4
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u32(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`LONG`] values.
///
/// [`LONG`]: ifd/types/struct.BYTE.html
#[macro_export]
macro_rules! LONG {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::LONG($values)),+])
    };
}

/// Two LONGs representing, respectively, the numerator and the denominator of a fraction.
#[derive(Debug, PartialEq)]
pub struct RATIONAL {
    pub numerator: u32,
    pub denominator: u32,
}
impl RATIONAL {
    /// Constructs a [`TiffTypeValues`] of `RATIONAL`s from a vector of
    /// pairs (numerator, denominator). Both must be `u32` values.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[(u32, u32)]>>(values: T) -> TiffTypeValues<RATIONAL> {
        TiffTypeValues::new(
            values
                .as_ref()
                .iter()
                .map(|&(numerator, denominator)| RATIONAL {
                    numerator,
                    denominator,
                })
                .collect(),
        )
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `RATIONAL`
    /// from a pair (numerator, denominator). Both values must be `u32`.
    ///
    /// In other words, marks this `RATIONAL` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(numerator: u32, denominator: u32) -> TiffTypeValues<RATIONAL> {
        TiffTypeValues::new(vec![RATIONAL {
            numerator,
            denominator,
        }])
    }
}
impl TiffType for RATIONAL {
    fn id() -> u16 {
        5
    }
    fn size() -> u32 {
        8
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u32(self.numerator)?;
        file.write_u32(self.denominator)?;
        Ok(())
    }
}
/// Convenient macro to declare an IFD entry of [`RATIONAL`] values.
///
/// [`RATIONAL`]: ifd/types/struct.RATIONAL.html
#[macro_export]
macro_rules! RATIONAL {
    ($(($num: expr, $den: expr)),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::RATIONAL{numerator: $num, denominator: $den}),+])
    };
}

/// 8-bit signed (twos-complement) integer.
#[derive(Debug, PartialEq)]
pub struct SBYTE(pub i8);
impl SBYTE {
    /// Constructs a [`TiffTypeValues`] of `SBYTE`s from a vector of
    /// `i8`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[i8]>>(values: T) -> TiffTypeValues<SBYTE> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| SBYTE(value)).collect())
    }
    /// Constructs a [`TiffTypeValues`] consisting of a single `SBYTE`.
    ///
    /// In other words, marks this `SBYTE` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: i8) -> TiffTypeValues<SBYTE> {
        TiffTypeValues::new(vec![SBYTE(value)])
    }
}
impl TiffType for SBYTE {
    fn id() -> u16 {
        6
    }
    fn size() -> u32 {
        1
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_i8(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`SBYTE`] values.
///
/// [`SBYTE`]: ifd/types/struct.SBYTE.html
#[macro_export]
macro_rules! SBYTE {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::SBYTE($values)),+])
    };
}

/// 8-bit byte that may contain anything, depending on the definition of the field.
#[derive(Debug, PartialEq)]
pub struct UNDEFINED(pub u8);
impl UNDEFINED {
    /// Constructs a [`TiffTypeValues`] of `UNDEFINED`s from a vector of
    /// bytes.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[u8]>>(values: T) -> TiffTypeValues<UNDEFINED> {
        TiffTypeValues::new(
            values
                .as_ref()
                .iter()
                .map(|&value| UNDEFINED(value))
                .collect(),
        )
    }
    /// Constructs a [`TiffTypeValues`] consisting of a single `UNDEFINED`.
    ///
    /// In other words, marks this `UNDEFINED` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: u8) -> TiffTypeValues<UNDEFINED> {
        TiffTypeValues::new(vec![UNDEFINED(value)])
    }
}
impl TiffType for UNDEFINED {
    fn id() -> u16 {
        7
    }
    fn size() -> u32 {
        1
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u8(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`UNDEFINED`] values.
///
/// [`UNDEFINED`]: ifd/types/struct.UNDEFINED.html
#[macro_export]
macro_rules! UNDEFINED {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::UNDEFINED($values)),+])
    };
}

/// 16-bit (2-byte) signed (twos-complement) integer.
#[derive(Debug, PartialEq)]
pub struct SSHORT(pub i16);
impl SSHORT {
    /// Constructs a [`TiffTypeValues`] of `SSHORT`s from a vector of
    /// `i16`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[i16]>>(values: T) -> TiffTypeValues<SSHORT> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| SSHORT(value)).collect())
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `SSHORT`.
    ///
    /// In other words, marks this `SSHORT` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: i16) -> TiffTypeValues<SSHORT> {
        TiffTypeValues::new(vec![SSHORT(value)])
    }
}
impl TiffType for SSHORT {
    fn id() -> u16 {
        8
    }
    fn size() -> u32 {
        2
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_i16(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`SSHORT`] values.
///
/// [`SSHORT`]: ifd/types/struct.SSHORT.html
#[macro_export]
macro_rules! SSHORT {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::SSHORT($values)),+])
    };
}

/// 32-bit (4-byte) signed (twos-complement) integer.
#[derive(Debug, PartialEq)]
pub struct SLONG(pub i32);
impl SLONG {
    /// Constructs a [`TiffTypeValues`] of `SLONG`s from a vector of
    /// `i32`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[i32]>>(values: T) -> TiffTypeValues<SLONG> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| SLONG(value)).collect())
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `SLONG`.
    ///
    /// In other words, marks this `SLONG` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: i32) -> TiffTypeValues<SLONG> {
        TiffTypeValues::new(vec![SLONG(value)])
    }
}
impl TiffType for SLONG {
    fn id() -> u16 {
        9
    }
    fn size() -> u32 {
        4
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_i32(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`SLONG`] values.
///
/// [`SLONG`]: ifd/types/struct.SLONG.html
#[macro_export]
macro_rules! SLONG {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::SLONG($values)),+])
    };
}

/// Two SLONGs representing, respectively, the numerator and the denominator of a fraction.
#[derive(Debug, PartialEq)]
pub struct SRATIONAL {
    pub numerator: i32,
    pub denominator: i32,
}
impl SRATIONAL {
    /// Constructs a [`TiffTypeValues`] of `SRATIONAL`s from a vector of
    /// pairs (numerator, denominator). Both must be `i32` values.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[(i32, i32)]>>(values: T) -> TiffTypeValues<SRATIONAL> {
        TiffTypeValues::new(
            values
                .as_ref()
                .iter()
                .map(|&(numerator, denominator)| SRATIONAL {
                    numerator,
                    denominator,
                })
                .collect(),
        )
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `SRATIONAL`
    /// from a pair (numerator, denominator). Both values must be `i32`.
    ///
    /// In other words, marks this `SRATIONAL` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(numerator: i32, denominator: i32) -> TiffTypeValues<SRATIONAL> {
        TiffTypeValues::new(vec![SRATIONAL {
            numerator,
            denominator,
        }])
    }
}
impl TiffType for SRATIONAL {
    fn id() -> u16 {
        10
    }
    fn size() -> u32 {
        8
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_i32(self.numerator)?;
        file.write_i32(self.denominator)?;
        Ok(())
    }
}
/// Convenient macro to declare an IFD entry of [`SRATIONAL`] values.
///
/// [`SRATIONAL`]: ifd/types/struct.SRATIONAL.html
#[macro_export]
macro_rules! SRATIONAL {
    ($(($num: expr, $den: expr)),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::SRATIONAL{numerator: $num, denominator: $den}),+])
    };
}

/// Single precision (4-byte) IEEE format.
#[derive(Debug, PartialEq)]
pub struct FLOAT(pub f32);
impl FLOAT {
    /// Constructs a [`TiffTypeValues`] of `FLOAT`s from a vector of
    /// `f32`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[f32]>>(values: T) -> TiffTypeValues<FLOAT> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| FLOAT(value)).collect())
    }

    /// Constructs a [`TiffTypeValues`] consisting of a single `FLOAT`.
    ///
    /// In other words, marks this `FLOAT` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: f32) -> TiffTypeValues<FLOAT> {
        TiffTypeValues::new(vec![FLOAT(value)])
    }
}
impl TiffType for FLOAT {
    fn id() -> u16 {
        11
    }
    fn size() -> u32 {
        4
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_f32(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`FLOAT`] values.
///
/// [`FLOAT`]: ifd/types/struct.FLOAT.html
#[macro_export]
macro_rules! FLOAT {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::FLOAT($values)),+])
    };
}

/// Double precision (8-byte) IEEE format.
#[derive(Debug, PartialEq)]
pub struct DOUBLE(pub f64);
impl DOUBLE {
    /// Constructs a [`TiffTypeValues`] of `DOUBLE`s from a vector of
    /// `f64`.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn values<T: AsRef<[f64]>>(values: T) -> TiffTypeValues<DOUBLE> {
        TiffTypeValues::new(values.as_ref().iter().map(|&value| DOUBLE(value)).collect())
    }
    /// Constructs a [`TiffTypeValues`] consisting of a single `DOUBLE`.
    ///
    /// In other words, marks this `DOUBLE` as the single value of its
    /// field.
    ///
    /// [`TiffTypeValues`]: ../values/struct.TiffTypeValues.html
    pub fn single(value: f64) -> TiffTypeValues<DOUBLE> {
        TiffTypeValues::new(vec![DOUBLE(value)])
    }
}
impl TiffType for DOUBLE {
    fn id() -> u16 {
        12
    }
    fn size() -> u32 {
        8
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_f64(self.0)
    }
}
/// Convenient macro to declare an IFD entry of [`DOUBLE`] values.
///
/// [`DOUBLE`]: ifd/types/struct.DOUBLE.html
#[macro_export]
macro_rules! DOUBLE {
    ($($values: expr),+) => {
        ::tiff_encoder::ifd::values::TiffTypeValues::new(vec![$(::tiff_encoder::ifd::types::DOUBLE($values)),+])
    };
}

/// 32-bit (4-byte) unsigned integer used exclusively to point to IFDs.
///
/// This type is not supposed to be used directly. See [`OffsetsToIfds`].
///
/// [`OffsetsToIfds`]: ../values/struct.OffsetsToIfds.html
#[derive(Debug, PartialEq)]
pub struct IFD(pub(crate) u32);
impl TiffType for IFD {
    fn id() -> u16 {
        13
    }
    fn size() -> u32 {
        4
    }
    fn write_to(self, file: &mut EndianFile) -> io::Result<()> {
        file.write_u32(self.0)
    }
}
