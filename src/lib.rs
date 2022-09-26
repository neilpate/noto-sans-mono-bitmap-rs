//! Provides pre-rasterized characters from the "Noto Sans Mono" font in different sizes and font
//! weights for multiple unicode ranges. This crate is `no_std` and needs no allocations or floating
//! point operations. Useful in kernels and bootloaders when only "soft-float" is available. Strictly
//! speaking, this crate is more than a basic bitmap font, because it encodes each pixel as a byte
//! and not as a bit, which results in a much nicer result on the screen.
//!
//! * Original font files taken from: <https://fonts.google.com/noto/specimen/Noto+Sans+Mono>
//! * License: SIL Open Font License (OFL) <https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL>
//!
//! ## TL;DR
//! * ✅ `no_std`, zero allocations, no floating point operations
//! * ✅ most important symbols, numbers, and letters as pre-rasterized bitmap. Unicode-ranges are selectable.
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
//! use noto_sans_mono_bitmap::{get_raster, get_raster_width, RasterHeight, FontWeight};
//!
//! // Minimal example.
//!
//! let width = get_raster_width(FontWeight::Regular, RasterHeight::Size14);
//! println!(
//!     "Each char of the mono-spaced font will be {}px in width if the font \
//!      weight is {:?} and the height is {}",
//!     width,
//!     FontWeight::Regular,
//!     RasterHeight::Size14.val()
//! );
//! let char_raster = get_raster('A', FontWeight::Regular, RasterHeight::Size14).expect("unsupported char");
//! println!("{:?}", char_raster);
//! for (row_i, row) in char_raster.raster().iter().enumerate() {
//!     for (col_i, pixel) in row.iter().enumerate() {
//!         println!("[{:02}][{:02}]: {:03}", row_i, col_i, pixel);
//!     }
//! }
//! ```
//!
//! ## Cargo Features and Crate Size
//! By default, only a reasonable subset of possible features is included. The raw crate-size is a few
//! MiB in size but after compilation and discarding irrelevant parts (i.e., size 14, regular font,
//! only ASCII), the overhead should be at less than 120 KiB in binary size, according to my
//! measurements. The compiler can reliably discard unused sizes or weights, but not so for unicode
//! ranges. Thus, it is recommended to include no more features than necessary.
//!
//! With all features included inside the binary, and without any discarding by the compiler, you
//! can expect 5 or more MiB of memory requirements. However, this would require the rather unlikely
//! case that you use different sizes and font weights simulatnously. etc.
//!
//! For a full support of all unicode ranges, use an on-the-fly rasterization process instead of this
//! crate.

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

/// Describes the relevant information for a rendered char of the font.
#[derive(Debug)]
pub struct RasterizedChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    raster: &'static [&'static [u8]],
    /// Height of the raster box. The actual font size is slightly smaller.
    height: usize,
    /// The width of the rasterized char. It is guaranteed, that all chars
    /// of the same font weight and raster height also have the same width
    /// (as you would expect from a mono font.)
    width: usize,
}

impl RasterizedChar {
    /// The actual font data that is `height` * `width` bytes in size.
    /// Each byte describes the intensity of a pixel from 0 to 255.
    #[inline]
    pub const fn raster(&self) -> &'static [&'static [u8]] {
        self.raster
    }

    /// Height of the raster box. The actual font size is slightly smaller.
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// The width of the raster char. It is guaranteed, that all chars
    /// of the same font weight and raster height also have the same width
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

/// The height of the pre-rasterized font. The font size will be a a few
/// percent less, because each letter contains vertical padding for proper
/// alignment of chars (i.e. ÄyA). The width of each character will be also
/// less than the height, because there is no horizontal padding included.
///
/// The available variants depend on the selected Cargo build features.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum RasterHeight {
    #[cfg(feature = "size_14")]
    Size14 = 14,
    #[cfg(feature = "size_18")]
    Size18 = 18,
    #[cfg(feature = "size_22")]
    Size22 = 22,
    #[cfg(feature = "size_32")]
    Size32 = 32,
}

impl RasterHeight {
    /// Returns the numeric value of the variant.
    #[inline]
    pub const fn val(self) -> usize {
        self as _
    }
}

/// Returns a [`RasterizedChar`] for the given char, [`FontWeight`], and [`RasterHeight`].
///
/// Returns None, if the given char is not known by the font. In this case,
/// you could fall back to `get_raster(' ', ...)`.
#[inline]
pub fn get_raster(c: char, style: FontWeight, size: RasterHeight) -> Option<RasterizedChar> {
    let raster = match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::light::size_14::get_char(c),
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::light::size_18::get_char(c),
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::light::size_22::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::light::size_32::get_char(c),
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::regular::size_14::get_char(c),
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::regular::size_18::get_char(c),
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::regular::size_22::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::regular::size_32::get_char(c),
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::bold::size_14::get_char(c),
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::bold::size_18::get_char(c),
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::bold::size_22::get_char(c),
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::bold::size_32::get_char(c),
        },
    };

    raster.map(|raster| RasterizedChar {
        raster,
        height: size.val(),
        width: get_raster_width(style, size),
    })
}

/// Returns the width in pixels a char will occupy on the screen. The width is constant for all
/// characters regarding the same combination of [`FontWeight`] and [`RasterHeight`]. The width is
/// a few percent smaller than the height of each char
#[inline]
pub const fn get_raster_width(style: FontWeight, size: RasterHeight) -> usize {
    match style {
        #[cfg(feature = "light")]
        FontWeight::Light => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::light::size_14::RASTER_WIDTH,
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::light::size_18::RASTER_WIDTH,
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::light::size_22::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::light::size_32::RASTER_WIDTH,
        },
        #[cfg(feature = "regular")]
        FontWeight::Regular => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::regular::size_14::RASTER_WIDTH,
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::regular::size_18::RASTER_WIDTH,
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::regular::size_22::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::regular::size_32::RASTER_WIDTH,
        },
        #[cfg(feature = "bold")]
        FontWeight::Bold => match size {
            #[cfg(feature = "size_14")]
            RasterHeight::Size14 => crate::bold::size_14::RASTER_WIDTH,
            #[cfg(feature = "size_18")]
            RasterHeight::Size18 => crate::bold::size_18::RASTER_WIDTH,
            #[cfg(feature = "size_22")]
            RasterHeight::Size22 => crate::bold::size_22::RASTER_WIDTH,
            #[cfg(feature = "size_32")]
            RasterHeight::Size32 => crate::bold::size_32::RASTER_WIDTH,
        },
    }
}

// # THIS FILE GETS AUTO GENERATED BY THE PROJECT IN "../codegen" (see repository!)
