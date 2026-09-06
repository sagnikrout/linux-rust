//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_tv_regs.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

// TV port control

// Enables the TV encoder

// Sources the TV encoder input from pipe B instead of A.

// Outputs composite video (DAC A only)

// Outputs SVideo video (DAC B/C)

// Outputs Component video (DAC A/B/C)

// Outputs Composite and SVideo (DAC A/B/C)

// Enables slow sync generation (945GM only)

// Selects 4x oversampling for 480i and 576p

// Selects 2x oversampling for 720p and 1080i

// Selects no oversampling for 1080p

// Selects 8x oversampling

// Selects progressive mode rather than interlaced

// Sets the colorburst to PAL mode.  Required for non-M PAL modes.

// Field for setting delay of Y compared to C

// Enables a fix for 480p/576p standard definition modes on the 915GM only

//
// Enables a fix for the 915GM only.
//
// Not sure what it does.
//

// Bits that must be preserved by software

// Read-only state that reports all features enabled

// Read-only state that reports that Macrovision is disabled in hardware

// Read-only state that reports that TV-out is disabled in hardware.

// Normal operation

// Encoder test pattern 1 - combo pattern

// Encoder test pattern 2 - full screen vertical 75% color bars

// Encoder test pattern 3 - full screen horizontal 75% color bars

// Encoder test pattern 4 - random noise

// Encoder test pattern 5 - linear color ramps

//
// This test mode forces the DACs to 50% of full output.
//
// This is used for load detection in combination with TVDAC_SENSE_MASK
//

//
// Reports that DAC state change logic has reported change (RO).
//
// This gets cleared when TV_DAC_STATE_EN is cleared
//

// Reports that DAC A voltage is above the detect threshold

// Reports that DAC B voltage is above the detect threshold

// Reports that DAC C voltage is above the detect threshold

//
// Enables DAC state detection logic, for load-based TV detection.
//
// The PLL of the chosen pipe (in TV_CTL) must be running, and the encoder set
// to off, for load detection to work.
//

// Sets the DAC A sense value to high

// Sets the DAC B sense value to high

// Sets the DAC C sense value to high

// Overrides the ENC_ENABLE and DAC voltage levels

// Sets the slew rate.  Must be preserved in software

//
// CSC coefficients are stored in a floating point format with 9 bits of
// mantissa and 2 or 3 bits of exponent.  The exponent is represented as 2**-n,
// where 2-bit exponents are unsigned n, and 3-bit exponents are signed n with
// -1 (0x3) being the only legal negative value.
//

//
// Y attenuation for component video.
//
// Stored in 1.9 fixed point.
//

//
// U attenuation for component video.
//
// Stored in 1.9 fixed point.
//

//
// V attenuation for component video.
//
// Stored in 1.9 fixed point.
//

// 2s-complement brightness adjustment

// Contrast adjustment, as a 2.6 unsigned floating point number

// Saturation adjustment, as a 2.6 unsigned floating point number

// Hue adjustment, as an integer phase angle in degrees

// Controls the DAC level for black

// Controls the DAC level for blanking

// Number of pixels in the hsync.

// Total number of pixels minus one in the line (display and blanking).

// Enables the colorburst (needed for non-component color)

// Offset of the colorburst from the start of hsync, in pixels minus one.

// Length of the colorburst

// End of hblank, measured in pixels minus one from start of hsync

// Start of hblank, measured in pixels minus one from start of hsync

// XXX

// XXX

// XXX

// Length of vsync, in half lines

// Offset of the start of vsync in field 1, measured in one less than the
// number of half lines.
//

//
// Offset of the start of vsync in field 2, measured in one less than the
// number of half lines.
//

// Enables generation of the equalization signal

// Length of vsync, in half lines

// Offset of the start of equalization in field 1, measured in one less than
// the number of half lines.
//

//
// Offset of the start of equalization in field 2, measured in one less than
// the number of half lines.
//

//
// Offset to start of vertical colorburst, measured in one less than the
// number of lines from vertical start.
//

//
// Offset to the end of vertical colorburst, measured in one less than the
// number of lines from the start of NBR.
//

//
// Offset to start of vertical colorburst, measured in one less than the
// number of lines from vertical start.
//

//
// Offset to the end of vertical colorburst, measured in one less than the
// number of lines from the start of NBR.
//

//
// Offset to start of vertical colorburst, measured in one less than the
// number of lines from vertical start.
//

//
// Offset to the end of vertical colorburst, measured in one less than the
// number of lines from the start of NBR.
//

//
// Offset to start of vertical colorburst, measured in one less than the
// number of lines from vertical start.
//

//
// Offset to the end of vertical colorburst, measured in one less than the
// number of lines from the start of NBR.
//

// Turns on the first subcarrier phase generation DDA

// Turns on the first subcarrier phase generation DDA

// Turns on the first subcarrier phase generation DDA

// Sets the subcarrier DDA to reset frequency every other field

// Sets the subcarrier DDA to reset frequency every fourth field

// Sets the subcarrier DDA to reset frequency every eighth field

// Sets the subcarrier DDA to never reset the frequency

// Sets the peak amplitude of the colorburst.

// Sets the increment of the first subcarrier phase generation DDA

// Sets the rollover for the second subcarrier phase generation DDA

// Sets the increent of the second subcarrier phase generation DDA

// Sets the rollover for the third subcarrier phase generation DDA

// Sets the increent of the third subcarrier phase generation DDA

// X coordinate of the display from the start of horizontal active

// Y coordinate of the display from the start of vertical active (NBR)

// Horizontal size of the display window, measured in pixels

//
// Vertical size of the display window, measured in pixels.
//
// Must be even for interlaced modes.
//

//
// Enables automatic scaling calculation.
//
// If set, the rest of the registers are ignored, and the calculated values can
// be read back from the register.
//

//
// Disables the vertical filter.
//
// This is required on modes more than 1024 pixels wide

// Enables adaptive vertical filtering

// Selects the least adaptive vertical filtering mode

// Selects the moderately adaptive vertical filtering mode

// Selects the most adaptive vertical filtering mode

//
// Sets the horizontal scaling factor.
//
// This should be the fractional part of the horizontal scaling factor divided
// by the oversampling rate.  TV_HSCALE should be less than 1, and set to:
//
// (src width - 1) / ((oversample * dest width) - 1)
//

//
// Sets the integer part of the 3.15 fixed-point vertical scaling factor.
//
// TV_VSCALE should be (src height - 1) / ((interlace * dest height) - 1)
//

//
// Sets the fractional part of the 3.15 fixed-point vertical scaling factor.
//
// \sa TV_VSCALE_INT_MASK
//

//
// Sets the integer part of the 3.15 fixed-point vertical scaling factor.
//
// TV_VSCALE should be (src height - 1) / (1/4 * (dest height - 1))
//
// For progressive modes, TV_VSCALE_IP_INT should be set to zeroes.
//

//
// Sets the fractional part of the 3.15 fixed-point vertical scaling factor.
//
// For progressive modes, TV_VSCALE_IP_INT should be set to zeroes.
//
// \sa TV_VSCALE_IP_INT_MASK
//

//
// Specifies which field to send the CC data in.
//
// CC data is usually sent in field 0.
//

// Sets the horizontal position of the CC data.  Usually 135.

// Sets the vertical position of the CC data.  Usually 21

// Second word of CC data to be transmitted.

// First word of CC data to be transmitted.

