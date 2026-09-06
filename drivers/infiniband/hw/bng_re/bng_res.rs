//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bng_re/bng_res.h
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
// Copyright (c) 2025 Broadcom.

pub const BNG_ROCE_FW_MAX_TIMEOUT: c_int = 60;

pub const MAX_PBL_LVL_0_PGS: c_int = 1;
pub const MAX_PBL_LVL_1_PGS: c_int = 512;
pub const MAX_PBL_LVL_1_PGS_SHIFT: c_int = 9;
pub const MAX_PBL_LVL_1_PGS_FOR_LVL_2: c_int = 256;

pub const MAX_PDL_LVL_SHIFT: c_int = 9;

pub const BNG_RE_DBR_EPOCH_SHIFT: c_int = 24;
pub const BNG_RE_DBR_TOGGLE_SHIFT: c_int = 25;
pub const BNG_MAX_TQM_ALLOC_REQ: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_reg_desc {
    pub bar_id: u8,
    pub bar_base: resource_size_t,
    pub offset: c_ulong,
    pub bar_reg: *mut void __iomem,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_db_info {
    pub db: *mut void __iomem,
    pub priv_db: *mut void __iomem,
    pub hwq: *mut bng_re_hwq,
    pub xid: u32,
    pub max_slot: u32,
    pub flags: u32,
    pub toggle: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bng_re_db_info_flags_mask {
    BNG_RE_FLAG_EPOCH_CONS_SHIFT        = 0x0UL,
    BNG_RE_FLAG_EPOCH_PROD_SHIFT        = 0x1UL,
    BNG_RE_FLAG_EPOCH_CONS_MASK         = 0x1UL,
    BNG_RE_FLAG_EPOCH_PROD_MASK         = 0x2UL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bng_re_db_epoch_flag_shift {
    BNG_RE_DB_EPOCH_CONS_SHIFT  = BNG_RE_DBR_EPOCH_SHIFT,
    BNG_RE_DB_EPOCH_PROD_SHIFT  = (BNG_RE_DBR_EPOCH_SHIFT - 1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_chip_ctx {
    pub chip_num: u16,
    pub hw_stats_size: u16,
    pub hwrm_intf_ver: u64,
    pub hwrm_cmd_max_timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_pbl {
    pub pg_count: u32,
    pub pg_size: u32,
    pub pg_arr: *mut c_void,
    pub pg_map_arr: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bng_re_pbl_lvl {
    BNG_PBL_LVL_0,
    BNG_PBL_LVL_1,
    BNG_PBL_LVL_2,
    BNG_PBL_LVL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bng_re_hwq_type {
    BNG_HWQ_TYPE_CTX,
    BNG_HWQ_TYPE_QUEUE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_sg_info {
    pub npages: u32,
    pub pgshft: u32,
    pub pgsize: u32,
    pub nopte: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_hwq_attr {
    pub res: *mut bng_re_res,
    pub sginfo: *mut bng_re_sg_info,
    pub type: bng_re_hwq_type,
    pub depth: u32,
    pub stride: u32,
    pub aux_stride: u32,
    pub aux_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_hwq {
    pub pdev: *mut pci_dev,
// lock to protect hwq
    pub lock: spinlock_t,
    pub 1]: bng_re_pbl pbl[BNG_PBL_LVL_MAX +,
// Valid values: 0, 1, 2
    pub level: bng_re_pbl_lvl,
// PBL entries
    pub pbl_ptr: *mut c_void,
// PBL  dma_addr
    pub pbl_dma_ptr: *mut dma_addr_t,
    pub max_elements: u32,
    pub depth: u32,
    pub element_size: u16,
    pub prod: u32,
    pub cons: u32,
// queue entry per page
    pub qe_ppg: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_stats {
    pub dma_map: dma_addr_t,
    pub dma: *mut c_void,
    pub size: u32,
    pub fw_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_res {
    pub pdev: *mut pci_dev,
    pub cctx: *mut bng_re_chip_ctx,
    pub dattr: *mut bng_re_dev_attr,
}

// pg = (u64)&hwq->pbl_ptr[pg_num];

// move cons and update toggle/epoch if wrap around
// cons += cnt;
// cons %= max_elements;
// dbinfo_flags ^= 1UL << BNG_RE_FLAG_EPOCH_CONS_SHIFT;
