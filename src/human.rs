use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "std")]
mod standard;

pub const KB: u64 = 1 << 10;
pub const MB: u64 = 1 << 20;
pub const GB: u64 = 1 << 30;
pub const TB: u64 = 1 << 40;

#[derive(Clone, Copy)]
pub enum Unit {
    Bytes,
    Kilo,
    Mega,
    Giga,
    Tera,
}

impl From<Size> for Unit {
    #[inline(always)]
    fn from(value: Size) -> Self {
        match value {
            Size::Bytes(_) => Self::Bytes,
            Size::Kilo(_) => Self::Kilo,
            Size::Mega(_) => Self::Mega,
            Size::Giga(_) => Self::Giga,
            Size::Tera(_) => Self::Tera,
        }
    }
}

/// Structure encoding a size in bytes.
/// This type implements common [Add] and [Sub] traits
/// so that it can be used to make operations on sizes
/// expressed in bytes.
///
/// # Example
///
/// ```
/// use huby::ByteSize;
///
/// let mut kb = ByteSize::from_kb(1);
/// let half = ByteSize::from_kb_f64(0.5);
///
/// assert_eq!(kb + half, ByteSize::from_kb_f64(1.5));
/// kb += half;
/// assert_eq!(kb, ByteSize::from_kb_f64(1.5));
/// kb -= half;
/// assert_eq!(kb, ByteSize::from_kb(1));
/// ```
#[derive(Default, Clone, Copy)]
pub struct ByteSize(Size);

/// Enum encoding a size in bytes, the data carried by
/// the enum is always a value expressed **in bytes**.
#[derive(Clone, Copy)]
#[repr(u64)]
enum Size {
    Bytes(u64),
    Kilo(u64),
    Mega(u64),
    Giga(u64),
    Tera(u64),
}

impl Default for Size {
    fn default() -> Self {
        Self::Bytes(0)
    }
}

impl Eq for ByteSize {}

impl Add for ByteSize {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_bytes(self.in_bytes() + rhs.in_bytes())
    }
}

impl Sub for ByteSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_bytes(self.in_bytes() - rhs.in_bytes())
    }
}

impl AddAssign for ByteSize {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        let res = self.in_bytes() + rhs.in_bytes();
        *self = Self::from_bytes(res)
    }
}

impl SubAssign for ByteSize {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        let res = self.in_bytes() - rhs.in_bytes();
        *self = Self::from_bytes(res)
    }
}

impl PartialEq for ByteSize {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.in_bytes() == other.in_bytes()
    }
}

impl PartialOrd for ByteSize {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.in_bytes().cmp(&other.in_bytes()))
    }
}

impl ByteSize {
    /// Create a [ByteSize] from a given number of bits.
    /// It is not checked where `b` is a multiple of 8.
    #[inline(always)]
    pub const fn from_bits_uncheked(b: u64) -> Self {
        Self::from_bytes(b / 8)
    }

    /// Creates a [ByteSize] from a given number of bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_bytes(1024), ByteSize::from_kb(1));
    /// assert_eq!(ByteSize::from_bytes(4096), ByteSize::from_kb(4));
    /// ```
    #[inline(always)]
    pub const fn from_bytes(b: u64) -> Self {
        if b < KB {
            Self(Size::Bytes(b))
        } else if b < MB {
            Self(Size::Kilo(b))
        } else if b < GB {
            Self(Size::Mega(b))
        } else if b < TB {
            Self(Size::Giga(b))
        } else {
            Self(Size::Tera(b))
        }
    }

