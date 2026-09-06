//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/s5h1409.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5h1409_config {
// the demodulator's i2c address
    pub demod_address: u8,
// serial/parallel output
pub const S5H1409_PARALLEL_OUTPUT: c_int = 0;
pub const S5H1409_SERIAL_OUTPUT: c_int = 1;
    pub output_mode: u8,
// GPIO Setting
pub const S5H1409_GPIO_OFF: c_int = 0;
pub const S5H1409_GPIO_ON: c_int = 1;
    pub gpio: u8,
// IF Freq for QAM in KHz, VSB is hardcoded to 5380
    pub qam_if: u16,
// Spectral Inversion
pub const S5H1409_INVERSION_OFF: c_int = 0;
pub const S5H1409_INVERSION_ON: c_int = 1;
    pub inversion: u8,
// Return lock status based on tuner lock, or demod lock
pub const S5H1409_TUNERLOCKING: c_int = 0;
pub const S5H1409_DEMODLOCKING: c_int = 1;
    pub status_mode: u8,
// MPEG signal timing
pub const S5H1409_MPEGTIMING_CONTINUOUS_INVERTING_CLOCK: c_int = 0;
pub const S5H1409_MPEGTIMING_CONTINUOUS_NONINVERTING_CLOCK: c_int = 1;
pub const S5H1409_MPEGTIMING_NONCONTINUOUS_INVERTING_CLOCK: c_int = 2;
pub const S5H1409_MPEGTIMING_NONCONTINUOUS_NONINVERTING_CLOCK: c_int = 3;
    pub mpeg_timing: u16,
// HVR-1600 optimizations (to better work with MXL5005s)
pub const S5H1409_HVR1600_NOOPTIMIZE: c_int = 0;
pub const S5H1409_HVR1600_OPTIMIZE: c_int = 1;
    pub hvr1600_opt: u8,
}

