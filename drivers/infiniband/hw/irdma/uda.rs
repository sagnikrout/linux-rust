//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/uda.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2016 - 2021 Intel Corporation
pub const IRDMA_UDA_MAX_FSI_MGS: c_int = 4096;
pub const IRDMA_UDA_MAX_PFS: c_int = 16;
pub const IRDMA_UDA_MAX_VFS: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ah_info {
    pub vsi: *mut irdma_sc_vsi,
    pub pd_idx: u32,
    pub dst_arpindex: u32,
    pub dest_ip_addr: [u32; 4],
    pub src_ip_addr: [u32; 4],
    pub flow_label: u32,
    pub ah_idx: u32,
    pub vlan_tag: u16,
    pub insert_vlan_tag: u8,
    pub tc_tos: u8,
    pub hop_ttl: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub ah_valid:1: bool,
    pub ipv4_valid:1: bool,
    pub do_lpbk:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_ah {
    pub dev: *mut irdma_sc_dev,
    pub ah_info: irdma_ah_info,
}
