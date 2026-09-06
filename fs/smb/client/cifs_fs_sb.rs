//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifs_fs_sb.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) International Business Machines  Corp., 2002,2004
// Author(s): Steve French (sfrench@us.ibm.com)
//

pub const CIFS_MOUNT_NO_XATTR: c_uint = 0x10  /* if set - disable xattr support       */;
pub const CIFS_MOUNT_MAP_SPECIAL_CHR: c_uint = 0x20 /* remap illegal chars in filenames   */;
pub const CIFS_MOUNT_POSIX_PATHS: c_uint = 0x40  /* Negotiate posix pathnames if possible*/;
pub const CIFS_MOUNT_UNX_EMUL: c_uint = 0x80  /* Network compat with SFUnix emulation */;
pub const CIFS_MOUNT_NO_BRL: c_uint = 0x100 /* No sending byte range locks to srv   */;
pub const CIFS_MOUNT_CIFS_ACL: c_uint = 0x200 /* send ACL requests to non-POSIX srv   */;
pub const CIFS_MOUNT_OVERR_UID: c_uint = 0x400 /* override uid returned from server    */;
pub const CIFS_MOUNT_OVERR_GID: c_uint = 0x800 /* override gid returned from server    */;
pub const CIFS_MOUNT_DYNPERM: c_uint = 0x1000 /* allow in-memory only mode setting   */;
pub const CIFS_MOUNT_NOPOSIXBRL: c_uint = 0x2000 /* mandatory not posix byte range lock */;
pub const CIFS_MOUNT_NOSSYNC: c_uint = 0x4000 /* don't do slow SMBflush on every sync*/;
pub const CIFS_MOUNT_FSCACHE: c_uint = 0x8000 /* local caching enabled */;
pub const CIFS_MOUNT_MF_SYMLINKS: c_uint = 0x10000 /* Minshall+French Symlinks enabled */;
pub const CIFS_MOUNT_MULTIUSER: c_uint = 0x20000 /* multiuser mount */;
pub const CIFS_MOUNT_STRICT_IO: c_uint = 0x40000 /* strict cache mode */;
pub const CIFS_MOUNT_RWPIDFORWARD: c_uint = 0x80000 /* use pid forwarding for rw */;
pub const CIFS_MOUNT_POSIXACL: c_uint = 0x100000 /* mirror of SB_POSIXACL in mnt_cifs_flags */;
pub const CIFS_MOUNT_CIFS_BACKUPUID: c_uint = 0x200000 /* backup intent bit for a user */;
pub const CIFS_MOUNT_CIFS_BACKUPGID: c_uint = 0x400000 /* backup intent bit for a group */;
pub const CIFS_MOUNT_MAP_SFM_CHR: c_uint = 0x800000 /* SFM/MAC mapping for illegal chars */;
pub const CIFS_MOUNT_USE_PREFIX_PATH: c_uint = 0x1000000 /* make subpath with unaccessible;
// root mountable
//
pub const CIFS_MOUNT_UID_FROM_ACL: c_uint = 0x2000000 /* try to get UID via special SID */;
pub const CIFS_MOUNT_NO_HANDLE_CACHE: c_uint = 0x4000000 /* disable caching dir handles */;
pub const CIFS_MOUNT_NO_DFS: c_uint = 0x8000000 /* disable DFS resolving */;
pub const CIFS_MOUNT_MODE_FROM_SID: c_uint = 0x10000000 /* retrieve mode from special ACE */;
pub const CIFS_MOUNT_RO_CACHE: c_uint = 0x20000000  /* assumes share will not change */;
pub const CIFS_MOUNT_RW_CACHE: c_uint = 0x40000000  /* assumes only client accessing */;
pub const CIFS_MOUNT_SHUTDOWN: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_sb_info {
    pub tlink_tree: rb_root,
    pub tcon_sb_link: list_head,
    pub tlink_tree_lock: spinlock_t,
    pub master_tlink: *mut tcon_link,
    pub local_nls: *mut nls_table,
    pub ctx: *mut smb3_fs_context,
    pub active: core::sync::atomic::AtomicI32,
    pub mnt_cifs_flags: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t outstanding_rreq; / nr of rreqs not yet fully deinitialized,
    pub prune_tlinks: delayed_work,
    pub rcu: rcu_head,
// only used when CIFS_MOUNT_USE_PREFIX_PATH is set
    pub prepath: *mut c_char,
//
// Indicate whether serverino option was turned off later
// (cifs_autodisable_serverino) in order to match new mounts.
//
    pub mnt_cifs_serverino_autodisabled: bool,
//
// Available once the mount has completed.
//
    pub root: *mut dentry,
}
