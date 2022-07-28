//! An embedded [Inter-IC Sound (I2S)][wikipedia] abstraction layer.
//!
//! This crate defines generic traits to be implemented by MCU HAL crates.
//!
//! For implementations see [here](https://crates.io/crates/embedded-i2s/reverse_dependencies).
//!
//! [wikipedia]: https://en.wikipedia.org/wiki/I%C2%B2S
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

/// Blocking I2S traits
pub mod blocking {

    /// Blocking I2S trait
    pub trait I2s<W> {
        /// Error type
        type Error: core::fmt::Debug;

        /// Reads enough bytes to fill `left_words` and `right_words`.
        ///
        /// It is allowed for `left_words` and `right_words` to have different lengths.
        /// The read runs for `max(left_words.len(), right_words.len())` words.
        /// Incoming words after the shorter buffer has been filled will be discarded.
        fn read<'w>(
            &mut self,
            left_words: &'w mut [W],
            right_words: &'w mut [W],
        ) -> Result<(), Self::Error>;

        /// Sends `left_words` and `right_words`.
        ///
        /// It is allowed for `left_words` and `right_words` to have different lengths.
        /// The write runs for `max(left_words.len(), right_words.len())` words.
        /// The value of words sent for the shorter channel after its buffer has been sent
        /// is implementation-defined, typically `0x00`, `0xFF`, or configurable.
        fn write<'w>(
            &mut self,
            left_words: &'w [W],
            right_words: &'w [W],
        ) -> Result<(), Self::Error>;

        /// Sends `left_words` and `right_words` getting the data from iterators.
        ///
        /// It is allowed for `left_words` and `right_words` to have different lengths.
        /// The write runs for `max(left_words.len(), right_words.len())` words.
        /// The value of words sent for the shorter channel after its buffer has been sent
        /// is implementation-defined, typically `0x00`, `0xFF`, or configurable.
        fn write_iter<LW, RW>(
            &mut self,
            left_words: LW,
            right_words: RW,
        ) -> Result<(), Self::Error>
        where
            LW: IntoIterator<Item = W>,
            RW: IntoIterator<Item = W>;
    }
}
