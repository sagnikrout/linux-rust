//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/flexfilelayoutxdr.h
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
//
// Copyright (c) 2016 Tom Haynes <loghyr@primarydata.com>
//
pub const _NFSD_FLEXFILELAYOUTXDR_H: c_int = 1;

pub const FF_FLAGS_NO_LAYOUTCOMMIT: c_int = 1;
pub const FF_FLAGS_NO_IO_THRU_MDS: c_int = 2;
pub const FF_FLAGS_NO_READ_IO: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_ff_netaddr {
    pub 1]: char netid[FF_NETID_LEN +,
    pub 1]: char addr[FF_ADDR_LEN +,
    pub netid_len: u32,
    pub addr_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_ff_device_addr {
    pub netaddr: pnfs_ff_netaddr,
    pub version: u32,
    pub minor_version: u32,
    pub rsize: u32,
    pub wsize: u32,
    pub tightly_coupled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_ff_layout {
    pub flags: u32,
    pub stats_collect_hint: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub deviceid: nfsd4_deviceid,
    pub stateid: stateid_t,
    pub fh: nfs_fh,
}
