//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/freescale/pinctrl-mxs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2012 Freescale Semiconductor, Inc.
//

pub const SET: c_uint = 0x4;
pub const CLR: c_uint = 0x8;
pub const TOG: c_uint = 0xc;

//
// pinmux-id bit field definitions
//
// bank:	15..12	(4)
// pin:		11..4	(8)
// muxsel:	3..0	(4)
//

//
// pin config bit field definitions
//
// pull-up:	6..5	(2)
// voltage:	4..3	(2)
// mA:		2..0	(3)
//
// MSB of each field is presence bit for the config.
//

pub const PULL_SHIFT: c_int = 5;

pub const VOL_SHIFT: c_int = 3;

pub const MA_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_function {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub ngroups: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_group {
    pub name: *const c_char,
    pub pins: *mut c_uint,
    pub npins: unsigned,
    pub muxsel: *mut u8,
    pub config: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_regs {
    pub muxsel: u16,
    pub drive: u16,
    pub pull: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_pinctrl_soc_data {
    pub regs: *const mxs_regs,
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub functions: *mut mxs_function,
    pub nfunctions: unsigned,
    pub groups: *mut mxs_group,
    pub ngroups: unsigned,
}
