//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/atmel/atmel-classd.h
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
pub const CLASSD_CR: c_uint = 0x00000000;
pub const CLASSD_CR_RESET: c_uint = 0x1;
pub const CLASSD_MR: c_uint = 0x00000004;
pub const CLASSD_MR_LEN_DIS: c_uint = 0x0;
pub const CLASSD_MR_LEN_EN: c_uint = 0x1;

pub const CLASSD_MR_LMUTE_DIS: c_uint = 0x0;
pub const CLASSD_MR_LMUTE_EN: c_uint = 0x1;

pub const CLASSD_MR_REN_DIS: c_uint = 0x0;
pub const CLASSD_MR_REN_EN: c_uint = 0x1;

pub const CLASSD_MR_RMUTE_DIS: c_uint = 0x0;
pub const CLASSD_MR_RMUTE_EN: c_uint = 0x1;

pub const CLASSD_MR_PWMTYP_SINGLE: c_uint = 0x0;
pub const CLASSD_MR_PWMTYP_DIFF: c_uint = 0x1;

pub const CLASSD_MR_NON_OVERLAP_DIS: c_uint = 0x0;
pub const CLASSD_MR_NON_OVERLAP_EN: c_uint = 0x1;

pub const CLASSD_MR_NOVR_VAL_5NS: c_uint = 0x0;
pub const CLASSD_MR_NOVR_VAL_10NS: c_uint = 0x1;
pub const CLASSD_MR_NOVR_VAL_15NS: c_uint = 0x2;
pub const CLASSD_MR_NOVR_VAL_20NS: c_uint = 0x3;

pub const CLASSD_INTPMR: c_uint = 0x00000008;

pub const CLASSD_INTPMR_DSP_CLK_FREQ_12M288: c_uint = 0x0;
pub const CLASSD_INTPMR_DSP_CLK_FREQ_11M2896: c_uint = 0x1;

pub const CLASSD_INTPMR_DEEMP_DIS: c_uint = 0x0;
pub const CLASSD_INTPMR_DEEMP_EN: c_uint = 0x1;

pub const CLASSD_INTPMR_SWAP_LEFT_ON_LSB: c_uint = 0x0;
pub const CLASSD_INTPMR_SWAP_RIGHT_ON_LSB: c_uint = 0x1;

pub const CLASSD_INTPMR_FRAME_8K: c_uint = 0x0;
pub const CLASSD_INTPMR_FRAME_16K: c_uint = 0x1;
pub const CLASSD_INTPMR_FRAME_32K: c_uint = 0x2;
pub const CLASSD_INTPMR_FRAME_48K: c_uint = 0x3;
pub const CLASSD_INTPMR_FRAME_96K: c_uint = 0x4;
pub const CLASSD_INTPMR_FRAME_22K: c_uint = 0x5;
pub const CLASSD_INTPMR_FRAME_44K: c_uint = 0x6;
pub const CLASSD_INTPMR_FRAME_88K: c_uint = 0x7;

pub const CLASSD_INTPMR_EQCFG_FLAT: c_uint = 0x0;
pub const CLASSD_INTPMR_EQCFG_B_BOOST_12: c_uint = 0x1;
pub const CLASSD_INTPMR_EQCFG_B_BOOST_6: c_uint = 0x2;
pub const CLASSD_INTPMR_EQCFG_B_CUT_12: c_uint = 0x3;
pub const CLASSD_INTPMR_EQCFG_B_CUT_6: c_uint = 0x4;
pub const CLASSD_INTPMR_EQCFG_M_BOOST_3: c_uint = 0x5;
pub const CLASSD_INTPMR_EQCFG_M_BOOST_8: c_uint = 0x6;
pub const CLASSD_INTPMR_EQCFG_M_CUT_3: c_uint = 0x7;
pub const CLASSD_INTPMR_EQCFG_M_CUT_8: c_uint = 0x8;
pub const CLASSD_INTPMR_EQCFG_T_BOOST_12: c_uint = 0x9;
pub const CLASSD_INTPMR_EQCFG_T_BOOST_6: c_uint = 0xa;
pub const CLASSD_INTPMR_EQCFG_T_CUT_12: c_uint = 0xb;
pub const CLASSD_INTPMR_EQCFG_T_CUT_6: c_uint = 0xc;

pub const CLASSD_INTPMR_MONO_DIS: c_uint = 0x0;
pub const CLASSD_INTPMR_MONO_EN: c_uint = 0x1;

pub const CLASSD_INTPMR_MONO_MODE_MIX: c_uint = 0x0;
pub const CLASSD_INTPMR_MONO_MODE_SAT: c_uint = 0x1;
pub const CLASSD_INTPMR_MONO_MODE_LEFT: c_uint = 0x2;
pub const CLASSD_INTPMR_MONO_MODE_RIGHT: c_uint = 0x3;

pub const CLASSD_INTSR: c_uint = 0x0000000c;
pub const CLASSD_THR: c_uint = 0x00000010;
pub const CLASSD_IER: c_uint = 0x00000014;
pub const CLASSD_IDR: c_uint = 0x00000018;
pub const CLASSD_IMR: c_uint = 0x0000001c;
pub const CLASSD_ISR: c_uint = 0x00000020;
pub const CLASSD_WPMR: c_uint = 0x000000e4;
