//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro_g1_regs.h
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
// Hantro VPU codec driver
//
// Copyright 2018 Google LLC.
// Tomasz Figa <tfiga@chromium.org>
//

// Decoder registers.
pub const G1_REG_INTERRUPT: c_uint = 0x004;

pub const G1_REG_CONFIG: c_uint = 0x008;

pub const G1_REG_DEC_CTRL0: c_uint = 0x00c;

// Setting AXI ID to 0xff to get auto generated ID to avoid possible conflicts

pub const G1_REG_DEC_CTRL1: c_uint = 0x010;

pub const G1_REG_DEC_CTRL2: c_uint = 0x014;

pub const G1_REG_DEC_CTRL3: c_uint = 0x018;

pub const G1_REG_DEC_CTRL4: c_uint = 0x01c;

pub const G1_REG_DEC_CTRL5: c_uint = 0x020;

pub const G1_REG_DEC_CTRL6: c_uint = 0x024;

pub const G1_REG_DEC_CTRL7: c_uint = 0x02c;

pub const G1_REG_ADDR_STR: c_uint = 0x030;
pub const G1_REG_ADDR_DST: c_uint = 0x034;

pub const G1_REG_LT_REF: c_uint = 0x098;
pub const G1_REG_VALID_REF: c_uint = 0x09c;
pub const G1_REG_ADDR_QTABLE: c_uint = 0x0a0;
pub const G1_REG_ADDR_DIR_MV: c_uint = 0x0a4;

pub const G1_REG_BD_P_REF_PIC: c_uint = 0x0bc;

pub const G1_REG_ERR_CONC: c_uint = 0x0c0;

pub const G1_REG_PRED_FLT: c_uint = 0x0c4;

pub const G1_REG_REF_BUF_CTRL: c_uint = 0x0cc;

pub const G1_REG_REF_BUF_CTRL2: c_uint = 0x0dc;

pub const G1_REG_SOFT_RESET: c_uint = 0x194;
// Post-processor registers.

