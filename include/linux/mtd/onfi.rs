//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/onfi.h
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
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
// Steven J. Hill <sjhill@realitydiluted.com>
// Thomas Gleixner <tglx@kernel.org>
//
// Contains all ONFI related definitions
//

// ONFI version bits

// ONFI features

// ONFI timing mode, used in both asynchronous and synchronous mode
pub const ONFI_DATA_INTERFACE_SDR: c_int = 0;

// ONFI feature number/address
pub const ONFI_FEATURE_NUMBER: c_int = 256;
pub const ONFI_FEATURE_ADDR_TIMING_MODE: c_uint = 0x1;
// Vendor-specific feature address (Micron)
pub const ONFI_FEATURE_ADDR_READ_RETRY: c_uint = 0x89;
pub const ONFI_FEATURE_ON_DIE_ECC: c_uint = 0x90;

// ONFI subfeature parameters length
pub const ONFI_SUBFEATURE_PARAM_LEN: c_int = 4;
// ONFI optional commands SET/GET FEATURES supported?

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_onfi_params {
// rev info and features block
// 'O' 'N' 'F' 'I'
    pub sig: [u8; 4],
    pub revision: __le16,
    pub features: __le16,
    pub opt_cmd: __le16,
    pub reserved0: [u8; 2],
    pub /: *mut *mut __le16 ext_param_page_length; / since ONFI 2.1,
    pub /: *mut *mut u8 num_of_param_pages; / since ONFI 2.1,
    pub reserved1: [u8; 17],
// manufacturer information block
    pub manufacturer: [c_char; 12],
    pub model: [c_char; 20],
    pub jedec_id: u8,
    pub date_code: __le16,
    pub reserved2: [u8; 13],
// memory organization block
    pub byte_per_page: __le32,
    pub spare_bytes_per_page: __le16,
    pub data_bytes_per_ppage: __le32,
    pub spare_bytes_per_ppage: __le16,
    pub pages_per_block: __le32,
    pub blocks_per_lun: __le32,
    pub lun_count: u8,
    pub addr_cycles: u8,
    pub bits_per_cell: u8,
    pub bb_per_lun: __le16,
    pub block_endurance: __le16,
    pub guaranteed_good_blocks: u8,
    pub guaranteed_block_endurance: __le16,
    pub programs_per_page: u8,
    pub ppage_attr: u8,
    pub ecc_bits: u8,
    pub interleaved_bits: u8,
    pub interleaved_ops: u8,
    pub reserved3: [u8; 13],
// electrical parameter block
    pub io_pin_capacitance_max: u8,
    pub sdr_timing_modes: __le16,
    pub program_cache_timing_mode: __le16,
    pub t_prog: __le16,
    pub t_bers: __le16,
    pub t_r: __le16,
    pub t_ccs: __le16,
    pub nvddr_timing_modes: u8,
    pub nvddr2_timing_modes: u8,
    pub nvddr_nvddr2_features: u8,
    pub clk_pin_capacitance_typ: __le16,
    pub io_pin_capacitance_typ: __le16,
    pub input_pin_capacitance_typ: __le16,
    pub input_pin_capacitance_max: u8,
    pub driver_strength_support: u8,
    pub t_int_r: __le16,
    pub t_adl: __le16,
    pub reserved4: [u8; 8],
// vendor
    pub vendor_revision: __le16,
    pub vendor: [u8; 88],
    pub crc: __le16,
    pub __packed: },
pub const ONFI_CRC_BASE: c_uint = 0x4F4E;
// Extended ECC information Block Definition (since ONFI 2.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onfi_ext_ecc_info {
    pub ecc_bits: u8,
    pub codeword_size: u8,
    pub bb_per_lun: __le16,
    pub block_endurance: __le16,
    pub reserved: [u8; 2],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct onfi_ext_section {
    pub type: u8,
    pub length: u8,
    pub __packed: },
pub const ONFI_EXT_SECTION_MAX: c_int = 8;
// Extended Parameter Page Definition (since ONFI 2.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onfi_ext_param_page {
    pub crc: __le16,
    pub /: *mut *mut u8 sig[4]; / 'E' 'P' 'P' 'S',
    pub reserved0: [u8; 10],
    pub sections: [onfi_ext_section; ONFI_EXT_SECTION_MAX],
//
// The actual size of the Extended Parameter Page is in
// @ext_param_page_length of nand_onfi_params{}.
// The following are the variable length sections.
// So we do not add any fields below. Please see the ONFI spec.
//
    pub __packed: },
//
// struct onfi_params - ONFI specific parameters that will be reused
// @version: ONFI version (BCD encoded), 0 if ONFI is not supported
// @tPROG: Page program time
// @tBERS: Block erase time
// @tR: Page read time
// @tCCS: Change column setup time
// @fast_tCAD: Command/Address/Data slow or fast delay (NV-DDR only)
// @sdr_timing_modes: Supported asynchronous/SDR timing modes
// @nvddr_timing_modes: Supported source synchronous/NV-DDR timing modes
// @vendor_revision: Vendor specific revision number
// @vendor: Vendor specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onfi_params {
    pub version: c_int,
    pub tPROG: u16,
    pub tBERS: u16,
    pub tR: u16,
    pub tCCS: u16,
    pub fast_tCAD: bool,
    pub sdr_timing_modes: u16,
    pub nvddr_timing_modes: u16,
    pub vendor_revision: u16,
    pub vendor: [u8; 88],
}
