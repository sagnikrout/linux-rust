//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/max2175.h
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
// Maxim Integrated MAX2175 RF to Bits tuner driver
//
// This driver & most of the hard coded values are based on the reference
// application delivered by Maxim for this device.
//
// Copyright (C) 2016 Maxim Integrated Products
// Copyright (C) 2017 Renesas Electronics Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max2175_region {
    MAX2175_REGION_EU = 0,	/* Europe */
    MAX2175_REGION_NA,	/* North America */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max2175_band {
    MAX2175_BAND_AM = 0,
    MAX2175_BAND_FM,
    MAX2175_BAND_VHF,
    MAX2175_BAND_L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max2175_eu_mode {
// EU modes
    MAX2175_EU_FM_1_2 = 0,
    MAX2175_DAB_1_2,

//
// Other possible modes to add in future
// MAX2175_DAB_1_0,
// MAX2175_DAB_1_3,
// MAX2175_EU_FM_2_2,
// MAX2175_EU_FMHD_4_0,
// MAX2175_EU_AM_1_0,
// MAX2175_EU_AM_2_2,
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max2175_na_mode {
// NA modes
    MAX2175_NA_FM_1_0 = 0,
    MAX2175_NA_FM_2_0,

//
// Other possible modes to add in future
// MAX2175_NA_FMHD_1_0,
// MAX2175_NA_FMHD_1_2,
// MAX2175_NA_AM_1_0,
// MAX2175_NA_AM_1_2,
//
}

// Supported I2S modes
// Coefficient table groups
// HSLS LO injection polarity
// Channel FSM modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max2175_csm_mode {
    MAX2175_LOAD_TO_BUFFER = 0,
    MAX2175_PRESET_TUNE,
    MAX2175_SEARCH,
    MAX2175_AF_UPDATE,
    MAX2175_JUMP_FAST_TUNE,
    MAX2175_CHECK,
    MAX2175_LOAD_AND_SWAP,
    MAX2175_END,
    MAX2175_BUFFER_PLUS_PRESET_TUNE,
    MAX2175_BUFFER_PLUS_SEARCH,
    MAX2175_BUFFER_PLUS_AF_UPDATE,
    MAX2175_BUFFER_PLUS_JUMP_FAST_TUNE,
    MAX2175_BUFFER_PLUS_CHECK,
    MAX2175_BUFFER_PLUS_LOAD_AND_SWAP,
    MAX2175_NO_ACTION
}
