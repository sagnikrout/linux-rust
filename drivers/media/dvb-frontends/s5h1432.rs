//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/s5h1432.h
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
// Samsung s5h1432 VSB/QAM demodulator driver
//
// Copyright (C) 2009 Bill Liu <Bill.Liu@Conexant.com>
//

pub const TAIWAN_HI_IF_FREQ_44_MHZ: c_int = 44000000;
pub const EUROPE_HI_IF_FREQ_36_MHZ: c_int = 36000000;
pub const IF_FREQ_6_MHZ: c_int = 6000000;
pub const IF_FREQ_3point3_MHZ: c_int = 3300000;
pub const IF_FREQ_3point5_MHZ: c_int = 3500000;
pub const IF_FREQ_4_MHZ: c_int = 4000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5h1432_config {
// serial/parallel output
pub const S5H1432_PARALLEL_OUTPUT: c_int = 0;
pub const S5H1432_SERIAL_OUTPUT: c_int = 1;
    pub output_mode: u8,
// GPIO Setting
pub const S5H1432_GPIO_OFF: c_int = 0;
pub const S5H1432_GPIO_ON: c_int = 1;
    pub gpio: u8,
// MPEG signal timing
pub const S5H1432_MPEGTIMING_CONTINUOUS_INVERTING_CLOCK: c_int = 0;
pub const S5H1432_MPEGTIMING_CONTINUOUS_NONINVERTING_CLOCK: c_int = 1;
pub const S5H1432_MPEGTIMING_NONCONTINUOUS_INVERTING_CLOCK: c_int = 2;
pub const S5H1432_MPEGTIMING_NONCONTINUOUS_NONINVERTING_CLOCK: c_int = 3;
    pub mpeg_timing: u16,
// IF Freq for QAM and VSB in KHz
pub const S5H1432_IF_3250: c_int = 3250;
pub const S5H1432_IF_3500: c_int = 3500;
pub const S5H1432_IF_4000: c_int = 4000;
pub const S5H1432_IF_5380: c_int = 5380;
pub const S5H1432_IF_44000: c_int = 44000;

    pub qam_if: u16,
    pub vsb_if: u16,
// Spectral Inversion
pub const S5H1432_INVERSION_OFF: c_int = 0;
pub const S5H1432_INVERSION_ON: c_int = 1;
    pub inversion: u8,
// Return lock status based on tuner lock, or demod lock
pub const S5H1432_TUNERLOCKING: c_int = 0;
pub const S5H1432_DEMODLOCKING: c_int = 1;
    pub status_mode: u8,
}

// config,

