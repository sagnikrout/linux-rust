//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/davinci_voicecodec.h
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
// DaVinci Voice Codec Core Interface for TI platforms
//
// Copyright (C) 2010 Texas Instruments, Inc
//
// Author: Miguel Aguilar <miguel.aguilar@ridgerun.com>
//

//
// Register values.
//
pub const DAVINCI_VC_PID: c_uint = 0x00;
pub const DAVINCI_VC_CTRL: c_uint = 0x04;
pub const DAVINCI_VC_INTEN: c_uint = 0x08;
pub const DAVINCI_VC_INTSTATUS: c_uint = 0x0c;
pub const DAVINCI_VC_INTCLR: c_uint = 0x10;
pub const DAVINCI_VC_EMUL_CTRL: c_uint = 0x14;
pub const DAVINCI_VC_RFIFO: c_uint = 0x20;
pub const DAVINCI_VC_WFIFO: c_uint = 0x24;
pub const DAVINCI_VC_FIFOSTAT: c_uint = 0x28;
pub const DAVINCI_VC_TST_CTRL: c_uint = 0x2C;
pub const DAVINCI_VC_REG05: c_uint = 0x94;
pub const DAVINCI_VC_REG09: c_uint = 0xA4;
pub const DAVINCI_VC_REG12: c_uint = 0xB0;
// DAVINCI_VC_CTRL bit fields
pub const DAVINCI_VC_CTRL_MASK: c_uint = 0x5500;

// DAVINCI_VC_INT bit fields
pub const DAVINCI_VC_INT_MASK: c_uint = 0x3F;

// DAVINCI_VC_REG05 bit fields
pub const DAVINCI_VC_REG05_PGA_GAIN: c_uint = 0x07;
// DAVINCI_VC_REG09 bit fields
pub const DAVINCI_VC_REG09_MUTE: c_uint = 0x40;
pub const DAVINCI_VC_REG09_DIG_ATTEN: c_uint = 0x3F;
// DAVINCI_VC_REG12 bit fields
pub const DAVINCI_VC_REG12_POWER_ALL_ON: c_uint = 0xFD;
pub const DAVINCI_VC_REG12_POWER_ALL_OFF: c_uint = 0x00;
pub const DAVINCI_VC_CELLS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum davinci_vc_cells {
    DAVINCI_VC_VCIF_CELL,
    DAVINCI_VC_CQ93VC_CELL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_vcif {
    pub pdev: *mut platform_device,
    pub dma_tx_channel: u32,
    pub dma_rx_channel: u32,
    pub dma_tx_addr: dma_addr_t,
    pub dma_rx_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_vc {
// Device data
    pub dev: *mut device,
    pub pdev: *mut platform_device,
    pub clk: *mut clk,
// Memory resources
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
// MFD cells
    pub cells: [mfd_cell; DAVINCI_VC_CELLS],
// Client devices
    pub davinci_vcif: davinci_vcif,
}
