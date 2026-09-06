//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/s5h1411.h
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
pub struct s5h1411_config {
// serial/parallel output
pub const S5H1411_PARALLEL_OUTPUT: c_int = 0;
pub const S5H1411_SERIAL_OUTPUT: c_int = 1;
    pub output_mode: u8,
// GPIO Setting
pub const S5H1411_GPIO_OFF: c_int = 0;
pub const S5H1411_GPIO_ON: c_int = 1;
    pub gpio: u8,
// MPEG signal timing
pub const S5H1411_MPEGTIMING_CONTINUOUS_INVERTING_CLOCK: c_int = 0;
pub const S5H1411_MPEGTIMING_CONTINUOUS_NONINVERTING_CLOCK: c_int = 1;
pub const S5H1411_MPEGTIMING_NONCONTINUOUS_INVERTING_CLOCK: c_int = 2;
pub const S5H1411_MPEGTIMING_NONCONTINUOUS_NONINVERTING_CLOCK: c_int = 3;
    pub mpeg_timing: u16,
// IF Freq for QAM and VSB in KHz
pub const S5H1411_IF_3250: c_int = 3250;
pub const S5H1411_IF_3500: c_int = 3500;
pub const S5H1411_IF_4000: c_int = 4000;
pub const S5H1411_IF_5380: c_int = 5380;
pub const S5H1411_IF_44000: c_int = 44000;

    pub qam_if: u16,
    pub vsb_if: u16,
// Spectral Inversion
pub const S5H1411_INVERSION_OFF: c_int = 0;
pub const S5H1411_INVERSION_ON: c_int = 1;
    pub inversion: u8,
// Return lock status based on tuner lock, or demod lock
pub const S5H1411_TUNERLOCKING: c_int = 0;
pub const S5H1411_DEMODLOCKING: c_int = 1;
    pub status_mode: u8,
}

