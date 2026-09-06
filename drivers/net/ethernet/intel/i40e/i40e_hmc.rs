//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_hmc.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

pub const I40E_HMC_MAX_BP_COUNT: c_int = 512;
// forward-declare the HW struct for the compiler
pub const I40E_HMC_INFO_SIGNATURE: c_uint = 0x484D5347 /* HMSG */;
pub const I40E_HMC_PD_CNT_IN_SD: c_int = 512;
pub const I40E_HMC_DIRECT_BP_SIZE: c_uint = 0x200000 /* 2M */;
pub const I40E_HMC_PAGED_BP_SIZE: c_int = 4096;
pub const I40E_HMC_PD_BP_BUF_ALIGNMENT: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_obj_info {
    pub /: *mut *mut u64 base; / base addr in FPM,
    pub /: *mut *mut u32 max_cnt; / max count available for this hmc func,
    pub /: *mut *mut u32 cnt; / count of objects driver actually wants to create,
    pub /: *mut *mut u64 size; / size in bytes of one object,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_sd_entry_type {
    I40E_SD_TYPE_INVALID = 0,
    I40E_SD_TYPE_PAGED   = 1,
    I40E_SD_TYPE_DIRECT  = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_bp {
    pub entry_type: i40e_sd_entry_type,
    pub /: *mut *mut i40e_dma_mem addr; / populate to be used by hw,
    pub sd_pd_index: u32,
    pub ref_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_pd_entry {
    pub bp: i40e_hmc_bp,
    pub sd_index: u32,
    pub rsrc_pg: bool,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_pd_table {
    pub /: *mut *mut i40e_dma_mem pd_page_addr; / populate to be used by hw,
    pub /: *mut *mut *mut i40e_hmc_pd_entry pd_entry; / [512] for sw book keeping,
    pub /: *mut *mut i40e_virt_mem pd_entry_virt_mem; / virt mem for pd_entry,
    pub ref_cnt: u32,
    pub sd_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_sd_entry {
    pub entry_type: i40e_sd_entry_type,
    pub valid: bool,
    pub pd_table: i40e_hmc_pd_table,
    pub bp: i40e_hmc_bp,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_sd_table {
    pub /: *mut *mut i40e_virt_mem addr; / used to track sd_entry allocations,
    pub sd_cnt: u32,
    pub ref_cnt: u32,
    pub /: *mut *mut *mut *mut i40e_hmc_sd_entry sd_entry; / (sd_cnt512) entries max,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_hmc_info {
    pub signature: u32,
// equals to pci func num for PF and dynamically allocated for VFs
    pub hmc_fn_id: u8,
    pub /: *mut *mut u16 first_sd_index; / index of the first available SD,
// hmc objects
    pub hmc_obj: *mut i40e_hmc_obj_info,
    pub hmc_obj_virt_mem: i40e_virt_mem,
    pub sd_table: i40e_hmc_sd_table,
}

//
// I40E_SET_PF_SD_ENTRY - marks the sd entry as valid in the hardware
// @hw: pointer to our hw struct
// @pa: pointer to physical address
// @sd_index: segment descriptor index
// @type: if sd entry is direct or paged
//

//
// I40E_CLEAR_PF_SD_ENTRY - marks the sd entry as invalid in the hardware
// @hw: pointer to our hw struct
// @sd_index: segment descriptor index
// @type: if sd entry is direct or paged
//

//
// I40E_INVALIDATE_PF_HMC_PD - Invalidates the pd cache in the hardware
// @hw: pointer to our hw struct
// @sd_idx: segment descriptor index
// @pd_idx: page descriptor index
//

//
// I40E_FIND_SD_INDEX_LIMIT - finds segment descriptor index limit
// @hmc_info: pointer to the HMC configuration information structure
// @type: type of HMC resources we're searching
// @index: starting index for the object
// @cnt: number of objects we're trying to create
// @sd_idx: pointer to return index of the segment descriptor in question
// @sd_limit: pointer to return the maximum number of segment descriptors
//
// This function calculates the segment descriptor index and index limit
// for the resource defined by i40e_hmc_rsrc_type.
//

// (sd_idx) = (u32)(fpm_addr / I40E_HMC_DIRECT_BP_SIZE);		\
// (sd_limit) = (u32)((fpm_limit - 1) / I40E_HMC_DIRECT_BP_SIZE);	\
// add one more to the limit to correct our range */		\
// (sd_limit) += 1;						\
//
// I40E_FIND_PD_INDEX_LIMIT - finds page descriptor index limit
// @hmc_info: pointer to the HMC configuration information struct
// @type: HMC resource type we're examining
// @idx: starting index for the object
// @cnt: number of objects we're trying to create
// @pd_index: pointer to return page descriptor index
// @pd_limit: pointer to return page descriptor index limit
//
// Calculates the page descriptor index and index limit for the resource
// defined by i40e_hmc_rsrc_type.
//

// (pd_index) = (u32)(fpm_adr / I40E_HMC_PAGED_BP_SIZE);		\
// (pd_limit) = (u32)((fpm_limit - 1) / I40E_HMC_PAGED_BP_SIZE);	\
// add one more to the limit to correct our range */		\
// (pd_limit) += 1;						\
