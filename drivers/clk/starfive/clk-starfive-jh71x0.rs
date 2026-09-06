//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/starfive/clk-starfive-jh71x0.h
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

// register fields

pub const JH71X0_CLK_MUX_SHIFT: c_int = 24;

pub const JH71X0_CLK_FRAC_SHIFT: c_int = 8;

// fractional divider min/max

// clock data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71x0_clk_data {
    pub name: *const c_char,
    pub flags: c_ulong,
    pub max: u32,
    pub parents: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71x0_clk {
    pub hw: clk_hw,
    pub idx: c_uint,
    pub max_div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71x0_clk_priv {
// protect clk enable and set rate/parent from happening at the same time
    pub rmw_lock: spinlock_t,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub original_clk: *mut clk,
    pub pll_clk_nb: notifier_block,
    pub pll: [*mut clk_hw; 3],
    pub num_reg: c_uint,
    pub __counted_by(num_reg): jh71x0_clk reg[],
}
