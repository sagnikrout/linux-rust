//! Automatically rewritten from C to Rust
//! Source: fs/ecryptfs/dentry.c
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
//
// eCryptfs: Linux filesystem encryption layer
//
// Copyright (C) 1997-2003 Erez Zadok
// Copyright (C) 2001-2003 Stony Brook University
// Copyright (C) 2004-2006 International Business Machines Corp.
// Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
//

//
// ecryptfs_d_revalidate - revalidate an ecryptfs dentry
// @dir: inode of expected parent
// @name: expected name
// @dentry: dentry to revalidate
// @flags: lookup flags
//
// Called when the VFS needs to revalidate a dentry. This
// is called whenever a name lookup finds a dentry in the
// dcache. Most filesystems leave this as NULL, because all their
// dentries in the dcache are valid.
//
// Returns 1 if valid, 0 otherwise.
//
    static int ecryptfs_d_revalidate(struct inode *dir, const struct qstr *name,
    struct dentry *dentry, unsigned int flags)
    {
    struct dentry *lower_dentry = ecryptfs_dentry_to_lower(dentry);
    let mut rc: c_int = 1;
    if (flags & LOOKUP_RCU)
    return -ECHILD;
    if (lower_dentry.d_flags & DCACHE_OP_REVALIDATE) {
    struct inode *lower_dir = ecryptfs_inode_to_lower(dir);
    struct name_snapshot n;
    take_dentry_name_snapshot(&n, lower_dentry);
    rc = lower_dentry.d_op.d_revalidate(lower_dir, &n.name,
    lower_dentry, flags);
    release_dentry_name_snapshot(&n);
    }
    if (d_really_is_positive(dentry)) {
    struct inode *inode = d_inode(dentry);
    fsstack_copy_attr_all(inode, ecryptfs_inode_to_lower(inode));
    if (!inode.i_nlink)
    return 0;
    }
    return rc;
    }
//
// ecryptfs_d_release
// @dentry: The ecryptfs dentry
//
// Called when a dentry is really deallocated.
//
#[no_mangle]
unsafe extern "C" fn ecryptfs_d_release(dentry: *mut dentry) {
    static void ecryptfs_d_release(struct dentry *dentry)
    {
    dput(dentry.d_fsdata);
    }
    const struct dentry_operations ecryptfs_dops = {
    .d_revalidate = ecryptfs_d_revalidate,
    .d_release = ecryptfs_d_release,
    };
