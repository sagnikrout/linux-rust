//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rvu_devlink.h
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
// Marvell RVU Admin Function Devlink
//
// Copyright (C) 2020 Marvell.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_af_rvu_health {
    NPA_AF_RVU_INTR,
    NPA_AF_RVU_GEN,
    NPA_AF_RVU_ERR,
    NPA_AF_RVU_RAS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_npa_event_ctx {
    pub npa_af_rvu_int: u64,
    pub npa_af_rvu_gen: u64,
    pub npa_af_rvu_err: u64,
    pub npa_af_rvu_ras: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_npa_health_reporters {
    pub npa_event_ctx: *mut rvu_npa_event_ctx,
    pub rvu_hw_npa_intr_reporter: *mut devlink_health_reporter,
    pub intr_work: work_struct,
    pub rvu_hw_npa_gen_reporter: *mut devlink_health_reporter,
    pub gen_work: work_struct,
    pub rvu_hw_npa_err_reporter: *mut devlink_health_reporter,
    pub err_work: work_struct,
    pub rvu_hw_npa_ras_reporter: *mut devlink_health_reporter,
    pub ras_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_af_rvu_health {
    NIX_AF_RVU_INTR,
    NIX_AF_RVU_GEN,
    NIX_AF_RVU_ERR,
    NIX_AF_RVU_RAS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_nix_event_ctx {
    pub nix_af_rvu_int: u64,
    pub nix_af_rvu_gen: u64,
    pub nix_af_rvu_err: u64,
    pub nix_af_rvu_ras: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_nix_health_reporters {
    pub nix_event_ctx: *mut rvu_nix_event_ctx,
    pub rvu_hw_nix_intr_reporter: *mut devlink_health_reporter,
    pub intr_work: work_struct,
    pub rvu_hw_nix_gen_reporter: *mut devlink_health_reporter,
    pub gen_work: work_struct,
    pub rvu_hw_nix_err_reporter: *mut devlink_health_reporter,
    pub err_work: work_struct,
    pub rvu_hw_nix_ras_reporter: *mut devlink_health_reporter,
    pub ras_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvu_devlink {
    pub dl: *mut devlink,
    pub rvu: *mut rvu,
    pub devlink_wq: *mut workqueue_struct,
    pub rvu_npa_health_reporter: *mut rvu_npa_health_reporters,
    pub rvu_nix_health_reporter: *mut rvu_nix_health_reporters,
}

// Devlink APIs
extern "C" {
    pub fn rvu_register_dl(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_unregister_dl(rvu: *mut rvu);
}
