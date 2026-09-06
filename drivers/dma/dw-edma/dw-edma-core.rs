//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw-edma/dw-edma-core.h
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
// Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
// Synopsys DesignWare eDMA core driver
//
// Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
//

pub const EDMA_LL_SZ: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_dir {
    EDMA_DIR_WRITE = 0,
    EDMA_DIR_READ
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_request {
    EDMA_REQ_NONE = 0,
    EDMA_REQ_STOP,
    EDMA_REQ_PAUSE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_status {
    EDMA_ST_IDLE = 0,
    EDMA_ST_PAUSE,
    EDMA_ST_BUSY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_xfer_type {
    EDMA_XFER_SCATTER_GATHER = 0,
    EDMA_XFER_CYCLIC,
    EDMA_XFER_INTERLEAVED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_burst {
    pub sar: u64,
    pub dar: u64,
    pub sz: u32,
// precalulate summary of previous burst total size
    pub xfer_sz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_desc {
    pub vd: virt_dma_desc,
    pub chan: *mut dw_edma_chan,
    pub alloc_sz: u32,
    pub done_burst: usize,
    pub start_burst: usize,
    pub cb: u8,
    pub nburst: usize,
    pub __counted_by(nburst): dw_edma_burst burst[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_chan {
    pub vc: virt_dma_chan,
    pub dw: *mut dw_edma,
    pub id: c_int,
    pub dir: dw_edma_dir,
    pub func_no: u8,
    pub ll_max: u32,
    pub /: *mut *mut dw_edma_region ll_region; / Linked list,
    pub msi: msi_msg,
    pub irq_mode: dw_edma_ch_irq_mode,
    pub request: dw_edma_request,
    pub status: dw_edma_status,
    pub configured: u8,
    pub config: dma_slave_config,
    pub non_ll: bool,
    pub irq_work: work_struct,
    pub irq_pending: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_irq {
    pub msi: msi_msg,
    pub dw: *mut dw_edma,
    pub HDMA_MAX_WR_CH): DECLARE_BITMAP(wr_mask,,
    pub HDMA_MAX_RD_CH): DECLARE_BITMAP(rd_mask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma {
    pub name: [c_char; 32],
    pub dma: dma_device,
    pub wr_ch_cnt: u16,
    pub rd_ch_cnt: u16,
    pub irq: *mut dw_edma_irq,
    pub nr_irqs: c_int,
    pub chan: *mut dw_edma_chan,
//
// WQ_HIGHPRI keeps completion processing responsive under heavy load;
// WQ_UNBOUND lets different channels run on different CPUs.
//
    pub wq: *mut workqueue_struct,
    pub /: *mut *mut raw_spinlock_t lock; / Protect v0 shared registers,
    pub chip: *mut dw_edma_chip,
    pub core: *const dw_edma_core_ops,
}

extern "C" {
    pub fn void(: *mut *mut dw_edma_handler_t)(struct dw_edma_chan) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_core_ops {
    pub dw): *mut *mut void (off)(struct dw_edma,
    pub dw): *mut *mut int (quiesce)(struct dw_edma,
    pub chan): *mut *mut int (ch_quiesce)(struct dw_edma_chan,
    pub dir): *mut *mut *mut u16 (ch_count)(struct dw_edma dw, enum dw_edma_dir,
    pub chan): *mut *mut dma_status (ch_status)(struct dw_edma_chan,
    pub abort): dw_edma_handler_t done, dw_edma_handler_t,
    pub child): *mut *mut *mut void (non_ll_start)(struct dw_edma_chan chan, struct dw_edma_burst,
    pub irq): u32 idx, bool cb, bool,
    pub addr): *mut *mut *mut void (ll_link)(struct dw_edma_chan chan, u32 idx, bool cb, u64,
    pub chan): *mut *mut void (ch_doorbell)(struct dw_edma_chan,
    pub chan): *mut *mut void (ch_enable)(struct dw_edma_chan,
    pub chan): *mut *mut void (ch_config)(struct dw_edma_chan,
    pub dw): *mut *mut void (debugfs_on)(struct dw_edma,
    pub dw): *mut *mut void (ack_emulated_irq)(struct dw_edma,
    pub dw): *mut *mut resource_size_t (db_offset)(struct dw_edma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_sg {
    pub sgl: *mut scatterlist,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_cyclic {
    pub paddr: dma_addr_t,
    pub len: usize,
    pub cnt: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_transfer {
    pub dchan: *mut dma_chan,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dw_edma_xfer {
    pub sg: dw_edma_sg,
    pub cyclic: dw_edma_cyclic,
    pub il: *mut dma_interleaved_template,
    pub xfer: },
    pub direction: dma_transfer_direction,
    pub flags: c_ulong,
    pub type: dw_edma_xfer_type,
}

extern "C" {
    pub fn container_of(_arg: vc, dw_edma_chan: struct, _arg: vc) -> return;
}
extern "C" {
    pub fn vc2dw_edma_chan(_arg: to_virt_chan(dchan)) -> return;
}
