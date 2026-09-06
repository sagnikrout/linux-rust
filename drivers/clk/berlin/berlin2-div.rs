//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/berlin/berlin2-div.h
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
// Copyright (c) 2014 Marvell Technology Group Ltd.
//
// Alexandre Belloni <alexandre.belloni@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin2_div_map {
    pub pll_select_offs: u16,
    pub pll_switch_offs: u16,
    pub div_select_offs: u16,
    pub div_switch_offs: u16,
    pub div3_switch_offs: u16,
    pub gate_offs: u16,
    pub pll_select_shift: u8,
    pub pll_switch_shift: u8,
    pub div_select_shift: u8,
    pub div_switch_shift: u8,
    pub div3_switch_shift: u8,
    pub gate_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct berlin2_div_data {
    pub name: *const c_char,
    pub parent_ids: *const u8,
    pub num_parents: c_int,
    pub flags: c_ulong,
    pub map: berlin2_div_map,
    pub div_flags: u8,
}
