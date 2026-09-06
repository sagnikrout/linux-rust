//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv6110x_reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

// Macro flag: #define __STV6110x_REG_H
pub const STV6110x_CTRL1: c_uint = 0x00;
pub const STV6110x_OFFST_CTRL1_K: c_int = 3;
pub const STV6110x_WIDTH_CTRL1_K: c_int = 5;
pub const STV6110x_OFFST_CTRL1_LPT: c_int = 2;
pub const STV6110x_WIDTH_CTRL1_LPT: c_int = 1;
pub const STV6110x_OFFST_CTRL1_RX: c_int = 1;
pub const STV6110x_WIDTH_CTRL1_RX: c_int = 1;
pub const STV6110x_OFFST_CTRL1_SYN: c_int = 0;
pub const STV6110x_WIDTH_CTRL1_SYN: c_int = 1;
pub const STV6110x_CTRL2: c_uint = 0x01;
pub const STV6110x_OFFST_CTRL2_CO_DIV: c_int = 6;
pub const STV6110x_WIDTH_CTRL2_CO_DIV: c_int = 2;
pub const STV6110x_OFFST_CTRL2_RSVD: c_int = 5;
pub const STV6110x_WIDTH_CTRL2_RSVD: c_int = 1;
pub const STV6110x_OFFST_CTRL2_REFOUT_SEL: c_int = 4;
pub const STV6110x_WIDTH_CTRL2_REFOUT_SEL: c_int = 1;
pub const STV6110x_OFFST_CTRL2_BBGAIN: c_int = 0;
pub const STV6110x_WIDTH_CTRL2_BBGAIN: c_int = 4;
pub const STV6110x_TNG0: c_uint = 0x02;
pub const STV6110x_OFFST_TNG0_N_DIV_7_0: c_int = 0;
pub const STV6110x_WIDTH_TNG0_N_DIV_7_0: c_int = 8;
pub const STV6110x_TNG1: c_uint = 0x03;
pub const STV6110x_OFFST_TNG1_R_DIV: c_int = 6;
pub const STV6110x_WIDTH_TNG1_R_DIV: c_int = 2;
pub const STV6110x_OFFST_TNG1_PRESC32_ON: c_int = 5;
pub const STV6110x_WIDTH_TNG1_PRESC32_ON: c_int = 1;
pub const STV6110x_OFFST_TNG1_DIV4SEL: c_int = 4;
pub const STV6110x_WIDTH_TNG1_DIV4SEL: c_int = 1;
pub const STV6110x_OFFST_TNG1_N_DIV_11_8: c_int = 0;
pub const STV6110x_WIDTH_TNG1_N_DIV_11_8: c_int = 4;
pub const STV6110x_CTRL3: c_uint = 0x04;
pub const STV6110x_OFFST_CTRL3_DCLOOP_OFF: c_int = 7;
pub const STV6110x_WIDTH_CTRL3_DCLOOP_OFF: c_int = 1;
pub const STV6110x_OFFST_CTRL3_RCCLK_OFF: c_int = 6;
pub const STV6110x_WIDTH_CTRL3_RCCLK_OFF: c_int = 1;
pub const STV6110x_OFFST_CTRL3_ICP: c_int = 5;
pub const STV6110x_WIDTH_CTRL3_ICP: c_int = 1;
pub const STV6110x_OFFST_CTRL3_CF: c_int = 0;
pub const STV6110x_WIDTH_CTRL3_CF: c_int = 5;
pub const STV6110x_STAT1: c_uint = 0x05;
pub const STV6110x_OFFST_STAT1_CALVCO_STRT: c_int = 2;
pub const STV6110x_WIDTH_STAT1_CALVCO_STRT: c_int = 1;
pub const STV6110x_OFFST_STAT1_CALRC_STRT: c_int = 1;
pub const STV6110x_WIDTH_STAT1_CALRC_STRT: c_int = 1;
pub const STV6110x_OFFST_STAT1_LOCK: c_int = 0;
pub const STV6110x_WIDTH_STAT1_LOCK: c_int = 1;
pub const STV6110x_STAT2: c_uint = 0x06;
pub const STV6110x_STAT3: c_uint = 0x07;
