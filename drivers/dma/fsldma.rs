//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/fsldma.h
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
// Copyright (C) 2007-2010 Freescale Semiconductor, Inc. All rights reserved.
//
// Author:
// Zhang Wei <wei.zhang@freescale.com>, Jul 2007
// Ebony Zhu <ebony.zhu@freescale.com>, May 2007
//

// Define data structures needed by Freescale
// MPC8540 and MPC8349 DMA controller.
//
pub const FSL_DMA_MR_CS: c_uint = 0x00000001;
pub const FSL_DMA_MR_CC: c_uint = 0x00000002;
pub const FSL_DMA_MR_CA: c_uint = 0x00000008;
pub const FSL_DMA_MR_EIE: c_uint = 0x00000040;
pub const FSL_DMA_MR_XFE: c_uint = 0x00000020;
pub const FSL_DMA_MR_EOLNIE: c_uint = 0x00000100;
pub const FSL_DMA_MR_EOLSIE: c_uint = 0x00000080;
pub const FSL_DMA_MR_EOSIE: c_uint = 0x00000200;
pub const FSL_DMA_MR_CDSM: c_uint = 0x00000010;
pub const FSL_DMA_MR_CTM: c_uint = 0x00000004;
pub const FSL_DMA_MR_EMP_EN: c_uint = 0x00200000;
pub const FSL_DMA_MR_EMS_EN: c_uint = 0x00040000;
pub const FSL_DMA_MR_DAHE: c_uint = 0x00002000;
pub const FSL_DMA_MR_SAHE: c_uint = 0x00001000;
pub const FSL_DMA_MR_SAHTS_MASK: c_uint = 0x0000C000;
pub const FSL_DMA_MR_DAHTS_MASK: c_uint = 0x00030000;
pub const FSL_DMA_MR_BWC_MASK: c_uint = 0x0f000000;
//
// Bandwidth/pause control determines how many bytes a given
// channel is allowed to transfer before the DMA engine pauses
// the current channel and switches to the next channel
//
pub const FSL_DMA_MR_BWC: c_uint = 0x0A000000;
// Special MR definition for MPC8349
pub const FSL_DMA_MR_EOTIE: c_uint = 0x00000080;
pub const FSL_DMA_MR_PRC_RM: c_uint = 0x00000800;
pub const FSL_DMA_SR_CH: c_uint = 0x00000020;
pub const FSL_DMA_SR_PE: c_uint = 0x00000010;
pub const FSL_DMA_SR_CB: c_uint = 0x00000004;
pub const FSL_DMA_SR_TE: c_uint = 0x00000080;
pub const FSL_DMA_SR_EOSI: c_uint = 0x00000002;
pub const FSL_DMA_SR_EOLSI: c_uint = 0x00000001;
pub const FSL_DMA_SR_EOCDI: c_uint = 0x00000001;
pub const FSL_DMA_SR_EOLNI: c_uint = 0x00000008;
pub const FSL_DMA_SATR_SBPATMU: c_uint = 0x20000000;
pub const FSL_DMA_SATR_STRANSINT_RIO: c_uint = 0x00c00000;
pub const FSL_DMA_SATR_SREADTYPE_SNOOP_READ: c_uint = 0x00050000;
pub const FSL_DMA_SATR_SREADTYPE_BP_IORH: c_uint = 0x00020000;
pub const FSL_DMA_SATR_SREADTYPE_BP_NREAD: c_uint = 0x00040000;
pub const FSL_DMA_SATR_SREADTYPE_BP_MREAD: c_uint = 0x00070000;
pub const FSL_DMA_DATR_DBPATMU: c_uint = 0x20000000;
pub const FSL_DMA_DATR_DTRANSINT_RIO: c_uint = 0x00c00000;
pub const FSL_DMA_DATR_DWRITETYPE_SNOOP_WRITE: c_uint = 0x00050000;
pub const FSL_DMA_DATR_DWRITETYPE_BP_FLUSH: c_uint = 0x00010000;

pub const FSL_DMA_EOSIE: c_uint = 0x8;

pub const FSL_DMA_BCR_MAX_CNT: c_uint = 0x03ffffffu;
pub const FSL_DMA_DGSR_TE: c_uint = 0x80;
pub const FSL_DMA_DGSR_CH: c_uint = 0x20;
pub const FSL_DMA_DGSR_PE: c_uint = 0x10;
pub const FSL_DMA_DGSR_EOLNI: c_uint = 0x08;
pub const FSL_DMA_DGSR_CB: c_uint = 0x04;
pub const FSL_DMA_DGSR_EOSI: c_uint = 0x02;
pub const FSL_DMA_DGSR_EOLSI: c_uint = 0x01;

