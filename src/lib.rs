//! Pre-rasterized bitmap font from "Noto Sans Mono", an open font from Google. \
//! * Original font files taken from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
//! * License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>
//!
//! Strictly speaking, this crate is more than a basic bitmap font, because it encodes each pixel
//! as a byte and not as a bit, which results in a much nicer result on the screen.
//!
//! ## TL;DR
//! * ✅ `no_std`, zero allocations, no floating point operations
//! * ✅ most important symbols, numbers, and letters as pre-rasterized bitmap
//! * ✅ Noto Sans Mono font as base
//! * ✅ different sizes and font weights (light, normal, bold)
//! * ✅ nice anti-aliasing/smoothing and better looking than legacy bitmap fonts
//! * ✅ every pixel is encoded in a byte (0-255) and not a bit, which results in a much nicer result on the screen.
//! * ✅ relevant font sizes, such as 14, 16, 24, 32, and 64px (as optional build time features)
//! * ✅ zero dependencies
//!
//! ## Terminology: Is Bitmap Font The Right Term?
//! Legacy (8x8) bitmap fonts usually refer to a font where each symbol is encoded in 8 bytes. The ones in a byte
//! (`0b00110000`) means "pixel on" and the zeroes' means "pixel off". However, my bitmap font actually encodes the
//! intensity of each pixel as a byte from 0 to 255. Hence, this is less size efficient than legacy bitmap fonts,
//! but looks much better. I still use the term bitmap font, because that term is used and known when talking
//! about pre-rasterized fonts/font rendering in an early stage of the boot process.
//!
//! ## When To Use This Crate
//! If you develop a kernel, you usually don't want to use the FPU (i.e. only soft float),
//! because otherwise you need to save the floating point registers on every context switch,
//! which is expensive. Because nice font rendering of TTF fonts heavily relies on many
//! floating point operations, this is not optimal inside a kernel (noticeable performance penalties).
//! Furthermore, in my experience it was hard to get some of the popular font rasterization
//! crates to compile with CPU features "+soft-float" and "-sse" (at least on x86_64).
//!
//! Legacy 8x8 bitmap fonts are ugly when printed to the screen. My crate can be seen as a nice
//! replacement with very nice anti-aliasing.
//!
//! If you have a standard environment or support for floating point operations, you might want
//! to rasterize the font by yourself with the crate `fontdue` and some TTF fonts rather than
//! using my crate.
//!
//! ## Minimal Code Example
//! ```rust
//! use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapHeight, FontWeight};
//!
//! // Minimal example.
//!
//! let width = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size16);
//! println!(
//!     "Each char of the mono-spaced font will be {}px in width if the font \
//!      weight={:?} and the bitmap height={}",
//!     width,
//!     FontWeight::Regular,
//!     BitmapHeight::Size16.val()
//! );
//! let bitmap_char = get_bitmap('A', FontWeight::Regular, BitmapHeight::Size16).expect("unsupported char");
//! println!("{:?}", bitmap_char);
//! for (row_i, row) in bitmap_char.bitmap().iter().enumerate() {
//!     for (col_i, pixel) in row.iter().enumerate() {
//!         println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
//!     }
//! }
//! ```
//!
//! ## Cargo Build Time Features
//! If all Cargo features are available, this bitmap fonts supports `light`, `regular`,
//! and `bold`, but no `italic` style, because Noto Sans Mono doesn't have an italic
//! TTF file. The rasterization was done with the awesome [fontdue-Crate](https://crates.io/crates/fontdue).
//!
//! By default, all sizes and font styles/weights are included via the cargo feature `all`.
//! This can be restricted by only using features such as `regular` and `size_14`. Anyhow,
//! a test of mine showed, that including all features in a release build only increases the
//! file size by a few dozen to a few hundred kilobytes. The Rust compiler is really smart
//! throwing out unused parts of the bitmap font, even if they are included as dependency.
//! Your binary will not be bloated by a few megabytes, according to my findings.
//!
//! The bitmap font includes the following unicode range:
//! - BASIC LATIN,
//! - LATIN 1 Supplement
//! - LATIN EXTENDED-A
//!
//! This means unicode symbols from `0 .. 0x17f`, hence letters
//! and symbols from a QWERTZ/QWERTY keyboard plus symbols such as
//! Ö, Ä, and Ü. Control characters are not included.

// # THIS FILE GETS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)

#![no_std]
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

// # THIS FILE GETS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)

mod bold;
mod light;
mod regular;

/// Describes the relevant information for a rendered char of the bitmap font.
///
/// To see why the term "bitmap" is used, see section Terminology in the README.
#[derive(Debug)]
pub struct BitmapChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    ///
    /// To see why the term "bitmap" is used, see section Terminology in the README.
    bitmap: &'static [&'static [u8]],
    /// Height of the bitmap box. The actual font size is slightly smaller.
    height: usize,
    /// The width of the bitmap char. It is guaranteed, that all chars
    /// of the same font weight and bitmap height also have the same width
    /// (as you would expect from a mono font.)
    width: usize,
}

impl BitmapChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    #[inline]
    pub const fn bitmap(&self) -> &'static [&'static [u8]] {
        self.bitmap
    }

    /// Height of the bitmap box. The actual font size is slightly smaller.
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// The width of the bitmap char. It is guaranteed, that all chars
    /// of the same font weight and bitmap height also have the same width
    /// (as you would expect from a mono font).
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }
}

