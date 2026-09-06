//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/zynqmp/clk-zynqmp.h
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
// Copyright (C) 2016-2018 Xilinx
//

// Common Flags
// must be gated across rate change

// must be gated across re-parent

// propagate rate change up one level

// do not gate even if unused

// don't re-parent on rate change

// do not gate, ever

// Type Flags for divider clock

// Type Flags for mux clock

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum topology_type {
    TYPE_INVALID,
    TYPE_MUX,
    TYPE_PLL,
    TYPE_FIXEDFACTOR,
    TYPE_DIV1,
    TYPE_DIV2,
    TYPE_GATE,
}

//
// struct clock_topology - Clock topology
// @type:	Type of topology
// @flag:	Topology flags
// @type_flag:	Topology type specific flag
// @custom_type_flag: Topology type specific custom flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_topology {
    pub type: u32,
    pub flag: u32,
    pub type_flag: u32,
    pub custom_type_flag: u8,
}

extern "C" {
    pub fn zynqmp_clk_map_common_ccf_flags(zynqmp_flag: u32) -> c_ulong;
}
