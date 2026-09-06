//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ti/omap-dmic.h
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
//
// omap-dmic.h  --  OMAP Digital Microphone Controller
//
pub const OMAP_DMIC_REVISION_REG: c_uint = 0x00;
pub const OMAP_DMIC_SYSCONFIG_REG: c_uint = 0x10;
pub const OMAP_DMIC_IRQSTATUS_RAW_REG: c_uint = 0x24;
pub const OMAP_DMIC_IRQSTATUS_REG: c_uint = 0x28;
pub const OMAP_DMIC_IRQENABLE_SET_REG: c_uint = 0x2C;
pub const OMAP_DMIC_IRQENABLE_CLR_REG: c_uint = 0x30;
pub const OMAP_DMIC_IRQWAKE_EN_REG: c_uint = 0x34;
pub const OMAP_DMIC_DMAENABLE_SET_REG: c_uint = 0x38;
pub const OMAP_DMIC_DMAENABLE_CLR_REG: c_uint = 0x3C;
pub const OMAP_DMIC_DMAWAKEEN_REG: c_uint = 0x40;
pub const OMAP_DMIC_CTRL_REG: c_uint = 0x44;
pub const OMAP_DMIC_DATA_REG: c_uint = 0x48;
pub const OMAP_DMIC_FIFO_CTRL_REG: c_uint = 0x4C;
pub const OMAP_DMIC_FIFO_DMIC1R_DATA_REG: c_uint = 0x50;
pub const OMAP_DMIC_FIFO_DMIC1L_DATA_REG: c_uint = 0x54;
pub const OMAP_DMIC_FIFO_DMIC2R_DATA_REG: c_uint = 0x58;
pub const OMAP_DMIC_FIFO_DMIC2L_DATA_REG: c_uint = 0x5C;
pub const OMAP_DMIC_FIFO_DMIC3R_DATA_REG: c_uint = 0x60;
pub const OMAP_DMIC_FIFO_DMIC3L_DATA_REG: c_uint = 0x64;
// IRQSTATUS_RAW, IRQSTATUS, IRQENABLE_SET, IRQENABLE_CLR bit fields

pub const OMAP_DMIC_IRQ_MASK: c_uint = 0x07;
// DMIC_DMAENABLE bit fields
pub const OMAP_DMIC_DMA_ENABLE: c_uint = 0x1;
// DMIC_CTRL bit fields

pub const OMAP_DMIC_UP_ENABLE_MASK: c_uint = 0x7;

// DMIC_FIFO_CTRL bit fields
pub const OMAP_DMIC_THRES_MAX: c_uint = 0xF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dmic_clk {
    OMAP_DMIC_SYSCLK_PAD_CLKS,		/* PAD_CLKS */
    OMAP_DMIC_SYSCLK_SLIMBLUS_CLKS,		/* SLIMBUS_CLK */
    OMAP_DMIC_SYSCLK_SYNC_MUX_CLKS,		/* DMIC_SYNC_MUX_CLK */
    OMAP_DMIC_ABE_DMIC_CLK,			/* abe_dmic_clk */
}
