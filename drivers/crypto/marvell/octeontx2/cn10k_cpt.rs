//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/cn10k_cpt.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2021 Marvell.
//

pub const CN10K_CPT_HW_CTX_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cn10k_cpt_hw_ctx {
    pub u: u64,
    pub reserved_0_47:48: u64,
    pub ctx_push_sz:7: u64,
    pub reserved_55:1: u64,
    pub ctx_hdr_sz:2: u64,
    pub aop_valid:1: u64,
    pub reserved_59:1: u64,
    pub ctx_sz:4: u64,
    pub w0: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_cpt_errata_ctx {
    pub hw_ctx: *mut cn10k_cpt_hw_ctx,
    pub cptr_dma: u64,
}

extern "C" {
    pub fn cn10k_cptpf_lmtst_init(cptpf: *mut otx2_cptpf_dev) -> c_int;
}
extern "C" {
    pub fn cn10k_cptvf_lmtst_init(cptvf: *mut otx2_cptvf_dev) -> c_int;
}
extern "C" {
    pub fn cn10k_cpt_lmtst_free(pdev: *mut pci_dev, lfs: *mut otx2_cptlfs_info);
}
extern "C" {
    pub fn cn10k_cpt_ctx_flush(pdev: *mut pci_dev, cptr: u64, inval: bool);
}
extern "C" {
    pub fn cn10k_cpt_hw_ctx_set(hctx: *mut cn10k_cpt_hw_ctx, ctx_sz: u16);
}
extern "C" {
    pub fn cptvf_hw_ops_get(cptvf: *mut otx2_cptvf_dev);
}
