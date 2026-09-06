//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_sriov.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2014-2016 Broadcom Corporation
// Copyright (c) 2016-2018 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const BNXT_VF_MIN_RSS_CTX: c_int = 1;
pub const BNXT_VF_MAX_RSS_CTX: c_int = 1;
pub const BNXT_VF_MIN_L2_CTX: c_int = 1;
pub const BNXT_VF_MAX_L2_CTX: c_int = 4;
extern "C" {
    pub fn bnxt_get_vf_config(: *mut net_device, _arg: c_int, : *mut ifla_vf_info) -> c_int;
}
extern "C" {
    pub fn bnxt_set_vf_mac(: *mut net_device, _arg: c_int, : *mut u8) -> c_int;
}
extern "C" {
    pub fn bnxt_set_vf_vlan(: *mut net_device, _arg: c_int, _arg: u16, _arg: u8, _arg: __be16) -> c_int;
}
extern "C" {
    pub fn bnxt_set_vf_bw(: *mut net_device, _arg: c_int, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_set_vf_link_state(: *mut net_device, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_set_vf_spoofchk(: *mut net_device, _arg: c_int, _arg: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_is_trusted_vf(bp: *mut bnxt, vf: *mut bnxt_vf_info) -> bool;
}
extern "C" {
    pub fn bnxt_set_vf_trust(dev: *mut net_device, vf_id: c_int, trust: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_cfg_hw_sriov(bp: *mut bnxt, num_vfs: *mut c_int, reset: bool) -> c_int;
}
extern "C" {
    pub fn __bnxt_sriov_disable(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_hwrm_exec_fwd_req(: *mut bnxt);
}
extern "C" {
    pub fn bnxt_update_vf_mac(: *mut bnxt);
}
extern "C" {
    pub fn bnxt_approve_mac(: *mut bnxt, : *const u8, _arg: bool) -> c_int;
}
