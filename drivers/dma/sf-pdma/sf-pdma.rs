//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/sf-pdma/sf-pdma.h
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
// SiFive FU540 Platform DMA driver
// Copyright (C) 2019 SiFive
//
// Based partially on:
// - drivers/dma/fsl-edma.c
// - drivers/dma/dw-edma
// - drivers/dma/pxa-dma.c
//
// See the following sources for further documentation:
// - Chapter 12 "Platform DMA Engine (PDMA)" of
// SiFive FU540-C000 v1.0
// https://static.dev.sifive.com/FU540-C000-v1.0.pdf
//

pub const PDMA_MAX_NR_CH: c_int = 4;
pub const PDMA_BASE_ADDR: c_uint = 0x3000000;
pub const PDMA_CHAN_OFFSET: c_uint = 0x1000;
// Register Offset
pub const PDMA_CTRL: c_uint = 0x000;
pub const PDMA_XFER_TYPE: c_uint = 0x004;
pub const PDMA_XFER_SIZE: c_uint = 0x008;
pub const PDMA_DST_ADDR: c_uint = 0x010;
pub const PDMA_SRC_ADDR: c_uint = 0x018;
pub const PDMA_ACT_TYPE: c_uint = 0x104 /* Read-only */;
pub const PDMA_REMAINING_BYTE: c_uint = 0x108 /* Read-only */;
pub const PDMA_CUR_DST_ADDR: c_uint = 0x110 /* Read-only*/;
pub const PDMA_CUR_SRC_ADDR: c_uint = 0x118 /* Read-only*/;
// CTRL
pub const PDMA_CLEAR_CTRL: c_uint = 0x0;

// Transfer Type
pub const PDMA_FULL_SPEED: c_uint = 0xFF000000;

// Error Recovery
pub const MAX_RETRY: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdma_regs {
// read-write regs
    pub /: *mut *mut *mut void __iomem ctrl; / 4 bytes,
    pub /: *mut *mut *mut void __iomem xfer_type; / 4 bytes,
    pub /: *mut *mut *mut void __iomem xfer_size; / 8 bytes,
    pub /: *mut *mut *mut void __iomem dst_addr; / 8 bytes,
    pub /: *mut *mut *mut void __iomem src_addr; / 8 bytes,
// read-only
    pub /: *mut *mut *mut void __iomem act_type; / 4 bytes,
    pub /: *mut *mut *mut void __iomem residue; / 8 bytes,
    pub /: *mut *mut *mut void __iomem cur_dst_addr; / 8 bytes,
    pub /: *mut *mut *mut void __iomem cur_src_addr; / 8 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sf_pdma_desc {
    pub xfer_type: u32,
    pub xfer_size: u64,
    pub dst_addr: u64,
    pub src_addr: u64,
    pub vdesc: virt_dma_desc,
    pub chan: *mut sf_pdma_chan,
    pub dirn: dma_transfer_direction,
    pub async_tx: *mut dma_async_tx_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sf_pdma_pm_state {
    RUNNING = 0,
    SUSPENDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sf_pdma_chan {
    pub vchan: virt_dma_chan,
    pub status: dma_status,
    pub pm_state: sf_pdma_pm_state,
    pub slave_id: u32,
    pub pdma: *mut sf_pdma,
    pub desc: *mut sf_pdma_desc,
    pub cfg: dma_slave_config,
    pub attr: u32,
    pub dma_dev_addr: dma_addr_t,
    pub dma_dev_size: u32,
    pub done_tasklet: tasklet_struct,
    pub err_tasklet: tasklet_struct,
    pub regs: pdma_regs,
    pub /: *mut *mut spinlock_t lock; / protect chan data,
    pub xfer_err: bool,
    pub txirq: c_int,
    pub errirq: c_int,
    pub retries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sf_pdma {
    pub dma_dev: dma_device,
    pub membase: *mut void __iomem,
    pub mappedbase: *mut void __iomem,
    pub transfer_type: u32,
    pub n_chans: u32,
    pub __counted_by(n_chans): sf_pdma_chan chans[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sf_pdma_driver_platdata {
    pub quirks: u32,
}
