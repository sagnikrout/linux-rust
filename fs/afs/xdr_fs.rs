//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/xdr_fs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// AFS fileserver XDR types
//
// Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_xdr_AFSFetchStatus {
    pub if_version: __be32,
pub const AFS_FSTATUS_VERSION: c_int = 1;
    pub type: __be32,
    pub nlink: __be32,
    pub size_lo: __be32,
    pub data_version_lo: __be32,
    pub author: __be32,
    pub owner: __be32,
    pub caller_access: __be32,
    pub anon_access: __be32,
    pub mode: __be32,
    pub parent_vnode: __be32,
    pub parent_unique: __be32,
    pub seg_size: __be32,
    pub mtime_client: __be32,
    pub mtime_server: __be32,
    pub group: __be32,
    pub sync_counter: __be32,
    pub data_version_hi: __be32,
    pub lock_count: __be32,
    pub size_hi: __be32,
    pub abort_code: __be32,
    pub __packed: },
pub const AFS_DIR_HASHTBL_SIZE: c_int = 128;
pub const AFS_DIR_DIRENT_SIZE: c_int = 32;
pub const AFS_DIR_SLOTS_PER_BLOCK: c_int = 64;
pub const AFS_DIR_BLOCK_SIZE: c_int = 2048;

pub const AFS_DIR_MAX_SLOTS: c_int = 65536;
pub const AFS_DIR_BLOCKS_WITH_CTR: c_int = 128;
pub const AFS_DIR_MAX_BLOCKS: c_int = 1023;
pub const AFS_DIR_RESV_BLOCKS: c_int = 1;
pub const AFS_DIR_RESV_BLOCKS0: c_int = 13;
//
// Directory entry structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union afs_xdr_dirent {
    pub valid: u8,
    pub unused: [u8; 1],
    pub hash_next: __be16,
    pub vnode: __be32,
    pub unique: __be32,
    pub name: [u8; ],
// When determining the number of dirent slots needed to
// represent a directory entry, name should be assumed to be 16
// bytes, due to a now-standardised (mis)calculation, but it is
// in fact 20 bytes in size.  afs_dir_calc_slots() should be
// used for this.
//
// For names longer than (16 or) 20 bytes, extra slots should
// be annexed to this one using the extended_name format.
//
    pub u: },
    pub extended_name: [u8; 32],
    pub __packed: },
//
// Directory block header (one at the beginning of every 2048-byte block).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_xdr_dir_hdr {
    pub npages: __be16,
    pub magic: __be16,

    pub reserved: u8,
    pub bitmap: [u8; 8],
    pub pad: [u8; 19],
    pub __packed: },
//
// Directory block layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union afs_xdr_dir_block {
    pub hdr: afs_xdr_dir_hdr,
    pub hdr: afs_xdr_dir_hdr,
    pub alloc_ctrs: [u8; AFS_DIR_BLOCKS_WITH_CTR],
    pub hashtable: [__be16; AFS_DIR_HASHTBL_SIZE],
    pub meta: },
    pub dirents: [afs_xdr_dirent; AFS_DIR_SLOTS_PER_BLOCK],
    pub __packed: },
//
// Directory layout on a linux VM page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_xdr_dir_page {
    pub blocks: [afs_xdr_dir_block; AFS_DIR_BLOCKS_PER_PAGE],
}

//
// Calculate the number of dirent slots required for any given name length.
// The calculation is made assuming the part of the name in the first slot is
// 16 bytes, rather than 20, but this miscalculation is now standardised.
//
