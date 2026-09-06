//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sprd/mux.h
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
// Spreadtrum multiplexer clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

//
// struct sprd_mux_ssel - Mux clock's source select bits in its register
// @shift: Bit offset of the divider in its register
// @width: Width of the divider field in its register
// @table: For some mux clocks, not all sources are used on some special
// chips, this matches the value of mux clock's register and the
// sources which are used for this mux clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_mux_ssel {
    pub shift: u8,
    pub width: u8,
    pub table: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_mux {
    pub mux: sprd_mux_ssel,
    pub common: sprd_clk_common,
}

extern "C" {
    pub fn container_of(_arg: common, sprd_mux: struct, _arg: common) -> return;
}
