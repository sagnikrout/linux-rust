//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8192-mmsys.h
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


// SPDX-License-Identifier: GPL-2.0-only
pub const MT8192_MMSYS_OVL_MOUT_EN: c_uint = 0xf04;
pub const MT8192_DISP_OVL1_2L_MOUT_EN: c_uint = 0xf08;
pub const MT8192_DISP_OVL0_2L_MOUT_EN: c_uint = 0xf18;
pub const MT8192_DISP_OVL0_MOUT_EN: c_uint = 0xf1c;
pub const MT8192_DISP_RDMA0_SEL_IN: c_uint = 0xf2c;
pub const MT8192_DISP_RDMA0_SOUT_SEL: c_uint = 0xf30;
pub const MT8192_DISP_CCORR0_SOUT_SEL: c_uint = 0xf34;
pub const MT8192_DISP_AAL0_SEL_IN: c_uint = 0xf38;
pub const MT8192_DISP_DITHER0_MOUT_EN: c_uint = 0xf3c;
pub const MT8192_DISP_DSI0_SEL_IN: c_uint = 0xf40;
pub const MT8192_DISP_OVL2_2L_MOUT_EN: c_uint = 0xf4c;

pub const MT8192_RDMA0_SEL_IN_OVL0_2L: c_uint = 0x3;
pub const MT8192_RDMA0_SOUT_COLOR0: c_uint = 0x1;
pub const MT8192_CCORR0_SOUT_AAL0: c_uint = 0x1;
pub const MT8192_AAL0_SEL_IN_CCORR0: c_uint = 0x1;
pub const MT8192_DSI0_SEL_IN_DITHER0: c_uint = 0x1;
