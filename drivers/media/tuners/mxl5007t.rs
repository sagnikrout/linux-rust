//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mxl5007t.h
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
// mxl5007t.h - driver for the MaxLinear MxL5007T silicon tuner
//
// Copyright (C) 2008 Michael Krufky <mkrufky@linuxtv.org>
//

// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl5007t_if_freq {
    MxL_IF_4_MHZ,      /*  4000000 */
    MxL_IF_4_5_MHZ,    /*  4500000 */
    MxL_IF_4_57_MHZ,   /*  4570000 */
    MxL_IF_5_MHZ,      /*  5000000 */
    MxL_IF_5_38_MHZ,   /*  5380000 */
    MxL_IF_6_MHZ,      /*  6000000 */
    MxL_IF_6_28_MHZ,   /*  6280000 */
    MxL_IF_9_1915_MHZ, /*  9191500 */
    MxL_IF_35_25_MHZ,  /* 35250000 */
    MxL_IF_36_15_MHZ,  /* 36150000 */
    MxL_IF_44_MHZ,     /* 44000000 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl5007t_xtal_freq {
    MxL_XTAL_16_MHZ,      /* 16000000 */
    MxL_XTAL_20_MHZ,      /* 20000000 */
    MxL_XTAL_20_25_MHZ,   /* 20250000 */
    MxL_XTAL_20_48_MHZ,   /* 20480000 */
    MxL_XTAL_24_MHZ,      /* 24000000 */
    MxL_XTAL_25_MHZ,      /* 25000000 */
    MxL_XTAL_25_14_MHZ,   /* 25140000 */
    MxL_XTAL_27_MHZ,      /* 27000000 */
    MxL_XTAL_28_8_MHZ,    /* 28800000 */
    MxL_XTAL_32_MHZ,      /* 32000000 */
    MxL_XTAL_40_MHZ,      /* 40000000 */
    MxL_XTAL_44_MHZ,      /* 44000000 */
    MxL_XTAL_48_MHZ,      /* 48000000 */
    MxL_XTAL_49_3811_MHZ, /* 49381100 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl5007t_clkout_amp {
    MxL_CLKOUT_AMP_0_94V = 0,
    MxL_CLKOUT_AMP_0_53V = 1,
    MxL_CLKOUT_AMP_0_37V = 2,
    MxL_CLKOUT_AMP_0_28V = 3,
    MxL_CLKOUT_AMP_0_23V = 4,
    MxL_CLKOUT_AMP_0_20V = 5,
    MxL_CLKOUT_AMP_0_17V = 6,
    MxL_CLKOUT_AMP_0_15V = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl5007t_config {
    pub if_diff_out_level: i32,
    pub clk_out_amp: mxl5007t_clkout_amp,
    pub xtal_freq_hz: mxl5007t_xtal_freq,
    pub if_freq_hz: mxl5007t_if_freq,
    pub invert_if:1: c_uint,
    pub loop_thru_enable:1: c_uint,
    pub clk_out_enable:1: c_uint,
}

