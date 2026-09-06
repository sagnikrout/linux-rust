//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/ms5611.h
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


// SPDX-License-Identifier: GPL-2.0
//
// MS5611 pressure and temperature sensor driver
//
// Copyright (c) Tomasz Duszynski <tduszyns@gmail.com>
//

pub const MS5611_RESET: c_uint = 0x1e;
pub const MS5611_READ_ADC: c_uint = 0x00;
pub const MS5611_READ_PROM_WORD: c_uint = 0xA0;
pub const MS5611_PROM_WORDS_NB: c_int = 8;
//
// OverSampling Rate descriptor.
// Warning: cmd MUST be kept aligned on a word boundary (see
// m5611_spi_read_adc_temp_and_pressure in ms5611_spi.c).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms5611_osr {
    pub conv_usec: c_ulong,
    pub cmd: u8,
    pub rate: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms5611_state {
    pub client: *mut c_void,
    pub lock: mutex,
    pub pressure_osr: *const ms5611_osr,
    pub temp_osr: *const ms5611_osr,
    pub prom: [u16; MS5611_PROM_WORDS_NB],
    pub st): *mut *mut int (reset)(struct ms5611_state,
    pub word): *mut *mut *mut int (read_prom_word)(struct ms5611_state st, int index, u16,
    pub pressure): *mut *mut s32 temp, s32,
    pub pressure): *mut i32,
}
