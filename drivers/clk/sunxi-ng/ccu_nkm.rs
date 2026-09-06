//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu_nkm.h
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
// struct ccu_nkm - Definition of an N-K-M clock
//
// Clocks based on the formula parent * N * K / M
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_nkm {
    pub enable: u32,
    pub lock: u32,
    pub n: ccu_mult_internal,
    pub k: ccu_mult_internal,
    pub m: ccu_div_internal,
    pub mux: ccu_mux_internal,
    pub fixed_post_div: c_uint,
    pub max_m_n_ratio: c_ulong,
    pub min_parent_m_ratio: c_ulong,
    pub common: ccu_common,
}

extern "C" {
    pub fn container_of(_arg: common, ccu_nkm: struct, _arg: common) -> return;
}
