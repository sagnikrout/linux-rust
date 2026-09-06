//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/export.c
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
// Copyright (C) International Business Machines  Corp., 2007
// Author(s): Steve French (sfrench@us.ibm.com)
//
// Common Internet FileSystem (CIFS) client
//
// Operations related to support for exporting files via NFSD
//
// See Documentation/filesystems/nfs/exporting.rst
// and examples in fs/exportfs
//
// Since cifs is a network file system, an "fsid" must be included for
// any nfs exports file entries which refer to cifs paths.  In addition
// the cifs mount must be mounted with the "serverino" option (ie use stable
// server inode numbers instead of locally generated temporary ones).
// Although cifs inodes do not use generation numbers (have generation number
// of zero) - the inode number alone should be good enough for simple cases
// in which users want to export cifs shares with NFS. The decode and encode
// could be improved by using a new routine which expects 64 bit inode numbers
// instead of the default 32 bit routines in fs/exportfs
//

    static struct dentry *cifs_get_parent(struct dentry *dentry)
    {
// BB need to add code here eventually to enable export via NFSD
    cifs_dbg(FYI, "get parent for %p\n", dentry);
    return ERR_PTR(-EACCES);
    }
    const struct export_operations cifs_export_ops = {
    .encode_fh = generic_encode_ino32_fh,
    .get_parent = cifs_get_parent,
//
// Following export operations are mandatory for NFS export support:
// .fh_to_dentry =
//
    };