    /// Creates a [ByteSize] from a given number of **kilo bytes**.
    ///
    /// # Example
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_kb(1024), ByteSize::from_mb(1));
    /// assert_eq!(ByteSize::from_kb(4096), ByteSize::from_mb(4));
    /// ```
    #[inline(always)]
    pub const fn from_kb(kb: u64) -> Self {
        Self::from_bytes(kb * KB)
    }

    /// Creates a [ByteSize] from a given number of **kilo bytes**.
    ///
    /// # Example
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_kb_f64(1.5), ByteSize::from_kb(1) + ByteSize::from_bytes(512));
    /// ```
    #[inline(always)]
    pub fn from_kb_f64(kb: f64) -> Self {
        Self::from_bytes((kb * KB as f64).round() as u64)
    }

    /// See [ByteSize::from_kb], only change is the parameter is expressed in MB
    #[inline(always)]
    pub const fn from_mb(mb: u64) -> Self {
        Self::from_bytes(mb * MB)
    }

    /// See [ByteSize::from_kb_f64], only change is the parameter is expressed in MB
    #[inline(always)]
    pub fn from_mb_f64(mb: f64) -> Self {
        Self::from_bytes((mb * MB as f64).round() as u64)
    }

    /// See [ByteSize::from_kb], only change is the parameter is expressed in GB
    #[inline(always)]
    pub const fn from_gb(gb: u64) -> Self {
        Self::from_bytes(gb * GB)
    }

    /// See [ByteSize::from_kb_f64], only change is the parameter is expressed in GB
    #[inline(always)]
    pub fn from_gb_f64(gb: f64) -> Self {
        Self::from_bytes((gb * GB as f64).round() as u64)
    }

    /// See [ByteSize::from_kb], only change is the parameter is expressed in TB
    #[inline(always)]
    pub const fn from_tb(gb: u64) -> Self {
        Self::from_bytes(gb * TB)
    }

    /// See [ByteSize::from_kb_f64], only change is the parameter is expressed in TB
    #[inline(always)]
    pub fn from_tb_f64(gb: f64) -> Self {
        Self::from_bytes((gb * TB as f64).round() as u64)
    }

    #[inline(always)]
    const fn unit_str(&self) -> &'static str {
        match self.0 {
            Size::Bytes(_) => "B",
            Size::Kilo(_) => "KB",
            Size::Mega(_) => "MB",
            Size::Giga(_) => "GB",
            Size::Tera(_) => "TB",
        }
    }

    /// Returns the value **in bytes**
    ///
    /// Example:
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_kb_f64(1.5).in_bytes(), 1536);
    /// ```
    #[inline(always)]
    pub const fn in_bytes(&self) -> u64 {
        match self.0 {
            Size::Bytes(b) => b,
            Size::Kilo(b) => b,
            Size::Mega(b) => b,
            Size::Giga(b) => b,
            Size::Tera(b) => b,
        }
    }

    /// Turn [ByteSize] into [ByteSize::Bytes] variant. This
    /// is mostly useful if we want to express the value another
    /// unit. The data carried by the enum does not change.
    ///
    /// Example:
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// let kb = ByteSize::from_kb(1);
    /// assert_eq!(kb.to_string(), "1.0KB");
    /// assert_eq!(kb.in_bytes(), 1024);
    /// assert_eq!(kb.into_bytes().to_string(), "1024.0B");
    /// assert_eq!(kb.in_bytes(), 1024);
    /// ```
    #[inline(always)]
    pub const fn into_bytes(self) -> Self {
        Self(Size::Bytes(self.in_bytes()))
    }

    /// See [ByteSize::into_bytes]
    #[inline(always)]
    pub const fn into_kb(self) -> Self {
        Self(Size::Kilo(self.in_bytes()))
    }

    /// See [ByteSize::into_bytes]
    #[inline(always)]
    pub const fn into_mb(self) -> Self {
        Self(Size::Mega(self.in_bytes()))
    }

    /// See [ByteSize::into_bytes]
    #[inline(always)]
    pub const fn into_gb(self) -> Self {
        Self(Size::Giga(self.in_bytes()))
    }

    /// See [ByteSize::into_bytes]
    #[inline(always)]
    pub const fn into_tb(self) -> Self {
        Self(Size::Tera(self.in_bytes()))
    }

    #[inline(always)]
    const fn divisor(&self) -> f64 {
        match self.0 {
            Size::Bytes(_) => 1.0,
            Size::Kilo(_) => KB as f64,
            Size::Mega(_) => MB as f64,
            Size::Giga(_) => GB as f64,
            Size::Tera(_) => TB as f64,
        }
    }

    /// Normalizes [ByteSize] to fit in the best variant
    ///
    /// # Example
    ///
    /// ```
    /// use huby::{ByteSize, Unit};
    ///
    /// // the best to represent 2048KB is 2MB  
    /// assert!(matches!(ByteSize::from_kb(2048).unit(), Unit::Mega))
    /// ```
    #[inline(always)]
    pub const fn normalize(self) -> Self {
        Self::from_bytes(self.in_bytes())
    }

    /// Returns the value of [ByteSize] expressed in the
    /// unit of the variant.
    ///
    /// # Example
    ///
    /// ```
    /// use huby::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_kb_f64(1.5).in_unit(), 1.5);
    /// assert_eq!(ByteSize::from_kb_f64(1024.0).into_mb().in_unit(), 1.0);
    /// assert_eq!(ByteSize::from_mb_f64(1024.0).into_gb().in_unit(), 1.0);
    /// assert_eq!(ByteSize::from_gb_f64(1024.0).into_tb().in_unit(), 1.0);
    /// ```
    #[inline(always)]
    pub fn in_unit(&self) -> f64 {
        self.in_bytes() as f64 / self.divisor()
    }

    /// Returns the [Unit] of a `ByteSize`
    #[inline(always)]
    pub fn unit(&self) -> Unit {
        self.0.into()
    }
}

#[cfg(test)]
mod test {
    use crate::{human::GB, ByteSize};

    #[test]
    fn test_add() {
        let mut a = ByteSize::from_bytes(12);
        let b = ByteSize::from_bytes(30);
        assert_eq!(a + b, ByteSize::from_bytes(42));
        a += b;
        assert_eq!(a, ByteSize::from_bytes(42));
        assert_eq!((a + ByteSize::from_gb(1)).in_bytes(), GB + 42)
    }

    #[test]
    fn test_into_other_units() {
        let b = ByteSize::from_gb(1000);
        println!("{}", b.into_kb())
    }
}
