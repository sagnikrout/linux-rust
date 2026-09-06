//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ras_umc.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const UMC_VRAM_TYPE_UNKNOWN: c_int = 0;
pub const UMC_VRAM_TYPE_GDDR1: c_int = 1;
pub const UMC_VRAM_TYPE_DDR2: c_int = 2;
pub const UMC_VRAM_TYPE_GDDR3: c_int = 3;
pub const UMC_VRAM_TYPE_GDDR4: c_int = 4;
pub const UMC_VRAM_TYPE_GDDR5: c_int = 5;
pub const UMC_VRAM_TYPE_HBM: c_int = 6;
pub const UMC_VRAM_TYPE_DDR3: c_int = 7;
pub const UMC_VRAM_TYPE_DDR4: c_int = 8;
pub const UMC_VRAM_TYPE_GDDR6: c_int = 9;
pub const UMC_VRAM_TYPE_DDR5: c_int = 10;
pub const UMC_VRAM_TYPE_LPDDR4: c_int = 11;
pub const UMC_VRAM_TYPE_LPDDR5: c_int = 12;
pub const UMC_VRAM_TYPE_HBM3E: c_int = 13;
pub const UMC_ECC_NEW_DETECTED_TAG: c_uint = 0x1;

// invalid node instance value
pub const UMC_INV_AID_NODE: c_uint = 0xffff;
//
// a flag to indicate v2 format channel index stored in eeprom
//
// v1: store channel index within a umc instance in eeprom
// range in UMC v12: 0 ~ 7
// v2: store global channel index in eeprom
// range in UMC v12: 0 ~ 127
//
// NOTE: it's better to store it in eeprom_table_record.mem_channel,
// but there is only 8 bits in mem_channel, and the channel number may
// increase in the future, we decide to save it in
// eeprom_table_record.retired_page. retired_page is useless in v2,
// we depend on eeprom_table_record.address instead of retired_page in v2.
// Only 48 bits are saved on eeprom, use bit 47 here.
//
// UMC_CHANNEL_IDX_V2 is replaced by nps value in cur_nps_retired_row_pfn
// in eeprom v3 format, so they have no conflict.
//

// three column bits and one row bit in MCA address flip
// in bad page retirement
//
pub const UMC_PA_FLIP_BITS_NUM: c_int = 4;
// bits [63:58] of pa carry the nps mode for RAS_TA_PA_TO_MCA,
// RAS TA will use it to get the nps related to pa
//
pub const UMC_PA_NPS_SHIFT: c_int = 58;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum umc_memory_partition_mode {
    UMC_MEMORY_PARTITION_MODE_NONE = 0,
    UMC_MEMORY_PARTITION_MODE_NPS1 = 1,
    UMC_MEMORY_PARTITION_MODE_NPS2 = 2,
    UMC_MEMORY_PARTITION_MODE_NPS3 = 3,
    UMC_MEMORY_PARTITION_MODE_NPS4 = 4,
    UMC_MEMORY_PARTITION_MODE_NPS6 = 6,
    UMC_MEMORY_PARTITION_MODE_NPS8 = 8,
    UMC_MEMORY_PARTITION_MODE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umc_flip_bits {
    pub flip_bits_in_pa: [u32; UMC_PA_FLIP_BITS_NUM],
    pub flip_row_bit: u32,
    pub r13_in_pa: u32,
    pub bit_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umc_mca_addr {
    pub err_addr: u64,
    pub ch_inst: u32,
    pub umc_inst: u32,
    pub node_inst: u32,
    pub socket_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umc_phy_addr {
    pub pa: u64,
    pub bank: u32,
    pub channel_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umc_bank_addr {
    pub /: *mut *mut uint32_t stack_id; / SID,
    pub bank_group: u32,
    pub bank: u32,
    pub row: u32,
    pub column: u32,
    pub channel: u32,
    pub /: *mut *mut uint32_t subchannel; / Also called Pseudochannel (PC),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_umc_ip_func {
    pub record): *mut *mut ras_bank_ecc bank, eeprom_umc_record,
    pub num): *mut *mut uint64_t pfns, uint32_t,
    pub soc_pa): *mut umc_bank_addr bank_addr, uint64_t,
    pub bank_addr): *mut uint64_t soc_pa, struct umc_bank_addr,
    pub sid): *mut *mut *mut *mut uint32_t did, uint32_t ch, uint32_t umc_inst, uint32_t,
    pub nps): u32,
    pub zero_pfn_ok): uint64_t pa, enum umc_memory_partition_mode nps, bool,
    pub pa): *mut *mut uint32_t (get_die_id)(uint64_t mca_addr, uint64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_store_record {
// point to data records array
    pub bps: *mut eeprom_umc_record,
// the count of entries
    pub count: c_int,
// the space can place new entries
    pub space_left: c_int,
// logical bad page number
    pub bad_page_num: c_int,
// the bad page number is ras_num_recs or
// ras_num_recs * retire_unit
//
    pub bad_page_num_old: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_umc_err_data {
    pub rom_data: eeprom_store_record,
    pub ram_data: eeprom_store_record,
    pub umc_nps_mode: umc_memory_partition_mode,
    pub last_retired_pfn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_umc {
    pub umc_ip_version: u32,
    pub umc_vram_type: u32,
    pub num_umc: u32,
    pub ip_func: *const ras_umc_ip_func,
    pub root: radix_tree_root,
    pub tree_lock: mutex,
    pub umc_lock: mutex,
    pub bank_log_lock: mutex,
    pub pending_ecc_lock: mutex,
    pub umc_err_data: ras_umc_err_data,
    pub pending_ecc_list: list_head,
// number of entries currently queued on pending_ecc_list
    pub pending_ecc_count: u32,
// number of entries dropped because pending_ecc_list was full
    pub pending_ecc_dropped: u32,
}

//
// Upper bound on entries that can be queued on pending_ecc_list while a
// GPU reset is in progress. Beyond this, new ECC events are dropped to
// prevent unbounded kernel memory growth in case of an ECC storm or
// malicious/repeated UMC error injection.
//
pub const RAS_UMC_PENDING_ECC_MAX: c_int = 8192;
extern "C" {
    pub fn ras_umc_sw_init(ras: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_sw_fini(ras: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_hw_init(ras: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_hw_fini(ras: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_handle_bad_pages(ras_core: *mut ras_core_context, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ras_umc_log_bad_bank(ras: *mut ras_core_context, bank: *mut ras_bank_ecc) -> c_int;
}
extern "C" {
    pub fn ras_umc_log_bad_bank_pending(ras_core: *mut ras_core_context, bank: *mut ras_bank_ecc) -> c_int;
}
extern "C" {
    pub fn ras_umc_log_pending_bad_bank(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_clear_logged_ecc(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_load_bad_pages(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_get_saved_eeprom_count(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_clean_badpage_data(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_get_badpage_count(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_get_badpage_record(ras_core: *mut ras_core_context, index: u32, record: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ras_umc_check_retired_addr(ras_core: *mut ras_core_context, addr: u64) -> bool;
}
extern "C" {
    pub fn ras_umc_bit_wise_xor(val: u32) -> u32;
}
