//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic_dcb.h
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
//
// QLogic qlcnic NIC Driver
// Copyright (c)  2009-2013 QLogic Corporation
//
pub const QLCNIC_DCB_STATE: c_int = 0;
pub const QLCNIC_DCB_AEN_MODE: c_int = 1;

extern "C" {
    pub fn qlcnic_register_dcb(: *mut qlcnic_adapter) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_dcb_ops {
    pub ): *mut *mut *mut int (query_hw_capability) (struct qlcnic_dcb , char,
    pub ): *mut *mut int (get_hw_capability) (struct qlcnic_dcb,
    pub u8): *mut *mut *mut *mut int (query_cee_param) (struct qlcnic_dcb , char ,,
    pub ): *mut *mut void (init_dcbnl_ops) (struct qlcnic_dcb,
    pub ): *mut *mut *mut void (aen_handler) (struct qlcnic_dcb , void,
    pub ): *mut *mut int (get_cee_cfg) (struct qlcnic_dcb,
    pub ): *mut *mut void (get_info) (struct qlcnic_dcb,
    pub ): *mut *mut int (attach) (struct qlcnic_dcb,
    pub ): *mut *mut void (free) (struct qlcnic_dcb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_dcb {
    pub param: *mut qlcnic_dcb_mbx_params,
    pub adapter: *mut qlcnic_adapter,
    pub aen_work: delayed_work,
    pub wq: *mut workqueue_struct,
    pub ops: *const qlcnic_dcb_ops,
    pub cfg: *mut qlcnic_dcb_cfg,
    pub state: c_ulong,
}
