//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/exp_rcv.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2017 Intel Corporation.
//

pub const EXP_TID_TIDLEN_MASK: c_uint = 0x7FFULL;
pub const EXP_TID_TIDLEN_SHIFT: c_int = 0;
pub const EXP_TID_TIDCTRL_MASK: c_uint = 0x3ULL;
pub const EXP_TID_TIDCTRL_SHIFT: c_int = 20;
pub const EXP_TID_TIDIDX_MASK: c_uint = 0x3FFULL;
pub const EXP_TID_TIDIDX_SHIFT: c_int = 22;

//
// Define fields in the KDETH header so we can update the header
// template.
//
pub const KDETH_OFFSET_SHIFT: c_int = 0;
pub const KDETH_OFFSET_MASK: c_uint = 0x7fff;
pub const KDETH_OM_SHIFT: c_int = 15;
pub const KDETH_OM_MASK: c_uint = 0x1;
pub const KDETH_TID_SHIFT: c_int = 16;
pub const KDETH_TID_MASK: c_uint = 0x3ff;
pub const KDETH_TIDCTRL_SHIFT: c_int = 26;
pub const KDETH_TIDCTRL_MASK: c_uint = 0x3;
pub const KDETH_INTR_SHIFT: c_int = 28;
pub const KDETH_INTR_MASK: c_uint = 0x1;
pub const KDETH_SH_SHIFT: c_int = 29;
pub const KDETH_SH_MASK: c_uint = 0x1;
pub const KDETH_KVER_SHIFT: c_int = 30;
pub const KDETH_KVER_MASK: c_uint = 0x3;
pub const KDETH_JKEY_SHIFT: c_uint = 0x0;
pub const KDETH_JKEY_MASK: c_uint = 0xff;
pub const KDETH_HCRC_UPPER_SHIFT: c_int = 16;
pub const KDETH_HCRC_UPPER_MASK: c_uint = 0xff;
pub const KDETH_HCRC_LOWER_SHIFT: c_int = 24;
pub const KDETH_HCRC_LOWER_MASK: c_uint = 0xff;

// KDETH OM multipliers and switch over point
pub const KDETH_OM_SMALL: c_int = 4;
pub const KDETH_OM_SMALL_SHIFT: c_int = 2;
pub const KDETH_OM_LARGE: c_int = 64;
pub const KDETH_OM_LARGE_SHIFT: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_group {
    pub list: list_head,
    pub base: u32,
    pub size: u8,
    pub used: u8,
    pub map: u8,
}

//
// Write an "empty" RcvArray entry.
// This function exists so the TID registaration code can use it
// to write to unused/unneeded entries and still take advantage
// of the WC performance improvements. The HFI will ignore this
// write to the RcvArray entry.
//
// Doing the WC fill writes only makes sense if the device is
// present and the RcvArray has been mapped as WC memory.
//
// hfi1_tid_group_to_idx - convert an index to a group
// @rcd - the receive context
// @grp - the group pointer
//
// hfi1_idx_to_tid_group - convert a group to an index
// @rcd - the receive context
// @idx - the index
//
extern "C" {
    pub fn hfi1_alloc_ctxt_rcv_groups(rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn hfi1_free_ctxt_rcv_groups(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn hfi1_exp_tid_group_init(rcd: *mut hfi1_ctxtdata);
}
