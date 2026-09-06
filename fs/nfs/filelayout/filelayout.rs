//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/filelayout/filelayout.h
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


//
// NFSv4 file layout driver data structures.
//
// Copyright (c) 2002
// The Regents of the University of Michigan
// All Rights Reserved
//
// Dean Hildebrand <dhildebz@umich.edu>
//
// Permission is granted to use, copy, create derivative works, and
// redistribute this software and such derivative works for any purpose,
// so long as the name of the University of Michigan is not used in
// any advertising or publicity pertaining to the use or distribution
// of this software without specific, written prior authorization. If
// the above copyright notice or any other identification of the
// University of Michigan is included in any copy of any portion of
// this software, then the disclaimer below must also be included.
//
// This software is provided as is, without representation or warranty
// of any kind either express or implied, including without limitation
// the implied warranties of merchantability, fitness for a particular
// purpose, or noninfringement.  The Regents of the University of
// Michigan shall not be liable for any damages, including special,
// indirect, incidental, or consequential damages, with respect to any
// claim arising out of or in connection with the use of the software,
// even if it has been or is hereafter advised of the possibility of
// such damages.
//

//
// Field testing shows we need to support up to 4096 stripe indices.
// We store each index as a u8 (u32 on the wire) to keep the memory footprint
// reasonable. This in turn means we support a maximum of 256
// RFC 5661 multipath_list4 structures.
//
pub const NFS4_PNFS_MAX_STRIPE_CNT: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stripetype4 {
    STRIPE_SPARSE = 1,
    STRIPE_DENSE = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_file_layout_dsaddr {
    pub id_node: nfs4_deviceid_node,
    pub stripe_count: u32,
    pub stripe_indices: *mut u8,
    pub ds_num: u32,
    pub __counted_by(ds_num): *mut *mut nfs4_pnfs_ds ds_list[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_filelayout_segment {
    pub generic_hdr: pnfs_layout_segment,
    pub stripe_type: u32,
    pub commit_through_mds: u32,
    pub stripe_unit: u32,
    pub first_stripe_index: u32,
    pub pattern_offset: u64,
    pub deviceid: nfs4_deviceid,
    pub /: *mut *mut *mut nfs4_file_layout_dsaddr dsaddr; / Point to GETDEVINFO data,
    pub num_fh: c_uint,
    pub fh_array: *mut nfs_fh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_filelayout {
    pub generic_hdr: pnfs_layout_hdr,
    pub commit_info: pnfs_ds_commit_info,
}

extern "C" {
    pub fn container_of(_arg: lo, nfs4_filelayout: struct, _arg: generic_hdr) -> return;
}
extern "C" {
    pub fn test_bit(_arg: NFS_DEVICEID_INVALID, _arg: &node->flags) -> return;
}
extern "C" {
    pub fn nfs4_fl_calc_j_index(lseg: *mut pnfs_layout_segment, offset: loff_t) -> u32;
}
extern "C" {
    pub fn nfs4_fl_calc_ds_index(lseg: *mut pnfs_layout_segment, j: u32) -> u32;
}
extern "C" {
    pub fn nfs4_fl_put_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr);
}
extern "C" {
    pub fn nfs4_fl_free_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr);
}
