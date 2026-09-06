//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_vf_lib_private.h
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
// Copyright (C) 2018-2021, Intel Corporation.

// This header file is for exposing functions in ice_vf_lib.c to other files
// which are also conditionally compiled depending on CONFIG_PCI_IOV.
// Functions which may be used by other files should be exposed as part of
// ice_vf_lib.h
//
// Functions in this file are exposed only when CONFIG_PCI_IOV is enabled, and
// thus this header must not be included by .c files which may be compiled
// with CONFIG_PCI_IOV disabled.
//
// To avoid this, only include this header file directly within .c files that
// are conditionally enabled in the "ice-$(CONFIG_PCI_IOV)" block.
//

extern "C" {
    pub fn ice_initialize_vf_entry(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_deinitialize_vf_entry(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_dis_vf_qs(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_reset_interrupts(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_check_vf_init(vf: *mut ice_vf) -> c_int;
}
extern "C" {
    pub fn ice_err_to_virt_err(err: c_int) -> virtchnl_status_code;
}
extern "C" {
    pub fn ice_vsi_apply_spoofchk(vsi: *mut ice_vsi, enable: bool) -> c_int;
}
extern "C" {
    pub fn ice_is_vf_trusted(vf: *mut ice_vf) -> bool;
}
extern "C" {
    pub fn ice_vf_has_no_qs_ena(vf: *mut ice_vf) -> bool;
}
extern "C" {
    pub fn ice_is_vf_link_up(vf: *mut ice_vf) -> bool;
}
extern "C" {
    pub fn ice_vf_ctrl_invalidate_vsi(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vf_ctrl_vsi_release(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vf_init_host_cfg(vf: *mut ice_vf, vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vf_invalidate_vsi(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vf_vsi_release(vf: *mut ice_vf);
}
