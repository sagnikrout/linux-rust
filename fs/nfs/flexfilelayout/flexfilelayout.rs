//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/flexfilelayout/flexfilelayout.h
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
// NFSv4 flexfile layout driver data structures.
//
// Copyright (c) 2014, Primary Data, Inc. All rights reserved.
//
// Tao Peng <bergwolf@primarydata.com>
//
pub const FF_FLAGS_NO_LAYOUTCOMMIT: c_int = 1;
pub const FF_FLAGS_NO_IO_THRU_MDS: c_int = 2;
pub const FF_FLAGS_NO_READ_IO: c_int = 4;

// XXX: Let's filter out insanely large mirror count for now to avoid oom
// due to network error etc.
pub const NFS4_FLEXFILE_LAYOUT_MAX_MIRROR_CNT: c_int = 4096;
pub const NFS4_FLEXFILE_LAYOUT_MAX_STRIPE_CNT: c_int = 4096;
// LAYOUTSTATS report interval in ms

pub const FF_LAYOUTSTATS_MAXDEV: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_ds_version {
    pub version: u32,
    pub minor_version: u32,
    pub rsize: u32,
    pub wsize: u32,
    pub tightly_coupled: bool,
}

// chained in global deviceid hlist
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layout_ds {
    pub id_node: nfs4_deviceid_node,
    pub ds_versions_cnt: u32,
    pub ds_versions: *mut nfs4_ff_ds_version,
    pub ds: *mut nfs4_pnfs_ds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layout_ds_err {
    pub /: *mut *mut list_head list; / linked in mirror error_list,
    pub offset: u64,
    pub length: u64,
    pub status: c_int,
    pub opnum: nfs_opnum4,
    pub stateid: nfs4_stateid,
    pub deviceid: nfs4_deviceid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_io_stat {
    pub ops_requested: __u64,
    pub bytes_requested: __u64,
    pub ops_completed: __u64,
    pub bytes_completed: __u64,
    pub bytes_not_delivered: __u64,
    pub total_busy_time: ktime_t,
    pub aggregate_completion_time: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_busy_timer {
    pub start_time: ktime_t,
    pub n_ops: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layoutstat {
    pub io_stat: nfs4_ff_io_stat,
    pub busy_timer: nfs4_ff_busy_timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layout_ds_stripe {
    pub mirror: *mut nfs4_ff_layout_mirror,
    pub devid: nfs4_deviceid,
    pub efficiency: u32,
    pub mirror_ds: *mut nfs4_ff_layout_ds,
    pub fh_versions_cnt: u32,
    pub fh_versions: *mut nfs_fh,
    pub stateid: nfs4_stateid,
    pub ro_cred: *const cred __rcu,
    pub rw_cred: *const cred __rcu,
    pub nfl: nfs_file_localio,
    pub read_stat: nfs4_ff_layoutstat,
    pub write_stat: nfs4_ff_layoutstat,
    pub start_time: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layout_mirror {
    pub layout: *mut pnfs_layout_hdr,
    pub mirrors: list_head,
    pub dss_count: u32,
    pub dss: *mut nfs4_ff_layout_ds_stripe,
    pub ref: refcount_t,
    pub lock: spinlock_t,
    pub flags: c_ulong,
    pub report_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ff_layout_segment {
    pub generic_hdr: pnfs_layout_segment,
    pub stripe_unit: u64,
    pub flags: u32,
    pub mirror_array_cnt: u32,
    pub __counted_by(mirror_array_cnt): *mut *mut nfs4_ff_layout_mirror mirror_array[],
}

// nfs4_flexfile_layout::flags bit indices

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_flexfile_layout {
    pub generic_hdr: pnfs_layout_hdr,
    pub commit_info: pnfs_ds_commit_info,
    pub mirrors: list_head,
    pub /: *mut *mut list_head error_list; / nfs4_ff_layout_ds_err,
    pub /: *mut *mut ktime_t last_report_time; / Layoutstat report times,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_flexfile_layoutreturn_args {
    pub errors: list_head,
    pub devinfo: [nfs42_layoutstat_devinfo; FF_LAYOUTSTATS_MAXDEV],
    pub num_errors: c_uint,
    pub num_dev: c_uint,
    pub pages: [*mut page; 1],
}

extern "C" {
    pub fn container_of(_arg: lo, nfs4_flexfile_layout: struct, _arg: generic_hdr) -> return;
}
extern "C" {
    pub fn container_of(_arg: node, nfs4_ff_layout_ds: struct, _arg: id_node) -> return;
}
//
// Sticky hdr-level mirror of FF_FLAGS_NO_IO_THRU_MDS so callers that have
// no current lseg (e.g. between LAYOUTRETURN and the next LAYOUTGET) can
// still honor the no-MDS-fallback policy.
//
extern "C" {
    pub fn do_div(_arg: tmp, _arg: dss_count) -> return;
}
extern "C" {
    pub fn nfs4_ff_layout_put_deviceid(mirror_ds: *mut nfs4_ff_layout_ds);
}
extern "C" {
    pub fn nfs4_ff_layout_free_deviceid(mirror_ds: *mut nfs4_ff_layout_ds);
}
extern "C" {
    pub fn ff_layout_send_layouterror(lseg: *mut pnfs_layout_segment);
}
extern "C" {
    pub fn ff_layout_encode_ds_ioerr(xdr: *mut xdr_stream, head: *const list_head) -> c_int;
}
extern "C" {
    pub fn ff_layout_free_ds_ioerr(head: *mut list_head);
}
extern "C" {
    pub fn ff_layout_avoid_mds_available_ds(lseg: *mut pnfs_layout_segment) -> bool;
}
extern "C" {
    pub fn ff_layout_avoid_read_on_rw(lseg: *mut pnfs_layout_segment) -> bool;
}
