//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sprd/gate.h
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
// Spreadtrum gate clock driver
//
// Copyright (C) 2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_gate {
    pub enable_mask: u32,
    pub flags: u16,
    pub sc_offset: u16,
    pub udelay: u16,
    pub common: sprd_clk_common,
}

//
// sprd_gate->flags is used for:
// CLK_GATE_SET_TO_DISABLE	BIT(0)
// CLK_GATE_HIWORD_MASK		BIT(1)
// CLK_GATE_BIG_ENDIAN		BIT(2)
// so we define new flags from	BIT(3)
//

extern "C" {
    pub fn container_of(_arg: common, sprd_gate: struct, _arg: common) -> return;
}
