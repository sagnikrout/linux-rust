//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/edma.h
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

pub const EDMA_MAX_WR_CH: c_int = 8;
pub const EDMA_MAX_RD_CH: c_int = 8;
pub const HDMA_MAX_WR_CH: c_int = 64;
pub const HDMA_MAX_RD_CH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_region {
    pub paddr: u64,
    pub mem: *mut c_void,
    pub io: *mut void __iomem,
    pub vaddr: },
    pub sz: usize,
}

//
// struct dw_edma_plat_ops - platform-specific eDMA methods
// @irq_vector:		Get IRQ number of the passed eDMA channel. Note the
// method accepts the channel id in the end-to-end
// numbering with the eDMA write channels being placed
// first in the row.
// @pci_address:	Get PCIe bus address corresponding to the passed CPU
// address. Note there is no need in specifying this
// function if the address translation is performed by
// the DW PCIe RP/EP controller with the DW eDMA device in
// subject and DMA_BYPASS isn't set for all the outbound
// iATU windows. That will be done by the controller
// automatically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_plat_ops {
    pub nr): *mut *mut *mut int (irq_vector)(struct device dev, unsigned int,
    pub cpu_addr): *mut *mut *mut u64 (pci_address)(struct device dev, phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_map_format {
    EDMA_MF_EDMA_LEGACY = 0x0,
    EDMA_MF_EDMA_UNROLL = 0x1,
    EDMA_MF_HDMA_COMPAT = 0x5,
    EDMA_MF_HDMA_NATIVE = 0x7,
}

//
// enum dw_edma_chip_flags - Flags specific to an eDMA chip
// @DW_EDMA_CHIP_LOCAL:		eDMA is used locally by an endpoint
// @DW_EDMA_CHIP_PARTIAL:	Only channels described by this instance are
// owned by this driver. Controller-wide state
// must be preserved, and layouts with shared
// direction-wide registers must only be shared at
// direction granularity. Layouts with per-channel
// registers may be shared at channel granularity.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_chip_flags {
    DW_EDMA_CHIP_LOCAL	= BIT(0),
    DW_EDMA_CHIP_PARTIAL	= BIT(1),
}

//
// enum dw_edma_ch_irq_mode - per-channel interrupt routing control
// @DW_EDMA_CH_IRQ_LOCAL:     local interrupt only (edma_int[])
// @DW_EDMA_CH_IRQ_REMOTE:    remote interrupt only (IMWr/MSI), without
// delivering local edma_int[].
//
// DesignWare EP eDMA can signal interrupts locally through the edma_int[]
// bus, and remotely using posted memory writes (IMWr) that may be
// interpreted as MSI/MSI-X by the RC.
//
// For the v0 eDMA linked-list programming path, DMA_*_INT_MASK gates the local
// edma_int[] assertion, while there is no dedicated per-channel mask for IMWr
// generation. To request a remote-only interrupt, Synopsys recommends setting
// both LIE and RIE, and masking the local interrupt in DMA_*_INT_MASK. See the
// DesignWare endpoint databook 6.30a, Linked List Mode interrupt handling
// ("Software Programming of an Endpoint's LIE and RIE Bits for Linked List
// Transfers", Attention).
//
// A local (DW_EDMA_CHIP_LOCAL) instance never issues transfers on a
// remote-routed channel: REMOTE routing on such an instance denotes a channel
// handed over to and driven by the remote side, and the recipe above is
// applied by the driving instance.
//
// HDMA linked-list watermark interrupts have the same LWIE/RWIE guidance. HDMA
// non-linked-list mode has dedicated local and remote stop/abort interrupt
// enables.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_edma_ch_irq_mode {
    DW_EDMA_CH_IRQ_LOCAL	= 0,
    DW_EDMA_CH_IRQ_REMOTE,
}

//
// struct dw_edma_chip - representation of DesignWare eDMA controller hardware
// @dev:		 struct device of the eDMA controller
// @nr_irqs:		 total number of DMA IRQs
// @ops:		 DMA channel to IRQ number mapping
// @flags:		 dw_edma_chip_flags
// @reg_base:		 DMA register base address
// @ll_wr_cnt:		 DMA write link list count
// @ll_rd_cnt:		 DMA read link list count
// @ll_region_wr:	 DMA descriptor link list memory for write channel
// @ll_region_rd:	 DMA descriptor link list memory for read channel
// @dt_region_wr:	 DMA data memory for write channel
// @dt_region_rd:	 DMA data memory for read channel
// @db_irq:		 Virtual IRQ dedicated to interrupt emulation
// @db_offset:		 Offset from DMA register base
// @mf:			 DMA register map format
// @func_no:		 PCI endpoint function number used by DMA TLPs
// @dw:			 struct dw_edma that is filled by dw_edma_probe()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_edma_chip {
    pub dev: *mut device,
    pub nr_irqs: c_int,
    pub ops: *const dw_edma_plat_ops,
    pub flags: u32,
    pub reg_base: *mut void __iomem,
    pub ll_wr_cnt: u16,
    pub ll_rd_cnt: u16,
// link list address
    pub ll_region_wr: [dw_edma_region; HDMA_MAX_WR_CH],
    pub ll_region_rd: [dw_edma_region; HDMA_MAX_RD_CH],
// data region
    pub dt_region_wr: [dw_edma_region; HDMA_MAX_WR_CH],
    pub dt_region_rd: [dw_edma_region; HDMA_MAX_RD_CH],
// interrupt emulation
    pub db_irq: c_int,
    pub db_offset: resource_size_t,
    pub mf: dw_edma_map_format,
    pub func_no: u8,
    pub dw: *mut dw_edma,
    pub cfg_non_ll: bool,
}

// Export to the platform drivers

extern "C" {
    pub fn dw_edma_probe(chip: *mut dw_edma_chip) -> c_int;
}
extern "C" {
    pub fn dw_edma_remove(chip: *mut dw_edma_chip) -> c_int;
}

