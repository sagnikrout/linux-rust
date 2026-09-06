//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/fwctl/pds.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Copyright(c) Advanced Micro Devices, Inc
//
// fwctl interface info for pds_fwctl
//

//
// struct fwctl_info_pds
// @uctx_caps:  bitmap of firmware capabilities
//
// Return basic information about the FW interface available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_info_pds {
    pub uctx_caps: __u32,
}

//
// enum pds_fwctl_capabilities
// @PDS_FWCTL_QUERY_CAP: firmware can be queried for information
// @PDS_FWCTL_SEND_CAP:  firmware can be sent commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_fwctl_capabilities {
    PDS_FWCTL_QUERY_CAP = 0,
    PDS_FWCTL_SEND_CAP,
}

//
// struct fwctl_rpc_pds
// @in.op:       requested operation code
// @in.ep:       firmware endpoint to operate on
// @in.rsvd:     reserved
// @in.len:      length of payload data
// @in.payload:  address of payload buffer
// @in:          rpc in parameters
// @out.retval:  operation result value
// @out.rsvd:    reserved
// @out.len:     length of result data buffer
// @out.payload: address of payload data buffer
// @out:         rpc out parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc_pds {
    pub op: __u32,
    pub ep: __u32,
    pub rsvd: __u32,
    pub len: __u32,
    pub payload: __aligned_u64,
    pub in: },
    pub retval: __u32,
    pub rsvd: [__u32; 2],
    pub len: __u32,
    pub payload: __aligned_u64,
    pub out: },
}
