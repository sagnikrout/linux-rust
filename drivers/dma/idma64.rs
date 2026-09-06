//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/idma64.h
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
// Driver for the Intel integrated DMA 64-bit
//
// Copyright (C) 2015 Intel Corporation
//

// Channel registers
pub const IDMA64_CH_SAR: c_uint = 0x00	/* Source Address Register */;
pub const IDMA64_CH_DAR: c_uint = 0x08	/* Destination Address Register */;
pub const IDMA64_CH_LLP: c_uint = 0x10	/* Linked List Pointer */;
pub const IDMA64_CH_CTL_LO: c_uint = 0x18	/* Control Register Low */;
pub const IDMA64_CH_CTL_HI: c_uint = 0x1c	/* Control Register High */;
pub const IDMA64_CH_SSTAT: c_uint = 0x20;
pub const IDMA64_CH_DSTAT: c_uint = 0x28;
pub const IDMA64_CH_SSTATAR: c_uint = 0x30;
pub const IDMA64_CH_DSTATAR: c_uint = 0x38;
pub const IDMA64_CH_CFG_LO: c_uint = 0x40	/* Configuration Register Low */;
pub const IDMA64_CH_CFG_HI: c_uint = 0x44	/* Configuration Register High */;
pub const IDMA64_CH_SGR: c_uint = 0x48;
pub const IDMA64_CH_DSR: c_uint = 0x50;
pub const IDMA64_CH_LENGTH: c_uint = 0x58;
// Bitfields in CTL_LO

// Bitfields in CTL_HI

// Bitfields in CFG_LO

// Bitfields in CFG_HI

// Interrupt registers
pub const IDMA64_INT_XFER: c_uint = 0x00;
pub const IDMA64_INT_BLOCK: c_uint = 0x08;
pub const IDMA64_INT_SRC_TRAN: c_uint = 0x10;
pub const IDMA64_INT_DST_TRAN: c_uint = 0x18;
pub const IDMA64_INT_ERROR: c_uint = 0x20;

// Common registers
pub const IDMA64_STATUS_INT: c_uint = 0x360	/* r */;
pub const IDMA64_CFG: c_uint = 0x398;
pub const IDMA64_CH_EN: c_uint = 0x3a0;
// Bitfields in CFG

// Hardware descriptor for Linked LIst transfers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64_lli {
    pub sar: u64,
    pub dar: u64,
    pub llp: u64,
    pub ctllo: u32,
    pub ctlhi: u32,
    pub sstat: u32,
    pub dstat: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64_hw_desc {
    pub lli: *mut idma64_lli,
    pub llp: dma_addr_t,
    pub phys: dma_addr_t,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64_desc {
    pub vdesc: virt_dma_desc,
    pub direction: dma_transfer_direction,
    pub hw: *mut idma64_hw_desc,
    pub ndesc: c_uint,
    pub length: usize,
    pub status: dma_status,
}

extern "C" {
    pub fn container_of(_arg: vdesc, idma64_desc: struct, _arg: vdesc) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64_chan {
    pub vchan: virt_dma_chan,
    pub regs: *mut void __iomem,
// hardware configuration
    pub direction: dma_transfer_direction,
    pub mask: c_uint,
    pub config: dma_slave_config,
    pub pool: *mut c_void,
    pub desc: *mut idma64_desc,
}

extern "C" {
    pub fn container_of(_arg: chan, idma64_chan: struct, _arg: vchan.chan) -> return;
}

extern "C" {
    pub fn readl(offset: idma64c->regs +) -> return;
}

extern "C" {
    pub fn lo_hi_readq(offset: idma64c->regs +) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64 {
    pub dma: dma_device,
    pub regs: *mut void __iomem,
// channels
    pub all_chan_mask: c_ushort,
    pub chan: *mut idma64_chan,
}

extern "C" {
    pub fn container_of(_arg: ddev, idma64: struct, _arg: dma) -> return;
}
extern "C" {
    pub fn readl(offset: idma64->regs +) -> return;
}

//
// struct idma64_chip - representation of iDMA 64-bit controller hardware
// @dev:		struct device of the DMA controller
// @sysdev:		struct device of the physical device that does DMA
// @irq:		irq line
// @regs:		memory mapped I/O space
// @idma64:		struct idma64 that is filed by idma64_probe()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idma64_chip {
    pub dev: *mut device,
    pub sysdev: *mut device,
    pub irq: c_int,
    pub regs: *mut void __iomem,
    pub idma64: *mut idma64,
}
