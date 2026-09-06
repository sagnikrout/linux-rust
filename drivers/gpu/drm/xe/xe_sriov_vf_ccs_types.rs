//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_vf_ccs_types.h
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
// Copyright © 2025 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sriov_vf_ccs_rw_ctxs {
    XE_SRIOV_VF_CCS_READ_CTX,
    XE_SRIOV_VF_CCS_WRITE_CTX,
    XE_SRIOV_VF_CCS_CTX_COUNT
}

//
// struct xe_sriov_vf_ccs_ctx - VF CCS migration context data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_vf_ccs_ctx {
// @ctx_id: Id to which context it belongs to
    pub ctx_id: xe_sriov_vf_ccs_rw_ctxs,
// @mig_q: exec queues used for migration
    pub mig_q: *mut xe_exec_queue,
// @mem: memory data
// @mem.ccs_bb_pool: Pool from which batch buffers are allocated.
    pub ccs_bb_pool: *mut xe_mem_pool,
    pub mem: },
}

//
// struct xe_sriov_vf_ccs - The VF CCS migration support data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_vf_ccs {
// @contexts: CCS read and write contexts for VF.
    pub contexts: [xe_sriov_vf_ccs_ctx; XE_SRIOV_VF_CCS_CTX_COUNT],
// @initialized: Initialization of VF CCS is completed or not.
    pub initialized: bool,
}
