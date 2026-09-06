//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8167-mmsys.h
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
pub const MT8167_DISP_REG_CONFIG_DISP_OVL0_MOUT_EN: c_uint = 0x030;
pub const MT8167_DISP_REG_CONFIG_DISP_DITHER_MOUT_EN: c_uint = 0x038;
pub const MT8167_DISP_REG_CONFIG_DISP_COLOR0_SEL_IN: c_uint = 0x058;
pub const MT8167_DISP_REG_CONFIG_DISP_DSI0_SEL_IN: c_uint = 0x064;
pub const MT8167_DISP_REG_CONFIG_DISP_RDMA0_SOUT_SEL_IN: c_uint = 0x06c;
pub const MT8167_DITHER_MOUT_EN_RDMA0: c_uint = 0x1;
pub const MT8167_DITHER_MOUT_EN_MASK: c_uint = 0x7;
pub const MT8167_RDMA0_SOUT_DSI0: c_uint = 0x2;
pub const MT8167_RDMA0_SOUT_MASK: c_uint = 0x3;
pub const MT8167_DSI0_SEL_IN_RDMA0: c_uint = 0x1;
pub const MT8167_DSI0_SEL_IN_MASK: c_uint = 0x3;
