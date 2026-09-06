//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bng_re/bng_sp.h
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

pub const BNG_VAR_MAX_WQE: c_int = 4352;
pub const BNG_VAR_MAX_SGE: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_dev_attr {
pub const FW_VER_ARR_LEN: c_int = 4;
    pub fw_ver: [u8; FW_VER_ARR_LEN],
pub const BNG_RE_NUM_GIDS_SUPPORTED: c_int = 256;
    pub max_sgid: u16,
    pub max_mrw: u16,
    pub max_qp: u32,
pub const BNG_RE_MAX_OUT_RD_ATOM: c_int = 126;
    pub max_qp_rd_atom: u32,
    pub max_qp_init_rd_atom: u32,
    pub max_qp_wqes: u32,
    pub max_qp_sges: u32,
    pub max_cq: u32,
    pub max_cq_wqes: u32,
    pub max_cq_sges: u32,
    pub max_mr: u32,
    pub max_mr_size: u64,
    pub max_pd: u32,
    pub max_mw: u32,
    pub max_raw_ethy_qp: u32,
    pub max_ah: u32,
    pub max_srq: u32,
    pub max_srq_wqes: u32,
    pub max_srq_sges: u32,
    pub max_pkey: u32,
    pub max_inline_data: u32,
    pub l2_db_size: u32,
    pub tqm_alloc_reqs: [u8; BNG_MAX_TQM_ALLOC_REQ],
    pub is_atomic: bool,
    pub dev_cap_flags: u16,
    pub dev_cap_flags2: u16,
    pub max_dpi: u32,
}

extern "C" {
    pub fn bng_re_get_dev_attr(rcfw: *mut bng_re_rcfw) -> c_int;
}
