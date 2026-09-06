//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dw/regs.h
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
// Driver for the Synopsys DesignWare AHB DMA Controller
//
// Copyright (C) 2005-2007 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
// Copyright (C) 2016 Intel Corporation
//

pub const DW_DMA_MAX_NR_REQUESTS: c_int = 16;
// flow controller
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_dma_fc {
    DW_DMA_FC_D_M2M,
    DW_DMA_FC_D_M2P,
    DW_DMA_FC_D_P2M,
    DW_DMA_FC_D_P2P,
    DW_DMA_FC_P_P2M,
    DW_DMA_FC_SP_P2P,
    DW_DMA_FC_P_M2P,
    DW_DMA_FC_DP_P2P,
}

//
// Redefine this macro to handle differences between 32- and 64-bit
// addressing, big vs. little endian, etc.
//

// Hardware register definitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_chan_regs {
    pub /: *mut *mut DW_REG(SAR); / Source Address Register,
    pub /: *mut *mut DW_REG(DAR); / Destination Address Register,
    pub /: *mut *mut DW_REG(LLP); / Linked List Pointer,
    pub /: *mut *mut u32 CTL_LO; / Control Register Low,
    pub /: *mut *mut u32 CTL_HI; / Control Register High,
    pub /: *mut *mut u32 CFG_LO; / Configuration Register Low,
    pub /: *mut *mut u32 CFG_HI; / Configuration Register High,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_irq_regs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_regs {
// per-channel registers
    pub CHAN: [dw_dma_chan_regs; DW_DMA_MAX_NR_CHANNELS],
// irq handling
    pub /: *mut *mut dw_dma_irq_regs RAW; / r,
    pub /: *mut *mut dw_dma_irq_regs STATUS; / r (raw & mask),
    pub /: *mut *mut dw_dma_irq_regs MASK; / rw (set = irq enabled),
    pub /: *mut *mut dw_dma_irq_regs CLEAR; / w (ack, affects "raw"),
    pub /: *mut *mut DW_REG(STATUS_INT); / r,
// software handshaking
// miscellaneous
// iDMA 32-bit support
// optional encoded params, 0x3c8..0x3f7
    pub __reserved: u32,
// per-channel configuration registers
    pub DWC_PARAMS: [u32; DW_DMA_MAX_NR_CHANNELS],
    pub MULTI_BLK_TYPE: u32,
    pub MAX_BLK_SIZE: u32,
// top-level parameters
    pub DW_PARAMS: u32,
// component ID
    pub COMP_TYPE: u32,
    pub COMP_VERSION: u32,
// iDMA 32-bit support
}

// Bitfields in DW_PARAMS

// Bitfields in DWC_PARAMS

// bursts size
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_dma_msize {
    DW_DMA_MSIZE_1,
    DW_DMA_MSIZE_4,
    DW_DMA_MSIZE_8,
    DW_DMA_MSIZE_16,
    DW_DMA_MSIZE_32,
    DW_DMA_MSIZE_64,
    DW_DMA_MSIZE_128,
    DW_DMA_MSIZE_256,
}

// Bitfields in LLP

// Bitfields in CTL_LO

// plus 4 transfer types for peripheral-as-flow-controller

// Bitfields in CTL_HI

// Bitfields in CFG_LO

// Bitfields in CFG_HI

// Bitfields in SGR

// Bitfields in DSR

// Bitfields in CFG

// iDMA 32-bit support
// bursts size
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idma32_msize {
    IDMA32_MSIZE_1,
    IDMA32_MSIZE_2,
    IDMA32_MSIZE_4,
    IDMA32_MSIZE_8,
    IDMA32_MSIZE_16,
    IDMA32_MSIZE_32,
}

// Bitfields in CTL_HI

// Bitfields in CFG_LO

// Bitfields in CFG_HI

// Bitfields in FIFO_PARTITION

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_dmac_flags {
    DW_DMA_IS_CYCLIC = 0,
    DW_DMA_IS_SOFT_LLP = 1,
    DW_DMA_IS_PAUSED = 2,
    DW_DMA_IS_INITIALIZED = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma_chan {
    pub chan: dma_chan,
    pub ch_regs: *mut void __iomem,
    pub mask: u8,
    pub priority: u8,
    pub direction: dma_transfer_direction,
// software emulation of the LLP transfers
    pub tx_node_active: *mut list_head,
    pub lock: spinlock_t,
// these other elements are all protected by lock
    pub flags: c_ulong,
    pub active_list: list_head,
    pub queue: list_head,
    pub descs_allocated: c_uint,
// hardware configuration
    pub block_size: c_uint,
    pub nollp: bool,
    pub max_burst: u32,
// custom slave configuration
    pub dws: dw_dma_slave,
// configuration passed via .device_config
    pub dma_sconfig: dma_slave_config,
}

extern "C" {
    pub fn container_of(_arg: chan, dw_dma_chan: struct, _arg: chan) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_dma {
    pub dma: dma_device,
    pub name: [c_char; 20],
    pub regs: *mut void __iomem,
    pub desc_pool: *mut dma_pool,
    pub tasklet: tasklet_struct,
// channels
    pub chan: *mut dw_dma_chan,
    pub all_chan_mask: u8,
    pub in_use: u8,
// Channel operations
    pub dwc): *mut *mut void (initialize_chan)(struct dw_dma_chan,
    pub drain): *mut *mut *mut void (suspend_chan)(struct dw_dma_chan dwc, bool,
    pub drain): *mut *mut *mut void (resume_chan)(struct dw_dma_chan dwc, bool,
    pub dwc): *mut *mut u32 (prepare_ctllo)(struct dw_dma_chan,
    pub len): *mut unsigned int width, size_t,
    pub width): *mut *mut *mut size_t (block2bytes)(struct dw_dma_chan dwc, u32 block, u32,
// Device operations
    pub id): *mut *mut *mut void (set_device_name)(struct dw_dma dw, int,
    pub dw): *mut *mut void (disable)(struct dw_dma,
    pub dw): *mut *mut void (enable)(struct dw_dma,
// platform data
    pub pdata: *mut dw_dma_platform_data,
}

extern "C" {
    pub fn container_of(_arg: ddev, dw_dma: struct, _arg: dma) -> return;
}
// LLI == Linked List Item; a.k.a. DMA block descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_lli {
// values that are not changed by hardware
    pub sar: __le32,
    pub dar: __le32,
    pub /: *mut *mut __le32 llp; / chain to next lli,
    pub ctllo: __le32,
// values that may get written back:
    pub ctlhi: __le32,
// sstat and dstat can snapshot peripheral register state.
// silicon config may discard either or both...
//
    pub sstat: __le32,
    pub dstat: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_desc {
// FIRST values the hardware uses
    pub lli: dw_lli,

// THEN values for driver housekeeping
    pub desc_node: list_head,
    pub tx_list: list_head,
    pub txd: dma_async_tx_descriptor,
    pub len: usize,
    pub total_len: usize,
    pub residue: u32,
}

extern "C" {
    pub fn container_of(_arg: txd, dw_desc: struct, _arg: txd) -> return;
}
