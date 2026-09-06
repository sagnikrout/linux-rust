//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-hfpll.h
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
pub struct hfpll_data {
    pub mode_reg: u32,
    pub l_reg: u32,
    pub m_reg: u32,
    pub n_reg: u32,
    pub user_reg: u32,
    pub droop_reg: u32,
    pub config_reg: u32,
    pub status_reg: u32,
    pub lock_bit: u8,
    pub l_val: u32,
    pub droop_val: u32,
    pub config_val: u32,
    pub user_val: u32,
    pub user_vco_mask: u32,
    pub low_vco_max_rate: c_ulong,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hfpll {
    pub d: *const hfpll_data,
    pub init_done: c_int,
    pub clkr: clk_regmap,
    pub lock: spinlock_t,
}

