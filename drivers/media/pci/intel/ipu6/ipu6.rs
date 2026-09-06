//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6.h
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
// Copyright (C) 2013 - 2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_version {
    IPU6_VER_INVALID = 0,
    IPU6_VER_6 = 1,
    IPU6_VER_6SE = 3,
    IPU6_VER_6EP = 5,
    IPU6_VER_6EP_MTL = 6,
}

//
// IPU6 - TGL
// IPU6SE - JSL
// IPU6EP - ADL/RPL
// IPU6EP_MTL - MTL
//
// ISYS DMA can overshoot. For higher resolutions over allocation is one line
// but it must be at minimum 1024 bytes. Value could be different in
// different versions / generations thus provide it via platform data.
//
pub const IPU6_ISYS_OVERALLOC_MIN: c_int = 1024;
// Physical pages in GDA is 128, page size is 2K for IPU6, 1K for others
pub const IPU6_DEVICE_GDA_NR_PAGES: c_int = 128;
// Virtualization factor to calculate the available virtual pages
pub const IPU6_DEVICE_GDA_VIRT_FACTOR: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_device {
    pub pdev: *mut pci_dev,
    pub devices: list_head,
    pub isys: *mut ipu6_bus_device,
    pub psys: *mut ipu6_bus_device,
    pub buttress: ipu6_buttress,
    pub cpd_fw: *const firmware,
    pub cpd_fw_name: *const c_char,
    pub cpd_metadata_cmpnt_size: u32,
    pub base: *mut void __iomem,
    pub need_ipc_reset: bool,
    pub secure_mode: bool,
    pub hw_ver: u8,
    pub bus_ready_to_probe: bool,
}

pub const IPU6_MMU_MAX_DEVICES: c_int = 4;
pub const IPU6_MMU_ADDR_BITS: c_int = 32;
// The firmware is accessible within the first 2 GiB only in non-secure mode.
pub const IPU6_MMU_ADDR_BITS_NON_SECURE: c_int = 31;
pub const IPU6_MMU_MAX_TLB_L1_STREAMS: c_int = 32;
pub const IPU6_MMU_MAX_TLB_L2_STREAMS: c_int = 32;
pub const IPU6_MAX_LI_BLOCK_ADDR: c_int = 128;
pub const IPU6_MAX_L2_BLOCK_ADDR: c_int = 64;

//
// To maximize the IOSF utlization, IPU6 need to send requests in bursts.
// At the DMA interface with the buttress, there are CDC FIFOs with burst
// collection capability. CDC FIFO burst collectors have a configurable
// threshold and is configured based on the outcome of performance measurements.
//
// isys has 3 ports with IOSF interface for VC0, VC1 and VC2
// psys has 4 ports with IOSF interface for VC0, VC1w, VC1r and VC2
//
// Threshold values are pre-defined and are arrived at after performance
// evaluations on a type of IPU6
//
pub const IPU6_MAX_VC_IOSF_PORTS: c_int = 4;
//
// IPU6 must configure correct arbitration mechanism related to the IOSF VC
// requests. There are two options per VC0 and VC1 - > 0 means rearbitrate on
// stall and 1 means stall until the request is completed.
//
pub const IPU6_BTRS_ARB_MODE_TYPE_REARB: c_int = 0;
pub const IPU6_BTRS_ARB_MODE_TYPE_STALL: c_int = 1;
// Currently chosen arbitration mechanism for VC0

// Currently chosen arbitration mechanism for VC1

//
// MMU Invalidation HW bug workaround by ZLW mechanism
//
// Old IPU6 MMUV2 has a bug in the invalidation mechanism which might result in
// wrong translation or replication of the translation. This will cause data
// corruption. So we cannot directly use the MMU V2 invalidation registers
// to invalidate the MMU. Instead, whenever an invalidate is called, we need to
// clear the TLB by evicting all the valid translations by filling it with trash
// buffer (which is guaranteed not to be used by any other processes). ZLW is
// used to fill the L1 and L2 caches with the trash buffer translations. ZLW
// or Zero length write, is pre-fetch mechanism to pre-fetch the pages in
// advance to the L1 and L2 caches without triggering any memory operations.
//
// In MMU V2, L1 -> 16 streams and 64 blocks, maximum 16 blocks per stream
// One L1 block has 16 entries, hence points to 16 * 4K pages
// L2 -> 16 streams and 32 blocks. 2 blocks per streams
// One L2 block maps to 1024 L1 entries, hence points to 4MB address range
// 2 blocks per L2 stream means, 1 stream points to 8MB range
//
// As we need to clear the caches and 8MB being the biggest cache size, we need
// to have trash buffer which points to 8MB address range. As these trash
// buffers are not used for any memory transactions, we need only the least
// amount of physical memory. So we reserve 8MB IOVA address range but only
// one page is reserved from physical memory. Each of this 8MB IOVA address
// range is then mapped to the same physical memory page.
//
// One L2 entry maps 1024 L1 entries and one L1 entry per page

// Max L2 blocks per stream
pub const IPU6_MMUV2_MAX_L2_BLOCKS: c_int = 2;
// Max L1 blocks per stream
pub const IPU6_MMUV2_MAX_L1_BLOCKS: c_int = 16;

// Entries per L1 block
pub const MMUV2_ENTRIES_PER_L1_BLOCK: c_int = 16;

