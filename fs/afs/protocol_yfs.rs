//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/protocol_yfs.h
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
// YFS protocol bits
//
// Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
pub const YFS_FS_SERVICE: c_int = 2500;
pub const YFS_CM_SERVICE: c_int = 2501;
pub const YFSCBMAX: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum YFS_CM_Operations {
    YFSCBProbe		= 206,	/* probe client */
    YFSCBGetLock		= 207,	/* get contents of CM lock table */
    YFSCBXStatsVersion	= 209,	/* get version of extended statistics */
    YFSCBGetXStats		= 210,	/* get contents of extended statistics data */
    YFSCBInitCallBackState3	= 213,	/* initialise callback state, version 3 */
    YFSCBProbeUuid		= 214,	/* check the client hasn't rebooted */
    YFSCBGetServerPrefs	= 215,
    YFSCBGetCellServDV	= 216,
    YFSCBGetLocalCell	= 217,
    YFSCBGetCacheConfig	= 218,
    YFSCBGetCellByNum	= 65537,
    YFSCBTellMeAboutYourself = 65538, /* get client capabilities */
    YFSCBCallBack		= 64204,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum YFS_FS_Operations {
    YFSFETCHACL		= 64131, /* YFS Fetch file AFS3 ACL */
    YFSFETCHSTATUS		= 64132, /* YFS Fetch file status */
    YFSSTOREACL		= 64134, /* YFS Store file AFS3 ACL */
    YFSSTORESTATUS		= 64135, /* YFS Store file status */
    YFSREMOVEFILE		= 64136, /* YFS Remove a file */
    YFSCREATEFILE		= 64137, /* YFS Create a file */
    YFSRENAME		= 64138, /* YFS Rename or move a file or directory */
    YFSSYMLINK		= 64139, /* YFS Create a symbolic link */
    YFSLINK			= 64140, /* YFS Create a hard link */
    YFSMAKEDIR		= 64141, /* YFS Create a directory */
    YFSREMOVEDIR		= 64142, /* YFS Remove a directory */
    YFSGETVOLUMESTATUS	= 64149, /* YFS Get volume status information */
    YFSSETVOLUMESTATUS	= 64150, /* YFS Set volume status information */
    YFSSETLOCK		= 64156, /* YFS Request a file lock */
    YFSEXTENDLOCK		= 64157, /* YFS Extend a file lock */
    YFSRELEASELOCK		= 64158, /* YFS Release a file lock */
    YFSLOOKUP		= 64161, /* YFS lookup file in directory */
    YFSFLUSHCPS		= 64165,
    YFSFETCHOPAQUEACL	= 64168, /* YFS Fetch file YFS ACL */
    YFSWHOAMI		= 64170,
    YFSREMOVEACL		= 64171,
    YFSREMOVEFILE2		= 64173,
    YFSSTOREOPAQUEACL2	= 64174,
    YFSRENAME_REPLACE	= 64176,
    YFSRENAME_NOREPLACE	= 64177,
    YFSRENAME_EXCHANGE	= 64187,
    YFSINLINEBULKSTATUS	= 64536, /* YFS Fetch multiple file statuses with errors */
    YFSFETCHDATA64		= 64537, /* YFS Fetch file data */
    YFSSTOREDATA64		= 64538, /* YFS Store file data */
    YFSUPDATESYMLINK	= 64540,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_u64 {
    pub msw: __be32,
    pub lsw: __be32,
    pub __packed: },
    pub ntohl(x.lsw): return ((u64)ntohl(x.msw) << 32) |,
    pub }: return (struct yfs_xdr_u64){ .msw = htonl(x >> 32), .lsw = htonl(x),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_vnode {
    pub lo: yfs_xdr_u64,
    pub hi: __be32,
    pub unique: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFid {
    pub volume: yfs_xdr_u64,
    pub vnode: yfs_xdr_vnode,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFetchStatus {
    pub type: __be32,
    pub nlink: __be32,
    pub size: yfs_xdr_u64,
    pub data_version: yfs_xdr_u64,
    pub author: yfs_xdr_u64,
    pub owner: yfs_xdr_u64,
    pub group: yfs_xdr_u64,
    pub mode: __be32,
    pub caller_access: __be32,
    pub anon_access: __be32,
    pub parent: yfs_xdr_vnode,
    pub data_access_protocol: __be32,
    pub mtime_client: yfs_xdr_u64,
    pub mtime_server: yfs_xdr_u64,
    pub lock_count: __be32,
    pub abort_code: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSCallBack {
    pub version: __be32,
    pub expiration_time: yfs_xdr_u64,
    pub type: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSStoreStatus {
    pub mask: __be32,
    pub mode: __be32,
    pub mtime_client: yfs_xdr_u64,
    pub owner: yfs_xdr_u64,
    pub group: yfs_xdr_u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_RPCFlags {
    pub rpc_flags: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSVolSync {
    pub vol_creation_date: yfs_xdr_u64,
    pub vol_update_date: yfs_xdr_u64,
    pub max_quota: yfs_xdr_u64,
    pub blocks_in_use: yfs_xdr_u64,
    pub blocks_avail: yfs_xdr_u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yfs_volume_type {
    yfs_volume_type_ro = 0,
    yfs_volume_type_rw = 1,
}

pub const yfs_FVSOnline: c_uint = 0x1;
pub const yfs_FVSInservice: c_uint = 0x2;
pub const yfs_FVSBlessed: c_uint = 0x4;
pub const yfs_FVSNeedsSalvage: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSFetchVolumeStatus {
    pub vid: yfs_xdr_u64,
    pub parent_id: yfs_xdr_u64,
    pub flags: __be32,
    pub type: __be32,
    pub max_quota: yfs_xdr_u64,
    pub blocks_in_use: yfs_xdr_u64,
    pub part_blocks_avail: yfs_xdr_u64,
    pub part_max_blocks: yfs_xdr_u64,
    pub vol_copy_date: yfs_xdr_u64,
    pub vol_backup_date: yfs_xdr_u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yfs_xdr_YFSStoreVolumeStatus {
    pub mask: __be32,
    pub min_quota: yfs_xdr_u64,
    pub max_quota: yfs_xdr_u64,
    pub file_quota: yfs_xdr_u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yfs_lock_type {
    yfs_LockNone		= -1,
    yfs_LockRead		= 0,
    yfs_LockWrite		= 1,
    yfs_LockExtend		= 2,
    yfs_LockRelease		= 3,
    yfs_LockMandatoryRead	= 0x100,
    yfs_LockMandatoryWrite	= 0x101,
    yfs_LockMandatoryExtend	= 0x102,
}

// RXYFS Viced Capability Flags
pub const YFS_VICED_CAPABILITY_ERRORTRANS: c_uint = 0x0001 /* Deprecated v0.195 */;
pub const YFS_VICED_CAPABILITY_64BITFILES: c_uint = 0x0002 /* Deprecated v0.195 */;
pub const YFS_VICED_CAPABILITY_WRITELOCKACL: c_uint = 0x0004 /* Can lock a file even without lock perm */;
pub const YFS_VICED_CAPABILITY_SANEACLS: c_uint = 0x0008 /* Deprecated v0.195 */;
