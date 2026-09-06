//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/analogbits-wrpll-cln28hpc.h
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
// Copyright (C) 2018-2019 SiFive, Inc.
// Wesley Terpstra
// Paul Walmsley
//

// DIVQ_VALUES: number of valid DIVQ values
pub const DIVQ_VALUES: c_int = 6;
//
// Bit definitions for struct wrpll_cfg.flags
//
// WRPLL_FLAGS_BYPASS_FLAG: if set, the PLL is either in bypass, or should be
// programmed to enter bypass
// WRPLL_FLAGS_RESET_FLAG: if set, the PLL is in reset
// WRPLL_FLAGS_INT_FEEDBACK_FLAG: if set, the PLL is configured for internal
// feedback mode
// WRPLL_FLAGS_EXT_FEEDBACK_FLAG: if set, the PLL is configured for external
// feedback mode (not yet supported by this driver)
//
pub const WRPLL_FLAGS_BYPASS_SHIFT: c_int = 0;

pub const WRPLL_FLAGS_RESET_SHIFT: c_int = 1;

pub const WRPLL_FLAGS_INT_FEEDBACK_SHIFT: c_int = 2;

pub const WRPLL_FLAGS_EXT_FEEDBACK_SHIFT: c_int = 3;

//
// struct wrpll_cfg - WRPLL configuration values
// @divr: reference divider value (6 bits), as presented to the PLL signals
// @divf: feedback divider value (9 bits), as presented to the PLL signals
// @divq: output divider value (3 bits), as presented to the PLL signals
// @flags: PLL configuration flags.  See above for more information
// @range: PLL loop filter range.  See below for more information
// @output_rate_cache: cached output rates, swept across DIVQ
// @parent_rate: PLL refclk rate for which values are valid
// @max_r: maximum possible R divider value, given @parent_rate
// @init_r: initial R divider value to start the search from
//
// @divr, @divq, @divq, @range represent what the PLL expects to see
// on its input signals.  Thus @divr and @divf are the actual divisors
// minus one.  @divq is a power-of-two divider; for example, 1 =
// divide-by-2 and 6 = divide-by-64.  0 is an invalid @divq value.
//
// When initially passing a struct wrpll_cfg record, the
// record should be zero-initialized with the exception of the @flags
// field.  The only flag bits that need to be set are either
// WRPLL_FLAGS_INT_FEEDBACK or WRPLL_FLAGS_EXT_FEEDBACK.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wrpll_cfg {
    pub divr: u8,
    pub divq: u8,
    pub range: u8,
    pub flags: u8,
    pub divf: u16,
// private:
    pub output_rate_cache: [u32; DIVQ_VALUES],
    pub parent_rate: c_ulong,
    pub max_r: u8,
    pub init_r: u8,
}

extern "C" {
    pub fn wrpll_calc_max_lock_us(c: *const wrpll_cfg) -> c_uint;
}
