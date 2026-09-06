//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8186-mmsys.h
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
// Values for DPI configuration in MMSYS address space
pub const MT8186_MMSYS_DPI_OUTPUT_FORMAT: c_uint = 0x400;

pub const MT8186_DPI_RGB888_SDR_CON: c_int = 0;
pub const MT8186_DPI_RGB888_DDR_CON: c_int = 1;
pub const MT8186_DPI_RGB565_SDR_CON: c_int = 2;
pub const MT8186_DPI_RGB565_DDR_CON: c_int = 3;
pub const MT8186_MMSYS_OVL_CON: c_uint = 0xF04;
pub const MT8186_MMSYS_OVL0_CON_MASK: c_uint = 0x3;
pub const MT8186_MMSYS_OVL0_2L_CON_MASK: c_uint = 0xC;

pub const MT8186_DISP_RDMA0_SOUT_SEL: c_uint = 0xF0C;
pub const MT8186_RDMA0_SOUT_SEL_MASK: c_uint = 0xF;

pub const MT8186_DISP_OVL0_2L_MOUT_EN: c_uint = 0xF14;
pub const MT8186_OVL0_2L_MOUT_EN_MASK: c_uint = 0xF;

pub const MT8186_DISP_OVL0_MOUT_EN: c_uint = 0xF18;
pub const MT8186_OVL0_MOUT_EN_MASK: c_uint = 0xF;

pub const MT8186_DISP_DITHER0_MOUT_EN: c_uint = 0xF20;
pub const MT8186_DITHER0_MOUT_EN_MASK: c_uint = 0xF;

pub const MT8186_DISP_RDMA0_SEL_IN: c_uint = 0xF28;
pub const MT8186_RDMA0_SEL_IN_MASK: c_uint = 0xF;
pub const MT8186_RDMA0_FROM_OVL0: c_int = 0;
pub const MT8186_RDMA0_FROM_OVL0_2L: c_int = 2;
pub const MT8186_DISP_DSI0_SEL_IN: c_uint = 0xF30;
pub const MT8186_DSI0_SEL_IN_MASK: c_uint = 0xF;
pub const MT8186_DSI0_FROM_RDMA0: c_int = 0;
pub const MT8186_DSI0_FROM_DITHER0: c_int = 1;
pub const MT8186_DSI0_FROM_RDMA1: c_int = 2;
pub const MT8186_DISP_RDMA1_MOUT_EN: c_uint = 0xF3C;
pub const MT8186_RDMA1_MOUT_EN_MASK: c_uint = 0xF;

pub const MT8186_DISP_RDMA1_SEL_IN: c_uint = 0xF40;
pub const MT8186_RDMA1_SEL_IN_MASK: c_uint = 0xF;
pub const MT8186_RDMA1_FROM_OVL0: c_int = 0;
pub const MT8186_RDMA1_FROM_OVL0_2L: c_int = 2;
pub const MT8186_RDMA1_FROM_DITHER0: c_int = 3;
pub const MT8186_DISP_DPI0_SEL_IN: c_uint = 0xF44;
pub const MT8186_DPI0_SEL_IN_MASK: c_uint = 0xF;
pub const MT8186_DPI0_FROM_RDMA1: c_int = 0;
pub const MT8186_DPI0_FROM_DITHER0: c_int = 1;
pub const MT8186_DPI0_FROM_RDMA0: c_int = 2;
pub const MT8186_MMSYS_SW0_RST_B: c_uint = 0x160;
