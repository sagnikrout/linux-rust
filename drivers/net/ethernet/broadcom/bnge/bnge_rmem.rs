//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_rmem.h
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
// Copyright (c) 2025 Broadcom
pub const PTU_PTE_VALID: c_uint = 0x1UL;
pub const PTU_PTE_LAST: c_uint = 0x2UL;
pub const PTU_PTE_NEXT_TO_LAST: c_uint = 0x4UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ring_mem_info {
// Number of pages to next level
    pub nr_pages: c_int,
    pub page_size: c_int,
    pub flags: u16,
pub const BNGE_RMEM_VALID_PTE_FLAG: c_int = 1;
pub const BNGE_RMEM_RING_PTE_FLAG: c_int = 2;
pub const BNGE_RMEM_USE_FULL_PAGE_FLAG: c_int = 4;
    pub depth: u16,
    pub pg_arr: *mut c_void,
    pub dma_arr: *mut dma_addr_t,
    pub pg_tbl: *mut __le64,
    pub dma_pg_tbl: dma_addr_t,
    pub vmem_size: c_int,
    pub vmem: *mut c_void,
    pub ctx_mem: *mut bnge_ctx_mem_type,
}

// The hardware supports certain page sizes.
// Use the supported page sizes to allocate the rings.
//

pub const BNGE_PAGE_SHIFT: c_int = 12;

pub const BNGE_PAGE_SHIFT: c_int = 13;

pub const BNGE_PAGE_SHIFT: c_int = 16;

// The RXBD length is 16-bit so we can only support page sizes < 64K

pub const BNGE_RX_PAGE_SHIFT: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ctx_pg_info {
    pub entries: u32,
    pub nr_pages: u32,
    pub ctx_pg_arr: [*mut c_void; MAX_CTX_PAGES],
    pub ctx_dma_arr: [dma_addr_t; MAX_CTX_PAGES],
    pub ring_mem: bnge_ring_mem_info,
    pub ctx_pg_tbl: *mut bnge_ctx_pg_info,
}

pub const BNGE_MAX_TQM_SP_RINGS: c_int = 1;
pub const BNGE_MAX_TQM_FP_RINGS: c_int = 8;

pub const BNGE_BACKING_STORE_CFG_LEGACY_LEN: c_int = 256;

pub const BNGE_CTX_MRAV_AV_SPLIT_ENTRY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ctx_mem_type {
    pub type: u16,
    pub entry_size: u16,
    pub flags: u32,

    pub instance_bmap: u32,
    pub init_value: u8,
    pub entry_multiple: u8,
    pub init_offset: u16,
pub const BNGE_CTX_INIT_INVALID_OFFSET: c_uint = 0xffff;
    pub max_entries: u32,
    pub min_entries: u32,
    pub last:1: u8,
    pub split_entry_cnt: u8,
pub const BNGE_MAX_SPLIT_ENTRY: c_int = 4;
    pub qp_l2_entries: u32,
    pub qp_qp1_entries: u32,
    pub qp_fast_qpmd_entries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ctx_mem_info {
    pub tqm_fp_rings_count: u8,
    pub flags: u32,
pub const BNGE_CTX_FLAG_INITED: c_uint = 0x01;
    pub ctx_arr: [bnge_ctx_mem_type; BNGE_CTX_V2_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ring_struct {
    pub ring_mem: bnge_ring_mem_info,
    pub fw_ring_id: u32,
    pub grp_idx: u16,
    pub /: *mut *mut u16 map_idx; / Used by NQs,
}

extern "C" {
    pub fn bnge_alloc_ring(bd: *mut bnge_dev, rmem: *mut bnge_ring_mem_info) -> c_int;
}
extern "C" {
    pub fn bnge_free_ring(bd: *mut bnge_dev, rmem: *mut bnge_ring_mem_info);
}
extern "C" {
    pub fn bnge_alloc_ctx_mem(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_free_ctx_mem(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_init_ring_struct(bn: *mut bnge_net);
}
