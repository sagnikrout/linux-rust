//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/hmc.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2020 Intel Corporation

pub const IRDMA_HMC_MAX_BP_COUNT: c_int = 512;
pub const IRDMA_MAX_SD_ENTRIES: c_int = 11;
pub const IRDMA_HW_DBG_HMC_INVALID_BP_MARK: c_uint = 0xca;
pub const IRDMA_HMC_INFO_SIGNATURE: c_uint = 0x484d5347;
pub const IRDMA_HMC_PD_CNT_IN_SD: c_int = 512;
pub const IRDMA_HMC_DIRECT_BP_SIZE: c_uint = 0x200000;
pub const IRDMA_HMC_MAX_SD_COUNT: c_int = 8192;
pub const IRDMA_HMC_PAGED_BP_SIZE: c_int = 4096;
pub const IRDMA_HMC_PD_BP_BUF_ALIGNMENT: c_int = 4096;
pub const IRDMA_FIRST_VF_FPM_ID: c_int = 8;
pub const FPM_MULTIPLIER: c_int = 1024;
pub const IRDMA_OBJ_LOC_MEM_BIT: c_uint = 0x4;
pub const IRDMA_XF_MULTIPLIER: c_int = 16;
pub const IRDMA_RRF_MULTIPLIER: c_int = 8;
pub const IRDMA_MIN_PBLE_PAGES: c_int = 3;
pub const IRDMA_HMC_PAGE_SIZE: c_int = 2097152;
pub const IRDMA_MIN_MR_PER_QP: c_int = 4;
pub const IRDMA_MIN_QP_CNT: c_int = 64;
pub const IRDMA_FSIAV_CNT_MAX: c_int = 1048576;
pub const IRDMA_MIN_IRD: c_int = 8;
pub const IRDMA_HMC_MIN_RRF: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_hmc_rsrc_type {
    IRDMA_HMC_IW_QP		 = 0,
    IRDMA_HMC_IW_CQ		 = 1,
    IRDMA_HMC_IW_SRQ	 = 2,
    IRDMA_HMC_IW_HTE	 = 3,
    IRDMA_HMC_IW_ARP	 = 4,
    IRDMA_HMC_IW_APBVT_ENTRY = 5,
    IRDMA_HMC_IW_MR		 = 6,
    IRDMA_HMC_IW_XF		 = 7,
    IRDMA_HMC_IW_XFFL	 = 8,
    IRDMA_HMC_IW_Q1		 = 9,
    IRDMA_HMC_IW_Q1FL	 = 10,
    IRDMA_HMC_IW_TIMER       = 11,
    IRDMA_HMC_IW_FSIMC       = 12,
    IRDMA_HMC_IW_FSIAV       = 13,
    IRDMA_HMC_IW_PBLE	 = 14,
    IRDMA_HMC_IW_RRF	 = 15,
    IRDMA_HMC_IW_RRFFL       = 16,
    IRDMA_HMC_IW_HDR	 = 17,
    IRDMA_HMC_IW_MD		 = 18,
    IRDMA_HMC_IW_OOISC       = 19,
    IRDMA_HMC_IW_OOISCFFL    = 20,
    IRDMA_HMC_IW_MAX, /* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_sd_entry_type {
    IRDMA_SD_TYPE_INVALID = 0,
    IRDMA_SD_TYPE_PAGED   = 1,
    IRDMA_SD_TYPE_DIRECT  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_hmc_obj_mem {
    IRDMA_HOST_MEM = 0,
    IRDMA_LOC_MEM  = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_obj_info {
    pub base: u64,
    pub max_cnt: u32,
    pub cnt: u32,
    pub size: u64,
    pub mem_loc: irdma_hmc_obj_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_bp {
    pub entry_type: irdma_sd_entry_type,
    pub addr: irdma_dma_mem,
    pub sd_pd_index: u32,
    pub use_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_pd_entry {
    pub bp: irdma_hmc_bp,
    pub sd_index: u32,
    pub rsrc_pg:1: bool,
    pub valid:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_pd_table {
    pub pd_page_addr: irdma_dma_mem,
    pub pd_entry: *mut irdma_hmc_pd_entry,
    pub pd_entry_virt_mem: irdma_virt_mem,
    pub use_cnt: u32,
    pub sd_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_sd_entry {
    pub entry_type: irdma_sd_entry_type,
    pub valid: bool,
    pub pd_table: irdma_hmc_pd_table,
    pub bp: irdma_hmc_bp,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_sd_table {
    pub addr: irdma_virt_mem,
    pub sd_cnt: u32,
    pub use_cnt: u32,
    pub sd_entry: *mut irdma_hmc_sd_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_info {
    pub signature: u32,
    pub hmc_fn_id: u8,
    pub first_sd_index: u16,
    pub hmc_obj: *mut irdma_hmc_obj_info,
    pub hmc_obj_virt_mem: irdma_virt_mem,
    pub sd_table: irdma_hmc_sd_table,
    pub sd_indexes: [u16; IRDMA_HMC_MAX_SD_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_update_sd_entry {
    pub cmd: u64,
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_update_sds_info {
    pub cnt: u32,
    pub hmc_fn_id: u8,
    pub entry: [irdma_update_sd_entry; IRDMA_MAX_SD_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_fcn_info {
    pub vf_id: u32,
    pub protocol_used: u8,
    pub free_fcn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_create_obj_info {
    pub hmc_info: *mut irdma_hmc_info,
    pub add_sd_virt_mem: irdma_virt_mem,
    pub rsrc_type: u32,
    pub start_idx: u32,
    pub count: u32,
    pub add_sd_cnt: u32,
    pub entry_type: irdma_sd_entry_type,
    pub privileged: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_del_obj_info {
    pub hmc_info: *mut irdma_hmc_info,
    pub del_sd_virt_mem: irdma_virt_mem,
    pub rsrc_type: u32,
    pub start_idx: u32,
    pub count: u32,
    pub del_sd_cnt: u32,
    pub privileged: bool,
}

extern "C" {
    pub fn irdma_prep_remove_sd_bp(hmc_info: *mut irdma_hmc_info, idx: u32) -> c_int;
}
extern "C" {
    pub fn irdma_prep_remove_pd_page(hmc_info: *mut irdma_hmc_info, idx: u32) -> c_int;
}
