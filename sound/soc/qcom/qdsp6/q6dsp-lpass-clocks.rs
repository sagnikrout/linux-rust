//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6dsp-lpass-clocks.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6dsp_clk_init {
    pub clk_id: c_int,
    pub q6dsp_clk_id: c_int,
    pub name: *mut c_char,
    pub rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6dsp_clk_desc {
    pub clks: *const q6dsp_clk_init,
    pub num_clks: usize,
    pub freq): int root_clk, unsigned int,
    pub h): *const *const *const *const int (lpass_vote_clk)(struct device dev, uint32_t hid, char n, uint32_t,
    pub h): *mut *mut *mut int (lpass_unvote_clk)(struct device dev, uint32_t hid, uint32_t,
}

extern "C" {
    pub fn q6dsp_clock_dev_probe(pdev: *mut platform_device) -> c_int;
}
