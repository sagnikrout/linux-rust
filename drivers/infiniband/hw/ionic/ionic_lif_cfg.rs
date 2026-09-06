//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/ionic/ionic_lif_cfg.h
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
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

pub const IONIC_PAGE_SIZE_SUPPORTED: c_uint = 0x40201000 /* 4kb, 2Mb, 1Gb */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_lif_cfg {
    pub hwdev: *mut device,
    pub lif: *mut ionic_lif,
    pub lif_index: c_int,
    pub lif_hw_index: c_int,
    pub dbid: u32,
    pub dbid_count: c_int,
    pub dbpage: *mut u64 __iomem,
    pub intr_ctrl: *mut ionic_intr __iomem,
    pub db_phys: phys_addr_t,
    pub phc_state: *mut c_void,
    pub page_size_supported: u64,
    pub npts_per_lif: u32,
    pub nmrs_per_lif: u32,
    pub nahs_per_lif: u32,
    pub aq_base: u32,
    pub cq_base: u32,
    pub eq_base: u32,
    pub aq_count: c_int,
    pub eq_count: c_int,
    pub cq_count: c_int,
    pub qp_count: c_int,
    pub stats_type: u16,
    pub aq_qtype: u8,
    pub sq_qtype: u8,
    pub rq_qtype: u8,
    pub cq_qtype: u8,
    pub eq_qtype: u8,
    pub udma_count: u8,
    pub udma_qgrp_shift: u8,
    pub rdma_version: u8,
    pub qp_opcodes: u8,
    pub admin_opcodes: u8,
    pub max_stride: u8,
    pub sq_expdb: bool,
    pub rq_expdb: bool,
    pub expdb_mask: u8,
}

extern "C" {
    pub fn ionic_fill_lif_cfg(lif: *mut ionic_lif, cfg: *mut ionic_lif_cfg);
}
extern "C" {
    pub fn ionic_lif_fw_version(lif: *mut ionic_lif, str: *mut c_char, len: usize);
}
extern "C" {
    pub fn ionic_lif_asic_rev(lif: *mut ionic_lif) -> u8;
}
