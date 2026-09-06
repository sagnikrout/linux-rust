//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/pxa/mfp.h
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
// Common Multi-Function Pin Definitions
//
// Copyright (C) 2007 Marvell International Ltd.
//
// 2007-8-21: eric miao <eric.miao@marvell.com>
// initial version
//

// list of all the configurable MFP pins
// additional pins on PXA930
// additional pins on MMP2
//
// a possible MFP configuration is represented by a 32-bit integer
//
// bit  0.. 9 - MFP Pin Number (1024 Pins Maximum)
// bit 10..12 - Alternate Function Selection
// bit 13..15 - Drive Strength
// bit 16..18 - Low Power Mode State
// bit 19..20 - Low Power Mode Edge Detection
// bit 21..22 - Run Mode Pull State
//
// to facilitate the definition, the following macros are provided
//
// MFP_CFG_DEFAULT - default MFP configuration value, with
// alternate function = 0,
// drive strength = fast 3mA (MFP_DS03X)
// low power mode = default
// edge detection = none
//
// MFP_CFG	- default MFPR value with alternate function
// MFP_CFG_DRV	- default MFPR value with alternate function and
// pin drive strength
// MFP_CFG_LPM	- default MFPR value with alternate function and
// low power mode
// MFP_CFG_X	- default MFPR value with alternate function,
// pin drive strength and low power mode
//
pub type mfp_cfg_t = c_ulong;

//
// each MFP pin will have a MFPR register, since the offset of the
// register varies between processors, the processor specific code
// should initialize the pin offsets by mfp_init()
//
// mfp_init_base() - accepts a virtual base for all MFPR registers and
// initialize the MFP table to a default state
//
// mfp_init_addr() - accepts a table of "mfp_addr_map" structure, which
// represents a range of MFP pins from "start" to "end", with the offset
// beginning at "offset", to define a single pin, let "end" = -1.
//
// use
//
// MFP_ADDR_X() to define a range of pins
// MFP_ADDR()   to define a single pin
// MFP_ADDR_END to signal the end of pin offset definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfp_addr_map {
    pub start: c_uint,
    pub end: c_uint,
    pub offset: c_ulong,
}

extern "C" {
    pub fn mfp_init_base(mfpr_base: *mut void __iomem);
}
extern "C" {
    pub fn mfp_init_addr(map: *mut mfp_addr_map);
}
//
// mfp_{read, write}()	- for direct read/write access to the MFPR register
// mfp_config()		- for configuring a group of MFPR registers
// mfp_config_lpm()	- configuring all low power MFPR registers for suspend
// mfp_config_run()	- configuring all run time  MFPR registers after resume
//
extern "C" {
    pub fn mfp_read(mfp: c_int) -> c_ulong;
}
extern "C" {
    pub fn mfp_write(mfp: c_int, mfpr_val: c_ulong);
}
extern "C" {
    pub fn mfp_config(mfp_cfgs: *mut c_ulong, num: c_int);
}
extern "C" {
    pub fn mfp_config_run();
}
extern "C" {
    pub fn mfp_config_lpm();
}

