//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/afs.h
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
// AFS common types
//
// Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

pub const AFS_VL_MAX_LIFESPAN: c_int = 120;
pub const AFS_PROBE_MAX_LIFESPAN: c_int = 30;
pub type afs_volid_t = u64;
pub type afs_vnodeid_t = u64;
pub type afs_dataversion_t = u64;

//
// AFS file identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_fid {
    pub /: *mut *mut afs_volid_t vid; / volume ID,
    pub /: *mut *mut afs_vnodeid_t vnode; / Lower 64-bits of file index within volume,
    pub /: *mut *mut u32 vnode_hi; / Upper 32-bits of file index,
    pub /: *mut *mut u32 unique; / unique ID number (file index version),
}

//
// AFS callback notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_callback {
    pub /: *mut *mut time64_t expires_at; / Time at which expires,
// unsigned		version;	/* Callback version
// afs_callback_type_t	type;		/* Type of callback
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_callback_break {
    pub /: *mut *mut afs_fid fid; / File identifier,
// struct afs_callback	cb;		/* Callback details
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_uuid {
    pub /: *mut *mut __be32 time_low; / low part of timestamp,
    pub /: *mut *mut __be16 time_mid; / mid part of timestamp,
    pub /: *mut *mut __be16 time_hi_and_version; / high part of timestamp and version,
    pub /: *mut *mut __s8 clock_seq_hi_and_reserved; / clock seq hi and variant,
    pub /: *mut *mut __s8 clock_seq_low; / clock seq low,
    pub /: *mut *mut __s8 node[6]; / spatially unique node ID (MAC addr),
}

//
// AFS volume information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_volume_info {
    pub /: *mut *mut afs_volid_t vid; / volume ID,
    pub /: *mut *mut afs_voltype_t type; / type of this volume,
    pub /: *mut *mut afs_volid_t type_vids[5]; / volume ID's for possible types for this vol,
// list of fileservers serving this volume
    pub /: *mut *mut size_t nservers; / number of entries used in servers[],
    pub /: *mut *mut in_addr addr; / fileserver address,
    pub servers: [}; 8],
}

//
// AFS security ACE access mask
//
pub type afs_access_t = u32;
pub const AFS_ACE_READ: c_uint = 0x00000001U	/* - permission to read a file/dir */;
pub const AFS_ACE_WRITE: c_uint = 0x00000002U	/* - permission to write/chmod a file */;
pub const AFS_ACE_INSERT: c_uint = 0x00000004U	/* - permission to create dirent in a dir */;
pub const AFS_ACE_LOOKUP: c_uint = 0x00000008U	/* - permission to lookup a file/dir in a dir */;
pub const AFS_ACE_DELETE: c_uint = 0x00000010U	/* - permission to delete a dirent from a dir */;
pub const AFS_ACE_LOCK: c_uint = 0x00000020U	/* - permission to lock a file */;
pub const AFS_ACE_ADMINISTER: c_uint = 0x00000040U	/* - permission to change ACL */;
pub const AFS_ACE_USER_A: c_uint = 0x01000000U	/* - 'A' user-defined permission */;
pub const AFS_ACE_USER_B: c_uint = 0x02000000U	/* - 'B' user-defined permission */;
pub const AFS_ACE_USER_C: c_uint = 0x04000000U	/* - 'C' user-defined permission */;
pub const AFS_ACE_USER_D: c_uint = 0x08000000U	/* - 'D' user-defined permission */;
pub const AFS_ACE_USER_E: c_uint = 0x10000000U	/* - 'E' user-defined permission */;
pub const AFS_ACE_USER_F: c_uint = 0x20000000U	/* - 'F' user-defined permission */;
pub const AFS_ACE_USER_G: c_uint = 0x40000000U	/* - 'G' user-defined permission */;
pub const AFS_ACE_USER_H: c_uint = 0x80000000U	/* - 'H' user-defined permission */;
//
// AFS file status information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_file_status {
    pub /: *mut *mut u64 size; / file size,
    pub /: *mut *mut afs_dataversion_t data_version; / current data version,
    pub /: *mut *mut timespec64 mtime_client; / Last time client changed data,
    pub /: *mut *mut timespec64 mtime_server; / Last time server changed data,
    pub /: *mut *mut s64 author; / author ID,
    pub /: *mut *mut s64 owner; / owner ID,
    pub /: *mut *mut s64 group; / group ID,
    pub /: *mut *mut afs_access_t caller_access; / access rights for authenticated caller,
    pub /: *mut *mut afs_access_t anon_access; / access rights for unauthenticated caller,
    pub /: *mut *mut umode_t mode; / UNIX mode,
    pub /: *mut *mut afs_file_type_t type; / file type,
    pub /: *mut *mut u32 nlink; / link count,
    pub /: *mut *mut s32 lock_count; / file lock count (0=UNLK -1=WRLCK +ve=#RDLCK,
    pub /: *mut *mut u32 abort_code; / Abort if bulk-fetching this failed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_status_cb {
    pub status: afs_file_status,
    pub callback: afs_callback,
    pub /: *mut *mut bool have_status; / True if status record was retrieved,
    pub /: *mut *mut bool have_cb; / True if cb record was retrieved,
    pub /: *mut *mut bool have_error; / True if status.abort_code indicates an error,
}

//
// AFS file status change request
//
pub const AFS_SET_MTIME: c_uint = 0x01		/* set the mtime */;
pub const AFS_SET_OWNER: c_uint = 0x02		/* set the owner ID */;
pub const AFS_SET_GROUP: c_uint = 0x04		/* set the group ID (unsupported?) */;
pub const AFS_SET_MODE: c_uint = 0x08		/* set the UNIX mode */;
pub const AFS_SET_SEG_SIZE: c_uint = 0x10		/* set the segment size (unsupported) */;
//
// AFS volume synchronisation information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_volsync {
    pub /: *mut *mut time64_t creation; / Volume creation time (or TIME64_MIN),
    pub /: *mut *mut time64_t update; / Volume update time (or TIME64_MIN),
}

//
// AFS volume status record
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_volume_status {
    pub /: *mut *mut afs_volid_t vid; / volume ID,
    pub /: *mut *mut afs_volid_t parent_id; / parent volume ID,
    pub /: *mut *mut u8 online; / true if volume currently online and available,
    pub /: *mut *mut u8 in_service; / true if volume currently in service,
    pub /: *mut *mut u8 blessed; / same as in_service,
    pub /: *mut *mut u8 needs_salvage; / true if consistency checking required,
    pub /: *mut *mut u32 type; / volume type (afs_voltype_t),
    pub /: *mut *mut u64 min_quota; / minimum space set aside (blocks),
    pub /: *mut *mut u64 max_quota; / maximum space this volume may occupy (blocks),
    pub /: *mut *mut u64 blocks_in_use; / space this volume currently occupies (blocks),
    pub /: *mut *mut u64 part_blocks_avail; / space available in volume's partition,
    pub /: *mut *mut u64 part_max_blocks; / size of volume's partition,
    pub vol_copy_date: i64,
    pub vol_backup_date: i64,
}

pub const AFS_BLOCK_SIZE: c_int = 1024;
//
// XDR encoding of UUID in AFS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_uuid__xdr {
    pub time_low: __be32,
    pub time_mid: __be32,
    pub time_hi_and_version: __be32,
    pub clock_seq_hi_and_reserved: __be32,
    pub clock_seq_low: __be32,
    pub node: [__be32; 6],
}
