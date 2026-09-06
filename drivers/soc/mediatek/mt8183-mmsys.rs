//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8183-mmsys.h
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
pub const MT8183_DISP_OVL0_MOUT_EN: c_uint = 0xf00;
pub const MT8183_DISP_OVL0_2L_MOUT_EN: c_uint = 0xf04;
pub const MT8183_DISP_OVL1_2L_MOUT_EN: c_uint = 0xf08;
pub const MT8183_DISP_DITHER0_MOUT_EN: c_uint = 0xf0c;
pub const MT8183_DISP_PATH0_SEL_IN: c_uint = 0xf24;
pub const MT8183_DISP_DSI0_SEL_IN: c_uint = 0xf2c;
pub const MT8183_DISP_DPI0_SEL_IN: c_uint = 0xf30;
pub const MT8183_DISP_RDMA0_SOUT_SEL_IN: c_uint = 0xf50;
pub const MT8183_DISP_RDMA1_SOUT_SEL_IN: c_uint = 0xf54;

pub const MT8183_DISP_PATH0_SEL_IN_OVL0_2L: c_uint = 0x1;
pub const MT8183_DSI0_SEL_IN_RDMA0: c_uint = 0x1;
pub const MT8183_DSI0_SEL_IN_RDMA1: c_uint = 0x3;
pub const MT8183_DPI0_SEL_IN_RDMA0: c_uint = 0x1;
pub const MT8183_DPI0_SEL_IN_RDMA1: c_uint = 0x2;
pub const MT8183_RDMA0_SOUT_COLOR0: c_uint = 0x1;
pub const MT8183_RDMA1_SOUT_DSI0: c_uint = 0x1;
pub const MT8183_MMSYS_SW0_RST_B: c_uint = 0x140;
