//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320aic26.h
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
// Texas Instruments TLV320AIC26 low power audio CODEC
// register definitions
//
// Copyright (C) 2008 Secret Lab Technologies Ltd.
//
// AIC26 Registers

// Page 0: Auxiliary data registers

// Page 1: Auxiliary control registers

// Page 2: Audio control registers

// fsref dividers; used in register 'Audio Control 1'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic26_divisors {
    AIC26_DIV_1	= 0,
    AIC26_DIV_1_5	= 1,
    AIC26_DIV_2	= 2,
    AIC26_DIV_3	= 3,
    AIC26_DIV_4	= 4,
    AIC26_DIV_5	= 5,
    AIC26_DIV_5_5	= 6,
    AIC26_DIV_6	= 7,
}

// Digital data format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic26_datfm {
    AIC26_DATFM_I2S		= 0 << 8,
    AIC26_DATFM_DSP		= 1 << 8,
    AIC26_DATFM_RIGHTJ	= 2 << 8, /* right justified */
    AIC26_DATFM_LEFTJ	= 3 << 8, /* left justified */
}

// Sample word length in bits; used in register 'Audio Control 1'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aic26_wlen {
    AIC26_WLEN_16	= 0 << 10,
    AIC26_WLEN_20	= 1 << 10,
    AIC26_WLEN_24	= 2 << 10,
    AIC26_WLEN_32	= 3 << 10,
}