pub type v64 = u64 ;
pub type v32 = u32 ;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dma_ld_hw {
    pub src_addr: v64,
    pub dst_addr: v64,
    pub next_ln_addr: v64,
    pub count: v32,
    pub reserve: v32,
    pub __attribute__((aligned(32))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_desc_sw {
    pub hw: fsl_dma_ld_hw,
    pub node: list_head,
    pub tx_list: list_head,
    pub async_tx: dma_async_tx_descriptor,
    pub __attribute__((aligned(32))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsldma_chan_regs {
    pub /: *mut *mut u32 mr; / 0x00 - Mode Register,
    pub /: *mut *mut u32 sr; / 0x04 - Status Register,
    pub /: *mut *mut u64 cdar; / 0x08 - Current descriptor address register,
    pub /: *mut *mut u64 sar; / 0x10 - Source Address Register,
    pub /: *mut *mut u64 dar; / 0x18 - Destination Address Register,
    pub /: *mut *mut u32 bcr; / 0x20 - Byte Count Register,
    pub /: *mut *mut u64 ndar; / 0x24 - Next Descriptor Address Register,
}

pub const FSL_DMA_MAX_CHANS_PER_DEVICE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsldma_device {
    pub /: *mut *mut *mut void __iomem regs; / DGSR register base,
    pub dev: *mut device,
    pub common: dma_device,
    pub chan: [*mut fsldma_chan; FSL_DMA_MAX_CHANS_PER_DEVICE],
    pub /: *mut *mut u32 feature; / The same as DMA channels,
    pub /: *mut *mut int irq; / Channel IRQ,
    pub /: *mut *mut int addr_bits; / DMA addressing bits supported,
}

// Define macros for fsldma_chan->feature property
pub const FSL_DMA_LITTLE_ENDIAN: c_uint = 0x00000000;
pub const FSL_DMA_BIG_ENDIAN: c_uint = 0x00000001;
pub const FSL_DMA_IP_MASK: c_uint = 0x00000ff0;
pub const FSL_DMA_IP_85XX: c_uint = 0x00000010;
pub const FSL_DMA_IP_83XX: c_uint = 0x00000020;
pub const FSL_DMA_CHAN_PAUSE_EXT: c_uint = 0x00001000;
pub const FSL_DMA_CHAN_START_EXT: c_uint = 0x00002000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsldma_chan_regs_save {
    pub mr: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsldma_pm_state {
    RUNNING = 0,
    SUSPENDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsldma_chan {
    pub /: *mut *mut char name[8]; / Channel name,
    pub regs: *mut fsldma_chan_regs __iomem,
    pub /: *mut *mut spinlock_t desc_lock; / Descriptor operation lock,
//
// Descriptors which are queued to run, but have not yet been
// submitted to the hardware for execution
//
    pub ld_pending: list_head,
//
// Descriptors which are currently being executed by the hardware
//
    pub ld_running: list_head,
//
// Descriptors which have finished execution by the hardware. These
// descriptors have already had their cleanup actions run. They are
// waiting for the ACK bit to be set by the async_tx API.
//
    pub /: *mut *mut list_head ld_completed; / Link descriptors queue,
    pub /: *mut *mut dma_chan common; / DMA common channel,
    pub /: *mut *mut *mut dma_pool desc_pool; / Descriptors pool,
    pub /: *mut *mut *mut device dev; / Channel device,
    pub /: *mut *mut int irq; / Channel IRQ,
    pub /: *mut *mut int id; / Raw id of this channel,
    pub tasklet: tasklet_struct,
    pub feature: u32,
    pub /: *mut *mut bool idle; / DMA controller is idle,

    pub regs_save: fsldma_chan_regs_save,
    pub pm_state: fsldma_pm_state,

    pub enable): *mut *mut *mut void (toggle_ext_pause)(struct fsldma_chan fsl_chan, int,
    pub enable): *mut *mut *mut void (toggle_ext_start)(struct fsldma_chan fsl_chan, int,
    pub size): *mut *mut *mut void (set_src_loop_size)(struct fsldma_chan fsl_chan, int,
    pub size): *mut *mut *mut void (set_dst_loop_size)(struct fsldma_chan fsl_chan, int,
    pub size): *mut *mut *mut void (set_request_count)(struct fsldma_chan fsl_chan, int,
}

