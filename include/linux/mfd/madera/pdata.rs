//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/madera/pdata.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Platform data for Cirrus Logic Madera codecs
//
// Copyright (C) 2015-2018 Cirrus Logic
//

pub const MADERA_MAX_MICBIAS: c_int = 4;
pub const MADERA_MAX_CHILD_MICBIAS: c_int = 4;
pub const MADERA_MAX_GPSW: c_int = 2;
//
// struct madera_pdata - Configuration data for Madera devices
//
// @reset:	    GPIO controlling /RESET (NULL = none)
// @ldo1:	    Substruct of pdata for the LDO1 regulator
// @micvdd:	    Substruct of pdata for the MICVDD regulator
// @irq_flags:	    Mode for primary IRQ (defaults to active low)
// @gpio_base:	    Base GPIO number
// @gpio_configs:   Array of GPIO configurations (See
// Documentation/driver-api/pin-control.rst)
// @n_gpio_configs: Number of entries in gpio_configs
// @gpsw:	    General purpose switch mode setting. Depends on the external
// hardware connected to the switch. (See the SW1_MODE field
// in the datasheet for the available values for your codec)
// @codec:	    Substruct of pdata for the ASoC codec driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_pdata {
    pub reset: *mut gpio_desc,
    pub ldo1: arizona_ldo1_pdata,
    pub micvdd: arizona_micsupp_pdata,
    pub irq_flags: c_uint,
    pub gpio_base: c_int,
    pub gpio_configs: *const pinctrl_map,
    pub n_gpio_configs: c_int,
    pub gpsw: [u32; MADERA_MAX_GPSW],
    pub codec: madera_codec_pdata,
}
