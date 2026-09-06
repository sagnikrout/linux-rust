//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/callback.h
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
// linux/fs/nfs/callback.h
//
// Copyright (C) 2004 Trond Myklebust
//
// NFSv4 callback definitions
//

pub const NFS4_CALLBACK: c_uint = 0x40000000;
pub const NFS4_CALLBACK_XDRSIZE: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfs4_callback_procnum {
    CB_NULL = 0,
    CB_COMPOUND = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_process_state {
    pub clp: *mut nfs_client,
    pub slot: *mut nfs4_slot,
    pub net: *mut net,
    pub minorversion: u32,
    pub drc_status: __be32,
    pub referring_calls: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_compound_hdr_arg {
    pub taglen: c_uint,
    pub tag: *const c_char,
    pub minorversion: c_uint,
    pub /: *mut *mut unsigned int cb_ident; / v4.0 callback identifier,
    pub nops: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_compound_hdr_res {
    pub status: *mut __be32,
    pub taglen: c_uint,
    pub tag: *const c_char,
    pub nops: *mut __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_getattrargs {
    pub fh: nfs_fh,
    pub bitmap: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_getattrres {
    pub status: __be32,
    pub bitmap: [u32; 3],
    pub size: u64,
    pub change_attr: u64,
    pub atime: timespec64,
    pub ctime: timespec64,
    pub mtime: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_recallargs {
    pub fh: nfs_fh,
    pub stateid: nfs4_stateid,
    pub truncate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct referring_call {
    pub rc_sequenceid: u32,
    pub rc_slotid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct referring_call_list {
    pub rcl_sessionid: nfs4_sessionid,
    pub rcl_nrefcalls: u32,
    pub rcl_refcalls: *mut referring_call,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_sequenceargs {
    pub csa_addr: *mut sockaddr,
    pub csa_sessionid: nfs4_sessionid,
    pub csa_sequenceid: u32,
    pub csa_slotid: u32,
    pub csa_highestslotid: u32,
    pub csa_cachethis: u32,
    pub csa_nrclists: u32,
    pub csa_rclists: *mut referring_call_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_sequenceres {
    pub csr_status: __be32,
    pub csr_sessionid: nfs4_sessionid,
    pub csr_sequenceid: u32,
    pub csr_slotid: u32,
    pub csr_highestslotid: u32,
    pub csr_target_highestslotid: u32,
}

pub const RCA4_TYPE_MASK_RDATA_DLG: c_int = 0;
pub const RCA4_TYPE_MASK_WDATA_DLG: c_int = 1;
pub const RCA4_TYPE_MASK_DIR_DLG: c_int = 2;
pub const RCA4_TYPE_MASK_FILE_LAYOUT: c_int = 3;
pub const RCA4_TYPE_MASK_BLK_LAYOUT: c_int = 4;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MIN: c_int = 8;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MAX: c_int = 9;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MIN: c_int = 12;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MAX: c_int = 15;
pub const PNFS_FF_RCA4_TYPE_MASK_READ: c_int = 16;
pub const PNFS_FF_RCA4_TYPE_MASK_RW: c_int = 17;
pub const RCA4_TYPE_MASK_ALL: c_uint = 0x3f31f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_recallanyargs {
    pub craa_objs_to_keep: u32,
    pub craa_type_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_recallslotargs {
    pub crsa_target_highest_slotid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_layoutrecallargs {
    pub cbl_recall_type: u32,
    pub cbl_layout_type: u32,
    pub cbl_layoutchanged: u32,
    pub cbl_fh: nfs_fh,
    pub cbl_range: pnfs_layout_range,
    pub cbl_stateid: nfs4_stateid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_devicenotifyitem {
    pub cbd_notify_type: u32,
    pub cbd_layout_type: u32,
    pub cbd_dev_id: nfs4_deviceid,
    pub cbd_immediate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_devicenotifyargs {
    pub ndevs: u32,
    pub devs: *mut cb_devicenotifyitem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_notify_lock_args {
    pub cbnl_fh: nfs_fh,
    pub cbnl_owner: nfs_lowner,
    pub cbnl_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb_offloadargs {
    pub coa_fh: nfs_fh,
    pub coa_stateid: nfs4_stateid,
    pub error: u32,
    pub wr_count: u64,
    pub wr_writeverf: nfs_writeverf,
}

extern "C" {
    pub fn check_gss_callback_principal(: *mut nfs_client, : *mut svc_rqst) -> c_int;
}

extern "C" {
    pub fn nfs_callback_up(minorversion: u32, xprt: *mut rpc_xprt) -> c_int;
}

//
// nfs41: Callbacks are expected to not cause substantial latency,
// so we limit their concurrency to 1 by setting up the maximum number
// of slots for the backchannel.
//
pub const NFS41_BC_MIN_CALLBACKS: c_int = 1;
pub const NFS41_BC_MAX_CALLBACKS: c_int = 1;
pub const NFS4_MIN_NR_CALLBACK_THREADS: c_int = 1;
