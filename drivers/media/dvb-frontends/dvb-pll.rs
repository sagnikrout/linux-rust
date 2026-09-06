//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dvb-pll.h
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
// descriptions + helper functions for simple dvb plls.
//

pub const DVB_PLL_UNDEFINED: c_int = 0;
pub const DVB_PLL_THOMSON_DTT7579: c_int = 1;
pub const DVB_PLL_THOMSON_DTT759X: c_int = 2;
pub const DVB_PLL_LG_Z201: c_int = 3;
pub const DVB_PLL_UNKNOWN_1: c_int = 4;
pub const DVB_PLL_TUA6010XS: c_int = 5;
pub const DVB_PLL_ENV57H1XD5: c_int = 6;
pub const DVB_PLL_TUA6034: c_int = 7;
pub const DVB_PLL_TDA665X: c_int = 8;
pub const DVB_PLL_TDED4: c_int = 9;
pub const DVB_PLL_TDHU2: c_int = 10;
pub const DVB_PLL_SAMSUNG_TBMV: c_int = 11;
pub const DVB_PLL_PHILIPS_SD1878_TDA8261: c_int = 12;
pub const DVB_PLL_OPERA1: c_int = 13;
pub const DVB_PLL_SAMSUNG_DTOS403IH102A: c_int = 14;
pub const DVB_PLL_SAMSUNG_TDTC9251DH0: c_int = 15;
pub const DVB_PLL_SAMSUNG_TBDU18132: c_int = 16;
pub const DVB_PLL_SAMSUNG_TBMU24112: c_int = 17;
pub const DVB_PLL_TDEE4: c_int = 18;
pub const DVB_PLL_THOMSON_DTT7520X: c_int = 19;
pub const DVB_PLL_TUA6034_FRIIO: c_int = 20;
pub const DVB_PLL_TDA665X_EARTH_PT1: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_pll_config {
    pub fe: *mut dvb_frontend,
}

//
// dvb_pll_attach - Attach a dvb-pll to the supplied frontend structure.
//
// @fe: Frontend to attach to.
// @pll_addr: i2c address of the PLL (if used).
// @i2c: i2c adapter to use (set to NULL if not used).
// @pll_desc_id: dvb_pll_desc to use.
//
// return: Frontend pointer on success, NULL on failure
//