//
// In some of the IPU6 MMUs, there is provision to configure L1 and L2 page
// table caches. Both these L1 and L2 caches are divided into multiple sections
// called streams. There is maximum 16 streams for both caches. Each of these
// sections are subdivided into multiple blocks. When nr_l1streams = 0 and
// nr_l2streams = 0, means the MMU is of type MMU_V1 and do not support
// L1/L2 page table caches.
//
// L1 stream per block sizes are configurable and varies per usecase.
// L2 has constant block sizes - 2 blocks per stream.
//
// MMU1 support pre-fetching of the pages to have less cache lookup misses. To
// enable the pre-fetching, MMU1 AT (Address Translator) device registers
// need to be configured.
//
// There are four types of memory accesses which requires ZLW configuration.
// ZLW(Zero Length Write) is a mechanism to enable VT-d pre-fetching on IOMMU.
//
// 1. Sequential Access or 1D mode
// Set ZLW_EN -> 1
// set ZLW_PAGE_CROSS_1D -> 1
// Set ZLW_N to "N" pages so that ZLW will be inserte N pages ahead where
// N is pre-defined and hardcoded in the platform data
// Set ZLW_2D -> 0
//
// 2. ZLW 2D mode
// Set ZLW_EN -> 1
// set ZLW_PAGE_CROSS_1D -> 1,
// Set ZLW_N -> 0
// Set ZLW_2D -> 1
//
// 3. ZLW Enable (no 1D or 2D mode)
// Set ZLW_EN -> 1
// set ZLW_PAGE_CROSS_1D -> 0,
// Set ZLW_N -> 0
// Set ZLW_2D -> 0
//
// 4. ZLW disable
// Set ZLW_EN -> 0
// set ZLW_PAGE_CROSS_1D -> 0,
// Set ZLW_N -> 0
// Set ZLW_2D -> 0
//
// To configure the ZLW for the above memory access, four registers are
// available. Hence to track these four settings, we have the following entries
// in the struct ipu6_mmu_hw. Each of these entries are per stream and
// available only for the L1 streams.
//
// a. l1_zlw_en -> To track zlw enabled per stream (ZLW_EN)
// b. l1_zlw_1d_mode -> Track 1D mode per stream. ZLW inserted at page boundary
// c. l1_ins_zlw_ahead_pages -> to track how advance the ZLW need to be inserted
// Insert ZLW request N pages ahead address.
// d. l1_zlw_2d_mode -> To track 2D mode per stream (ZLW_2D)
//
// Currently L1/L2 streams, blocks, AT ZLW configurations etc. are pre-defined
// as per the usecase specific calculations. Any change to this pre-defined
// table has to happen in sync with IPU6 FW.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_mmu_hw {
    pub offset: c_ulong,
    pub base: *mut void __iomem,
}

//
// L1 has variable blocks per stream - total of 64 blocks and maximum of
// 16 blocks per stream. Configurable by using the block start address
// per stream. Block start address is calculated from the block size
//
// Is ZLW is enabled in each stream
//
// L2 has fixed 2 blocks per stream. Block address is calculated
// from the block size
//
// flag to track if WA is needed for successive invalidate HW bug
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_mmu_pdata {
    pub nr_mmus: u32,
    pub mmu_hw: [ipu6_mmu_hw; IPU6_MMU_MAX_DEVICES],
    pub mmid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_csi2_pdata {
    pub base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_internal_csi2_pdata {
    pub nports: u32,
    pub irq_mask: u32,
    pub ctrl0_irq_edge: u32,
    pub ctrl0_irq_clear: u32,
    pub ctrl0_irq_mask: u32,
    pub ctrl0_irq_enable: u32,
    pub ctrl0_irq_lnp: u32,
    pub ctrl0_irq_status: u32,
    pub fw_access_port_ofs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_internal_tpg_pdata {
    pub ntpgs: u32,
    pub offsets: *mut u32,
    pub sels: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_hw_variants {
    pub offset: c_ulong,
    pub nr_mmus: u32,
    pub mmu_hw: [ipu6_mmu_hw; IPU6_MMU_MAX_DEVICES],
    pub cdc_fifos: u8,
    pub cdc_fifo_threshold: [u8; IPU6_MAX_VC_IOSF_PORTS],
    pub dmem_offset: u32,
    pub spc_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_internal_pdata {
    pub csi2: ipu6_isys_internal_csi2_pdata,
    pub hw_variant: ipu6_hw_variants,
    pub num_parallel_streams: u32,
    pub isys_dma_overshoot: u32,
    pub sram_gran_shift: u32,
    pub sram_gran_size: u32,
    pub max_sram_size: u32,
    pub max_streams: u32,
    pub max_send_queues: u32,
    pub max_sram_blocks: u32,
    pub max_devq_size: u32,
    pub sensor_type_start: u32,
    pub sensor_type_end: u32,
    pub ltr: u32,
    pub memopen_threshold: u32,
    pub enhanced_iwake: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_pdata {
    pub base: *mut void __iomem,
    pub ipdata: *const ipu6_isys_internal_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_psys_internal_pdata {
    pub hw_variant: ipu6_hw_variants,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_psys_pdata {
    pub base: *mut void __iomem,
    pub ipdata: *const ipu6_psys_internal_pdata,
}

extern "C" {
    pub fn ipu6_fw_authenticate(data: *mut c_void, val: u64) -> c_int;
}
