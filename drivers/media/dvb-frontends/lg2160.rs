//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lg2160.h
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
// Support for LG2160 - ATSC/MH
//
// Copyright (C) 2010 Michael Krufky <mkrufky@linuxtv.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lg_chip_type {
    LG2160 = 0,
    LG2161 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lg2160_spi_clock {
    LG2160_SPI_3_125_MHZ = 0,
    LG2160_SPI_6_25_MHZ = 1,
    LG2160_SPI_12_5_MHZ = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lg2161_oif {
    LG2161_OIF_EBI2_SLA  = 1,
    LG2161_OIF_SDIO_SLA  = 2,
    LG2161_OIF_SPI_SLA   = 3,
    LG2161_OIF_SPI_MAS   = 4,
    LG2161_OIF_SERIAL_TS = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lg2160_config {
    pub i2c_addr: u8,
// user defined IF frequency in KHz
    pub if_khz: u16,
// disable i2c repeater - 0:repeater enabled 1:repeater disabled
    pub deny_i2c_rptr:1: c_uint,
// spectral inversion - 0:disabled 1:enabled
    pub spectral_inversion:1: c_uint,
    pub output_if: c_uint,
    pub spi_clock: lg2160_spi_clock,
    pub lg_chip: lg_chip_type,
}

