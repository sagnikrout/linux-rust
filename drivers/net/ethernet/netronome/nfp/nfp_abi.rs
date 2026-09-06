//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_abi.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Netronome Systems, Inc.
pub const __NFP_ABI__: c_int = 1;

pub const NFP_MBOX_CMD: c_uint = 0x00;
pub const NFP_MBOX_RET: c_uint = 0x04;
pub const NFP_MBOX_DATA_LEN: c_uint = 0x08;
pub const NFP_MBOX_RESERVED: c_uint = 0x0c;
pub const NFP_MBOX_DATA: c_uint = 0x10;
//
// enum nfp_mbox_cmd - PF mailbox commands
//
// @NFP_MBOX_NO_CMD:	null command
// Used to indicate previous command has finished.
//
// @NFP_MBOX_POOL_GET:	get shared buffer pool info/config
// Input  - struct nfp_shared_buf_pool_id
// Output - struct nfp_shared_buf_pool_info_get
//
// @NFP_MBOX_POOL_SET:	set shared buffer pool info/config
// Input  - struct nfp_shared_buf_pool_info_set
// Output - None
//
// @NFP_MBOX_PCIE_ABM_ENABLE:	enable PCIe-side advanced buffer management
// Enable advanced buffer management of the PCIe block.  If ABM is disabled
// PCIe block maintains a very short queue of buffers and does tail drop.
// ABM allows more advanced buffering and priority control.
// Input  - None
// Output - None
//
// @NFP_MBOX_PCIE_ABM_DISABLE:	disable PCIe-side advanced buffer management
// Input  - None
// Output - None
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_mbox_cmd {
    NFP_MBOX_NO_CMD			= 0x00,

    NFP_MBOX_POOL_GET		= 0x01,
    NFP_MBOX_POOL_SET		= 0x02,

    NFP_MBOX_PCIE_ABM_ENABLE	= 0x03,
    NFP_MBOX_PCIE_ABM_DISABLE	= 0x04,
}

//
// struct nfp_shared_buf - NFP shared buffer description
// @id:				numerical user-visible id of the shared buffer
// @size:			size in bytes of the buffer
// @ingress_pools_count:	number of ingress pools
// @egress_pools_count:		number of egress pools
// @ingress_tc_count:		number of ingress trafic classes
// @egress_tc_count:		number of egress trafic classes
// @pool_size_unit:		pool size may be in credits, each credit is
// @pool_size_unit bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_shared_buf {
    pub id: __le32,
    pub size: __le32,
    pub ingress_pools_count: __le16,
    pub egress_pools_count: __le16,
    pub ingress_tc_count: __le16,
    pub egress_tc_count: __le16,
    pub pool_size_unit: __le32,
}

//
// struct nfp_shared_buf_pool_id - shared buffer pool identification
// @shared_buf:		shared buffer id
// @pool:		pool index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_shared_buf_pool_id {
    pub shared_buf: __le32,
    pub pool: __le32,
}

//
// struct nfp_shared_buf_pool_info_get - struct devlink_sb_pool_info mirror
// @pool_type:		one of enum devlink_sb_pool_type
// @size:		pool size in units of SB's @pool_size_unit
// @threshold_type:	one of enum devlink_sb_threshold_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_shared_buf_pool_info_get {
    pub pool_type: __le32,
    pub size: __le32,
    pub threshold_type: __le32,
}

//
// struct nfp_shared_buf_pool_info_set - packed args of sb_pool_set
// @id:			pool identification info
// @size:		pool size in units of SB's @pool_size_unit
// @threshold_type:	one of enum devlink_sb_threshold_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_shared_buf_pool_info_set {
    pub id: nfp_shared_buf_pool_id,
    pub size: __le32,
    pub threshold_type: __le32,
}
