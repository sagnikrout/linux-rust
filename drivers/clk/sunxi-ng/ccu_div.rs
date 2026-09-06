//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu_div.h
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
// Copyright (c) 2016 Maxime Ripard. All rights reserved.
//

//
// struct ccu_div_internal - Internal divider description
// @shift: Bit offset of the divider in its register
// @width: Width of the divider field in its register
// @max: Maximum value allowed for that divider. This is the
// arithmetic value, not the maximum value to be set in the
// register.
// @flags: clk_divider flags to apply on this divider
// @table: Divider table pointer (if applicable)
//
// That structure represents a single divider, and is meant to be
// embedded in other structures representing the various clock
// classes.
//
// It is basically a wrapper around the clk_divider functions
// arguments.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_div_internal {
    pub shift: u8,
    pub width: u8,
    pub max: u32,
    pub offset: u32,
    pub flags: u32,
    pub table: *mut clk_div_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_div {
    pub enable: u32,
    pub div: ccu_div_internal,
    pub mux: ccu_mux_internal,
    pub common: ccu_common,
    pub fixed_post_div: c_uint,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_div: struct, _arg: common) -> return;
}
