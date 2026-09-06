//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/pble.h
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
// Copyright (c) 2015 - 2019 Intel Corporation
pub const PBLE_SHIFT: c_int = 6;
pub const PBLE_PER_PAGE: c_int = 512;
pub const HMC_PAGED_BP_SHIFT: c_int = 12;
pub const PBLE_512_SHIFT: c_int = 9;
pub const PBLE_INVALID_IDX: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_pble_level {
    PBLE_LEVEL_0 = 0,
    PBLE_LEVEL_1 = 1,
    PBLE_LEVEL_2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_alloc_type {
    PBLE_NO_ALLOC	  = 0,
    PBLE_SD_CONTIGOUS = 1,
    PBLE_SD_PAGED	  = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pble_chunkinfo {
    pub pchunk: *mut irdma_chunk,
    pub bit_idx: u64,
    pub bits_used: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pble_info {
    pub addr: *mut u64,
    pub idx: u32,
    pub cnt: u32,
    pub chunkinfo: irdma_pble_chunkinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pble_level2 {
    pub root: irdma_pble_info,
    pub leaf: *mut irdma_pble_info,
    pub leafmem: irdma_virt_mem,
    pub leaf_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pble_alloc {
    pub total_cnt: u32,
    pub level: irdma_pble_level,
    pub level1: irdma_pble_info,
    pub level2: irdma_pble_level2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_pd_idx {
    pub sd_idx: u32,
    pub pd_idx: u32,
    pub rel_pd_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_add_page_info {
    pub chunk: *mut irdma_chunk,
    pub sd_entry: *mut irdma_hmc_sd_entry,
    pub hmc_info: *mut irdma_hmc_info,
    pub idx: sd_pd_idx,
    pub pages: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_chunk {
    pub list: list_head,
    pub dmainfo: irdma_dma_info,
    pub bitmapbuf: *mut c_ulong,
    pub sizeofbitmap: u32,
    pub size: u64,
    pub vaddr: *mut c_void,
    pub fpm_addr: u64,
    pub pg_cnt: u32,
    pub type: irdma_alloc_type,
    pub dev: *mut irdma_sc_dev,
    pub chunkmem: irdma_virt_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pble_prm {
    pub clist: list_head,
    pub /: *mut *mut spinlock_t prm_lock; / protect prm bitmap,
    pub total_pble_alloc: u64,
    pub free_pble_cnt: u64,
    pub pble_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_pble_rsrc {
    pub unallocated_pble: u32,
    pub /: *mut *mut mutex pble_mutex_lock; / protect PBLE resource,
    pub dev: *mut irdma_sc_dev,
    pub fpm_base_addr: u64,
    pub next_fpm_addr: u64,
    pub pinfo: irdma_pble_prm,
    pub allocdpbles: u64,
    pub freedpbles: u64,
    pub stats_direct_sds: u32,
    pub stats_paged_sds: u32,
    pub stats_alloc_ok: u64,
    pub stats_alloc_fail: u64,
    pub stats_alloc_freed: u64,
    pub stats_lvl1: u64,
    pub stats_lvl2: u64,
}

extern "C" {
    pub fn irdma_destroy_pble_prm(pble_rsrc: *mut irdma_hmc_pble_rsrc);
}
extern "C" {
    pub fn irdma_pble_free_paged_mem(chunk: *mut irdma_chunk);
}
extern "C" {
    pub fn irdma_pble_get_paged_mem(chunk: *mut irdma_chunk, pg_cnt: u32) -> c_int;
}
extern "C" {
    pub fn irdma_prm_rem_bitmapmem(hw: *mut irdma_hw, chunk: *mut irdma_chunk);
}
