//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bng_re/bng_re.h
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

pub const BNG_RE_MIN_MSIX: c_int = 2;

pub const BNG_RE_CREQ_NQ_IDX: c_int = 0;

// NQ specific structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_nq_db {
    pub reg: bng_re_reg_desc,
    pub dbinfo: bng_re_db_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_nq {
    pub pdev: *mut pci_dev,
    pub res: *mut bng_re_res,
    pub name: *mut c_char,
    pub hwq: bng_re_hwq,
    pub nq_db: bng_re_nq_db,
    pub ring_id: u16,
    pub msix_vec: c_int,
    pub mask: cpumask_t,
    pub nq_tasklet: tasklet_struct,
    pub requested: bool,
    pub budget: c_int,
    pub load: u32,
    pub cqn_wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_nq_record {
    pub msix_entries: [bnge_msix_info; BNG_RE_MAX_MSIX],
    pub nq: [bng_re_nq; BNG_RE_MAX_MSIX],
    pub num_msix: c_int,
// serialize NQ access
    pub load_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_en_dev_info {
    pub rdev: *mut bng_re_dev,
    pub auxr_dev: *mut bnge_auxr_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_ring_attr {
    pub dma_arr: *mut dma_addr_t,
    pub pages: c_int,
    pub type: c_int,
    pub depth: u32,
    pub /: *mut *mut u32 lrid; / Logical ring id,
    pub mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_dev {
    pub ibdev: ib_device,
    pub flags: c_ulong,
pub const BNG_RE_FLAG_NETDEV_REGISTERED: c_int = 0;
pub const BNG_RE_FLAG_RCFW_CHANNEL_EN: c_int = 1;
    pub netdev: *mut net_device,
    pub adev: *mut auxiliary_device,
    pub aux_dev: *mut bnge_auxr_dev,
    pub chip_ctx: *mut bng_re_chip_ctx,
    pub fn_id: c_int,
    pub bng_res: bng_re_res,
    pub rcfw: bng_re_rcfw,
    pub nqr: *mut bng_re_nq_record,
// Device Resources
    pub dev_attr: *mut bng_re_dev_attr,
    pub dbg_root: *mut dentry,
    pub stats_ctx: bng_re_stats,
}
