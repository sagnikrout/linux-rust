//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8173-mmsys.h
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
pub const MT8173_DISP_REG_CONFIG_DISP_OVL0_MOUT_EN: c_uint = 0x040;
pub const MT8173_DISP_REG_CONFIG_DISP_OVL1_MOUT_EN: c_uint = 0x044;
pub const MT8173_DISP_REG_CONFIG_DISP_OD_MOUT_EN: c_uint = 0x048;
pub const MT8173_DISP_REG_CONFIG_DISP_GAMMA_MOUT_EN: c_uint = 0x04c;
pub const MT8173_DISP_REG_CONFIG_DISP_UFOE_MOUT_EN: c_uint = 0x050;
pub const MT8173_DISP_REG_CONFIG_DISP_COLOR0_SEL_IN: c_uint = 0x084;
pub const MT8173_DISP_REG_CONFIG_DISP_COLOR1_SEL_IN: c_uint = 0x088;
pub const MT8173_DISP_REG_CONFIG_DISP_AAL_SEL_IN: c_uint = 0x08c;
pub const MT8173_DISP_REG_CONFIG_DISP_UFOE_SEL_IN: c_uint = 0x0a0;
pub const MT8173_DISP_REG_CONFIG_DSI0_SEL_IN: c_uint = 0x0a4;
pub const MT8173_DISP_REG_CONFIG_DPI_SEL_IN: c_uint = 0x0ac;
pub const MT8173_DISP_REG_CONFIG_DISP_RDMA0_SOUT_SEL_IN: c_uint = 0x0b0;
pub const MT8173_DISP_REG_CONFIG_DISP_RDMA1_SOUT_EN: c_uint = 0x0c8;
pub const MT8173_DISP_REG_CONFIG_DISP_COLOR0_SOUT_SEL_IN: c_uint = 0x0bc;

