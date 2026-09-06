//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_dcb.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_dcb {
    pub max_tc: u8,
    pub ieee_pfc: *mut ieee_pfc,
    pub ieee_ets: *mut ieee_ets,
    pub dcbx_cap: u8,
    pub default_pri: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_cos2bw_cfg {
    pub pad: [u8; 3],
    pub queue_id: u8,
    pub min_bw: __le32,
    pub max_bw: __le32,
    pub tsa: u8,
    pub pri_lvl: u8,
    pub bw_weight: u8,
// for min_bw / max_bw

    pub unused: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_dscp2pri_entry {
    pub dscp: u8,
    pub mask: u8,
    pub pri: u8,
}

pub const HWRM_STRUCT_DATA_SUBTYPE_HOST_OPERATIONAL: c_uint = 0x0300;
extern "C" {
    pub fn bnxt_dcb_init(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dcb_free(bp: *mut bnxt);
}
