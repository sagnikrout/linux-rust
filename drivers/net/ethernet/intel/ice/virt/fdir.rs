//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/virt/fdir.h
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
// Copyright (C) 2021, Intel Corporation.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fdir_ctx_stat {
    ICE_FDIR_CTX_READY,
    ICE_FDIR_CTX_IRQ,
    ICE_FDIR_CTX_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_fdir_ctx {
    pub rx_tmr: timer_list,
    pub v_opcode: virtchnl_ops,
    pub stat: ice_fdir_ctx_stat,
    pub rx_desc: ice_32b_rx_flex_desc,

    pub flags: u32,
    pub conf: *mut c_void,
}

// VF FDIR information structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_fdir {
    pub fdir_fltr_cnt: [u16; ICE_FLTR_PTYPE_MAX][ICE_FD_HW_SEG_MAX],
    pub prof_entry_cnt: [c_int; ICE_FLTR_PTYPE_MAX][ICE_FD_HW_SEG_MAX],
    pub fdir_fltr_cnt_total: u16,
    pub fdir_prof: *mut ice_fd_hw_prof,
    pub fdir_rule_idr: idr,
    pub fdir_rule_list: list_head,
    pub /: *mut *mut spinlock_t ctx_lock; / protects FDIR context info,
    pub ctx_irq: ice_vf_fdir_ctx,
    pub ctx_done: ice_vf_fdir_ctx,
}

extern "C" {
    pub fn ice_vc_add_fdir_fltr(vf: *mut ice_vf, msg: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_vc_del_fdir_fltr(vf: *mut ice_vf, msg: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_vf_fdir_init(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vf_fdir_exit(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_flush_fdir_ctx(pf: *mut ice_pf);
}

