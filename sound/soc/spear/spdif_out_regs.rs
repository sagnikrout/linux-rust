//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/spear/spdif_out_regs.h
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
// SPEAr SPDIF OUT controller header file
//
// Copyright (ST) 2011 Vipin Kumar (vipin.kumar@st.com)
//
pub const SPDIF_OUT_SOFT_RST: c_uint = 0x00;

pub const SPDIF_OUT_FIFO_DATA: c_uint = 0x04;
pub const SPDIF_OUT_INT_STA: c_uint = 0x08;
pub const SPDIF_OUT_INT_STA_CLR: c_uint = 0x0C;

pub const SPDIF_OUT_INT_EN: c_uint = 0x10;
pub const SPDIF_OUT_INT_EN_SET: c_uint = 0x14;
pub const SPDIF_OUT_INT_EN_CLR: c_uint = 0x18;
pub const SPDIF_OUT_CTRL: c_uint = 0x1C;

pub const SPDIF_OUT_STA: c_uint = 0x20;
pub const SPDIF_OUT_PA_PB: c_uint = 0x24;
pub const SPDIF_OUT_PC_PD: c_uint = 0x28;
pub const SPDIF_OUT_CL1: c_uint = 0x2C;
pub const SPDIF_OUT_CR1: c_uint = 0x30;
pub const SPDIF_OUT_CL2_CR2_UV: c_uint = 0x34;
pub const SPDIF_OUT_PAUSE_LAT: c_uint = 0x38;
pub const SPDIF_OUT_FRMLEN_BRST: c_uint = 0x3C;
pub const SPDIF_OUT_CFG: c_uint = 0x40;

