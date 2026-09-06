//! Automatically rewritten from C Header to Rust Module
//! Source: fs/affs/amigaffs.h
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

pub const FS_OFS: c_uint = 0x444F5300;
pub const FS_FFS: c_uint = 0x444F5301;
pub const FS_INTLOFS: c_uint = 0x444F5302;
pub const FS_INTLFFS: c_uint = 0x444F5303;
pub const FS_DCOFS: c_uint = 0x444F5304;
pub const FS_DCFFS: c_uint = 0x444F5305;
pub const MUFS_FS: c_uint = 0x6d754653   /* 'muFS' */;
pub const MUFS_OFS: c_uint = 0x6d754600   /* 'muF\0' */;
pub const MUFS_FFS: c_uint = 0x6d754601   /* 'muF\1' */;
pub const MUFS_INTLOFS: c_uint = 0x6d754602   /* 'muF\2' */;
pub const MUFS_INTLFFS: c_uint = 0x6d754603   /* 'muF\3' */;
pub const MUFS_DCOFS: c_uint = 0x6d754604   /* 'muF\4' */;
pub const MUFS_DCFFS: c_uint = 0x6d754605   /* 'muF\5' */;
pub const T_SHORT: c_int = 2;
pub const T_LIST: c_int = 16;
pub const T_DATA: c_int = 8;

pub const ST_ROOT: c_int = 1;
pub const ST_USERDIR: c_int = 2;
pub const ST_SOFTLINK: c_int = 3;
pub const ST_LINKDIR: c_int = 4;
pub const AFFS_ROOT_BMAPS: c_int = 25;
// Seconds since Amiga epoch of 1978/01/01 to UNIX

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_date {
    pub days: __be32,
    pub mins: __be32,
    pub ticks: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_short_date {
    pub days: __be16,
    pub mins: __be16,
    pub ticks: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_root_head {
    pub ptype: __be32,
// The following fields are not used, but kept as documentation.
    pub spare1: __be32,
    pub spare2: __be32,
    pub hash_size: __be32,
    pub spare3: __be32,
    pub checksum: __be32,
    pub hashtable: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_root_tail {
    pub bm_flag: __be32,
    pub bm_blk: [__be32; AFFS_ROOT_BMAPS],
    pub bm_ext: __be32,
    pub root_change: affs_date,
    pub disk_name: [u8; 32],
    pub spare1: __be32,
    pub spare2: __be32,
    pub disk_change: affs_date,
    pub disk_create: affs_date,
    pub spare3: __be32,
    pub spare4: __be32,
    pub dcache: __be32,
    pub stype: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_head {
    pub ptype: __be32,
    pub key: __be32,
    pub block_count: __be32,
    pub spare1: __be32,
    pub first_data: __be32,
    pub checksum: __be32,
    pub table: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_tail {
    pub spare1: __be32,
    pub uid: __be16,
    pub gid: __be16,
    pub protect: __be32,
    pub size: __be32,
    pub comment: [u8; 92],
    pub change: affs_date,
    pub name: [u8; 32],
    pub spare2: __be32,
    pub original: __be32,
    pub link_chain: __be32,
    pub spare: [__be32; 5],
    pub hash_chain: __be32,
    pub parent: __be32,
    pub extension: __be32,
    pub stype: __be32,
}

// Permission bits
pub const FIBF_OTR_READ: c_uint = 0x8000;
pub const FIBF_OTR_WRITE: c_uint = 0x4000;
pub const FIBF_OTR_EXECUTE: c_uint = 0x2000;
pub const FIBF_OTR_DELETE: c_uint = 0x1000;
pub const FIBF_GRP_READ: c_uint = 0x0800;
pub const FIBF_GRP_WRITE: c_uint = 0x0400;
pub const FIBF_GRP_EXECUTE: c_uint = 0x0200;
pub const FIBF_GRP_DELETE: c_uint = 0x0100;
pub const FIBF_HIDDEN: c_uint = 0x0080;
pub const FIBF_SCRIPT: c_uint = 0x0040;
pub const FIBF_PURE: c_uint = 0x0020		/* no use under linux */;
pub const FIBF_ARCHIVED: c_uint = 0x0010		/* never set, always cleared on write */;
pub const FIBF_NOREAD: c_uint = 0x0008		/* 0 means allowed */;
pub const FIBF_NOWRITE: c_uint = 0x0004		/* 0 means allowed */;
pub const FIBF_NOEXECUTE: c_uint = 0x0002		/* 0 means allowed, ignored under linux */;
pub const FIBF_NODELETE: c_uint = 0x0001		/* 0 means allowed */;
pub const FIBF_OWNER: c_uint = 0x000F		/* Bits pertaining to owner */;
pub const FIBF_MASK: c_uint = 0xEE0E		/* Bits modified by Linux */;