/// Supported font weights.
///
/// The available variants depend on the selected Cargo build features.
#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum FontWeight {
    #[cfg(feature = "light")]
    Light,
    #[cfg(feature = "regular")]
    Regular,
    #[cfg(feature = "bold")]
    Bold,
}

impl FontWeight {
    /// Returns the numeric value of the enum variant.
    #[inline]
    pub const fn val(self) -> usize {
        self as _
    }
}

/// The height of the bitmap font. The font size will be a a few
/// percent less, because each bitmap letter contains vertical padding
/// for proper alignment of chars (i.e. ÄyA). The width of each bitmap
/// character will be also less than the height, because there is no
/// horizontal padding included.
///
/// The available variants depend on the selected Cargo build features.
///
/// To see why the term "bitmap" is used, see section Terminology in the README.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum BitmapHeight {
    #[cfg(feature = "size_14")]
    Size14 = 14,
    #[cfg(feature = "size_16")]
    Size16 = 16,
    #[cfg(feature = "size_18")]
    Size18 = 18,
    #[cfg(feature = "size_20")]
    Size20 = 20,
    #[cfg(feature = "size_22")]
    Size22 = 22,
    #[cfg(feature = "size_24")]
    Size24 = 24,
    #[cfg(feature = "size_32")]
    Size32 = 32,
    #[cfg(feature = "size_64")]
    Size64 = 64,
}

impl BitmapHeight {
    /// Returns the numeric value of the variant.
    #[inline]
    pub const fn val(self) -> usize {
        self as _
    }
}

/// Returns a [`BitmapChar`] for the given char, [`FontWeight`], and [`BitmapHeight`].
///
/// Returns None, if the given char is not known by the bitmap font. In this case,
/// you could fall back to `get_bitmap(' ', ...)`.
///
/// To see why the term "bitmap" is used, see section Terminology in the README.
#[inline]
pub fn get_bitmap(c: char, style: FontWeight, size: BitmapHeight) -> Option<BitmapChar> {
    let bitmap = match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::light::size_14::get_char(c),
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::light::size_16::get_char(c),
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::light::size_18::get_char(c),
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::light::size_20::get_char(c),
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::light::size_22::get_char(c),
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::light::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::light::size_32::get_char(c),
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::light::size_64::get_char(c),
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::regular::size_14::get_char(c),
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::regular::size_16::get_char(c),
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::regular::size_18::get_char(c),
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::regular::size_20::get_char(c),
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::regular::size_22::get_char(c),
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::regular::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::regular::size_32::get_char(c),
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::regular::size_64::get_char(c),
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::bold::size_14::get_char(c),
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::bold::size_16::get_char(c),
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::bold::size_18::get_char(c),
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::bold::size_20::get_char(c),
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::bold::size_22::get_char(c),
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::bold::size_24::get_char(c),
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::bold::size_32::get_char(c),
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::bold::size_64::get_char(c),
        },
    };

    bitmap.map(|bitmap| BitmapChar {
        bitmap,
        height: size.val(),
        width: get_bitmap_width(style, size),
    })
}

/// Returns the width in pixels a char will occupy on the screen. The width is constant for all
/// characters regarding the same combination of [`FontWeight`] and [`BitmapHeight`]. The width is
/// a few percent smaller than the height of each char
///
/// To see why the term "bitmap" is used, see section Terminology in the README.
#[inline]
pub const fn get_bitmap_width(style: FontWeight, size: BitmapHeight) -> usize {
    match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::light::size_14::BITMAP_WIDTH,
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::light::size_16::BITMAP_WIDTH,
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::light::size_18::BITMAP_WIDTH,
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::light::size_20::BITMAP_WIDTH,
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::light::size_22::BITMAP_WIDTH,
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::light::size_24::BITMAP_WIDTH,
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::light::size_32::BITMAP_WIDTH,
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::light::size_64::BITMAP_WIDTH,
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::regular::size_14::BITMAP_WIDTH,
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::regular::size_16::BITMAP_WIDTH,
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::regular::size_18::BITMAP_WIDTH,
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::regular::size_20::BITMAP_WIDTH,
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::regular::size_22::BITMAP_WIDTH,
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::regular::size_24::BITMAP_WIDTH,
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::regular::size_32::BITMAP_WIDTH,
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::regular::size_64::BITMAP_WIDTH,
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_14")]
            BitmapHeight::Size14 => crate::bold::size_14::BITMAP_WIDTH,
            #[cfg(feature = "size_16")]
            BitmapHeight::Size16 => crate::bold::size_16::BITMAP_WIDTH,
            #[cfg(feature = "size_18")]
            BitmapHeight::Size18 => crate::bold::size_18::BITMAP_WIDTH,
            #[cfg(feature = "size_20")]
            BitmapHeight::Size20 => crate::bold::size_20::BITMAP_WIDTH,
            #[cfg(feature = "size_22")]
            BitmapHeight::Size22 => crate::bold::size_22::BITMAP_WIDTH,
            #[cfg(feature = "size_24")]
            BitmapHeight::Size24 => crate::bold::size_24::BITMAP_WIDTH,
            #[cfg(feature = "size_32")]
            BitmapHeight::Size32 => crate::bold::size_32::BITMAP_WIDTH,
            #[cfg(feature = "size_64")]
            BitmapHeight::Size64 => crate::bold::size_64::BITMAP_WIDTH,
        },
    }
}

// # THIS FILE GETS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)
