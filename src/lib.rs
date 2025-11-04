#![no_std]

use core::{cmp::Ordering, ops::Range};

#[cfg(feature = "span-value-usize")]
/// Type of span values
pub type SpanValue = usize;
#[cfg(feature = "span-value-u128")]
/// Type of span values
pub type SpanValue = u128;
#[cfg(feature = "span-value-u64")]
/// Type of span values
pub type SpanValue = u64;
#[cfg(feature = "span-value-u32")]
/// Type of span values
pub type SpanValue = u32;
#[cfg(feature = "span-value-u16")]
/// Type of span values
pub type SpanValue = u16;
#[cfg(feature = "span-value-u8")]
/// Type of span values
pub type SpanValue = u8;

/// The `Span` type represents an area of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// The start of the `Span` (Inclusive)
    pub start: SpanValue,
    /// The end of the `Span` (Exclusive)
    pub end: SpanValue,
}

impl Span {
    /// Creates a new `Span`. This span will start and end at the 0th character, making it have a length of zero.
    #[inline(always)]
    pub fn new() -> Self {
        Self::new_from(0, 0)
    }

    /// Creates a new `Span` from a pair of start and end indexes.
    ///
    /// # Panics
    /// Panics if start is greater than end, since spans can't have a negative length.
    #[inline(always)]
    pub fn new_from(start: SpanValue, end: SpanValue) -> Self {
        assert!(end >= start, "cannot create negative-size span");

        Span { start, end }
    }

    /// Grows the span from the front. This moves the end value up by `amount`.
    #[inline(always)]
    pub fn grow_front(&mut self, amount: SpanValue) {
        self.end += amount;
    }

    /// Returns a span that is grown from the front. This moves the end value up by `amount`.
    #[inline(always)]
    pub fn with_grow_front(&self, amount: SpanValue) -> Self {
        let mut new = *self;
        new.end += amount;
        new
    }

    /// Grows the span from the back. This moves the start value back by `amount`.
    ///
    /// # Panics
    /// Panics if the start of the span is less than `amount`, since spans can't have a negative start value,
    #[inline(always)]
    pub fn grow_back(&mut self, amount: SpanValue) {
        assert!(
            self.start >= amount,
            "cannot create a span with a negative start value"
        );
        self.start -= amount;
    }

    /// Returns a span that is grown from the back. This moves the start value back by `amount`.
    ///
    /// # Panics
    /// Panics if the start of the span is less than `amount`, since spans can't have a negative start value,
    #[inline(always)]
    pub fn with_grow_back(&self, amount: SpanValue) -> Self {
        assert!(
            self.start >= amount,
            "cannot create a span with a negative start value"
        );
        let mut new = *self;
        new.start -= amount;
        new
    }

    /// Shrinks the span from the back. This moves the start value up by `amount`.
    ///
    /// # Panics
    /// Panics if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    #[inline(always)]
    pub fn shrink_back(&mut self, amount: SpanValue) {
        assert!(self.len() >= amount, "cannot create negative-size span");
        self.start += amount;
    }

    /// Returns a span that is shrunk from the back. This moves the start value up by `amount`.
    ///
    /// # Panics
    /// Panics if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    #[inline(always)]
    pub fn with_shrink_back(&self, amount: SpanValue) -> Self {
        assert!(self.len() >= amount, "cannot create negative-size span");
        let mut new = *self;
        new.start += amount;
        new
    }

    /// Shrinks the span from the front. This moves the end value back by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    #[inline(always)]
    pub fn shrink_front(&mut self, amount: SpanValue) {
        assert!(self.len() >= amount, "cannot create negative-size span");
        self.end -= amount;
    }

    /// Returns a span shrunk from the front. This moves the end value back by `amount`.
    ///
    /// # Panics
    /// This method will panic if the size of the `Span` is less than `amount`, since a `Span`'s size can't be negative.
    #[inline(always)]
    pub fn with_shrink_front(&self, amount: SpanValue) -> Self {
        assert!(self.len() >= amount, "cannot create negative-size span");
        let mut new = *self;
        new.end -= amount;
        new
    }

    /// Checks if a `Span`'s size is `0`. Returns `true` if `0`, and false if anything else.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the length of a `Span`.
    #[inline(always)]
    pub fn len(&self) -> SpanValue {
        self.end - self.start
    }

    /// Resets `self` by changing the start to be the end, plus 1, and changing the end to be the start.
    /// The function also returns the old span.
    #[inline(always)]
    pub fn reset(&mut self) -> Self {
        let span = *self;
        self.start = self.end;
        span
    }

    /// Applies the span to `string`, with `start` and `end` corresponding to char indexes.
    ///
    /// # Panics
    /// Panics if `string` is shorter than the end of the span.
    #[allow(clippy::unnecessary_cast)]
    pub fn apply<'a>(&self, string: &'a str) -> &'a str {
        let mut chars = string.char_indices();

        let start = chars
            .nth(self.start as usize)
            .expect("string is too short to have the span applied")
            .0;
        let end = chars
            .nth(self.len() as usize)
            .expect("string is too short to have the span applied")
            .0;
        &string[start..end]
    }

    /// Applies the span to `string`, with `start` and `end` corresponding to byte indexes.
    ///
    /// # Panics
    /// Panics if `string` is shorter than the end of the span.
    #[allow(clippy::unnecessary_cast)]
    pub fn apply_bytes<'a>(&self, string: &'a str) -> &'a str {
        assert!(
            string.len() >= self.end as usize,
            "string is too short to have the span applied"
        );
        &string[(self.start as usize)..(self.end as usize)]
    }
}

impl From<Span> for Range<SpanValue> {
    #[inline(always)]
    fn from(val: Span) -> Self {
        val.start..val.end
    }
}

impl From<Range<SpanValue>> for Span {
    #[inline(always)]
    fn from(value: Range<SpanValue>) -> Self {
        Self::new_from(value.start, value.end)
    }
}

impl PartialOrd for Span {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        dual_order(self.start.cmp(&other.start), self.end.cmp(&other.end))
    }
}

fn dual_order(x: Ordering, y: Ordering) -> Option<Ordering> {
    match (x, y) {
        (x, y) if x == y => Some(x),
        (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
        (x, Ordering::Equal) => Some(x),
        (Ordering::Equal, x) => Some(x),
        _ => unreachable!(),
    }
}

#[cfg(feature = "ariadne")]
impl ariadne::Span for Span {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    #[allow(clippy::unnecessary_cast)]
    fn start(&self) -> usize {
        self.start as usize
    }

    #[allow(clippy::unnecessary_cast)]
    fn end(&self) -> usize {
        self.end as usize
    }
}
