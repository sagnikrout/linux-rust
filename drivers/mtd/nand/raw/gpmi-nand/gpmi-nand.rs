//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/gpmi-nand/gpmi-nand.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Freescale GPMI NAND Flash Driver
//
// Copyright (C) 2010-2011 Freescale Semiconductor, Inc.
// Copyright (C) 2008 Embedded Alley Solutions, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resources {
    pub gpmi_regs: *mut void __iomem,
    pub bch_regs: *mut void __iomem,
    pub dma_low_channel: c_uint,
    pub dma_high_channel: c_uint,
    pub clock: [*mut clk; GPMI_CLK_MAX],
}

//
// struct bch_geometry - BCH geometry description.
// @gf_len:                   The length of Galois Field. (e.g., 13 or 14)
// @ecc_strength:             A number that describes the strength of the ECC
// algorithm.
// @page_size:                The size, in bytes, of a physical page, including
// both data and OOB.
// @metadata_size:            The size, in bytes, of the metadata.
// @ecc0_chunk_size:          The size, in bytes, of a first ECC chunk.
// @eccn_chunk_size:          The size, in bytes, of a single ECC chunk after
// the first chunk in the page.
// @ecc_chunk_count:          The number of ECC chunks in the page,
// @payload_size:             The size, in bytes, of the payload buffer.
// @auxiliary_size:           The size, in bytes, of the auxiliary buffer.
// @auxiliary_status_offset:  The offset into the auxiliary buffer at which
// the ECC status appears.
// @block_mark_byte_offset:   The byte offset in the ECC-based page view at
// which the underlying physical block mark appears.
// @block_mark_bit_offset:    The bit offset into the ECC-based page view at
// which the underlying physical block mark appears.
// @ecc_for_meta:             The flag to indicate if there is a dedicate ecc
// for meta.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bch_geometry {
    pub gf_len: c_uint,
    pub ecc_strength: c_uint,
    pub page_size: c_uint,
    pub metadata_size: c_uint,
    pub ecc0_chunk_size: c_uint,
    pub eccn_chunk_size: c_uint,
    pub ecc_chunk_count: c_uint,
    pub payload_size: c_uint,
    pub auxiliary_size: c_uint,
    pub auxiliary_status_offset: c_uint,
    pub block_mark_byte_offset: c_uint,
    pub block_mark_bit_offset: c_uint,
    pub /: *mut *mut unsigned int ecc_for_meta; / ECC for meta data,
}

//
// struct boot_rom_geometry - Boot ROM geometry description.
// @stride_size_in_pages:        The size of a boot block stride, in pages.
// @search_area_stride_exponent: The logarithm to base 2 of the size of a
// search area in boot block strides.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_rom_geometry {
    pub stride_size_in_pages: c_uint,
    pub search_area_stride_exponent: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpmi_type {
    IS_MX23,
    IS_MX28,
    IS_MX6Q,
    IS_MX6SX,
    IS_MX7D,
    IS_MX8QXP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmi_devdata {
    pub type: gpmi_type,
    pub bch_max_ecc_strength: c_int,
    pub /: *mut *mut int max_chain_delay; / See the SDR EDO mode,
    pub clks: *const *const c_char,
    pub clks_count: c_int,
    pub support_edo_timing: bool,
}

//
// struct gpmi_nfc_hardware_timing - GPMI hardware timing parameters.
// @must_apply_timings:        Whether controller timings have already been
// applied or not (useful only while there is
// support for only one chip select)
// @clk_rate:                  The clock rate that must be used to derive the
// following parameters
// @timing0:                   HW_GPMI_TIMING0 register
// @timing1:                   HW_GPMI_TIMING1 register
// @ctrl1n:                    HW_GPMI_CTRL1n register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmi_nfc_hardware_timing {
    pub must_apply_timings: bool,
    pub clk_rate: unsigned long int,
    pub timing0: u32,
    pub timing1: u32,
    pub ctrl1n: u32,
}

pub const GPMI_MAX_TRANSFERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmi_transfer {
    pub cmdbuf: [u8; 8],
    pub sgl: scatterlist,
    pub direction: dma_data_direction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmi_nand_data {
// Devdata
    pub devdata: *const gpmi_devdata,
// System Interface
    pub dev: *mut device,
    pub pdev: *mut platform_device,
// Resources
    pub resources: resources,
// Flash Hardware
    pub hw: gpmi_nfc_hardware_timing,
// BCH
    pub bch_geometry: bch_geometry,
    pub bch_done: completion,
// NAND Boot issue
    pub swap_block_mark: bool,
    pub rom_geometry: boot_rom_geometry,
// MTD / NAND
    pub base: nand_controller,
    pub nand: nand_chip,
    pub transfers: [gpmi_transfer; GPMI_MAX_TRANSFERS],
    pub ntransfers: c_int,
    pub bch: bool,
    pub bch_flashlayout0: u32,
    pub bch_flashlayout1: u32,
    pub data_buffer_dma: *mut c_char,
    pub auxiliary_virt: *mut c_void,
    pub auxiliary_phys: dma_addr_t,
    pub raw_buffer: *mut c_void,
// DMA channels
pub const DMA_CHANS: c_int = 8;
    pub dma_chans: [*mut dma_chan; DMA_CHANS],
    pub dma_done: completion,

    pub dbg_root: *mut dentry,
    pub dbg_bch_geo: debugfs_blob_wrapper,
    pub raw_mode: bool,

}

// BCH : Status Block Completion Codes
pub const STATUS_GOOD: c_uint = 0x00;
pub const STATUS_ERASED: c_uint = 0xff;
pub const STATUS_UNCORRECTABLE: c_uint = 0xfe;
// Use the devdata to distinguish different Archs.

