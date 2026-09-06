//! Automatically rewritten from C to Rust
//! Source: fs/hfs/sysdep.c
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
// linux/fs/hfs/sysdep.c
//
// Copyright (C) 1996  Paul H. Hargrove
// (C) 2003 Ardis Technologies <roman@ardistech.com>
// This file may be distributed under the terms of the GNU General Public License.
//
// This file contains the code to do various system dependent things.
//

// dentry case-handling: just lowercase everything
    static int hfs_revalidate_dentry(struct inode *dir, const struct qstr *name,
    struct dentry *dentry, unsigned int flags)
    {
    struct inode *inode;
    int diff;
    if (flags & LOOKUP_RCU)
    return -ECHILD;
    inode = d_inode(dentry);
    if(!inode)
    return 1;
// fix up inode on a timezone change
    diff = sys_tz.tz_minuteswest * 60 - HFS_I(inode).tz_secondswest;
    if (diff) {
    let mut ts: timespec64 = inode_get_ctime(inode);
    inode_set_ctime(inode, ts.tv_sec + diff, ts.tv_nsec);
    ts = inode_get_atime(inode);
    inode_set_atime(inode, ts.tv_sec + diff, ts.tv_nsec);
    ts = inode_get_mtime(inode);
    inode_set_mtime(inode, ts.tv_sec + diff, ts.tv_nsec);
    HFS_I(inode).tz_secondswest += diff;
    }
    return 1;
    }
    const struct dentry_operations hfs_dentry_operations =
    {
    .d_revalidate	= hfs_revalidate_dentry,
    .d_hash		= hfs_hash_dentry,
    .d_compare	= hfs_compare_dentry,
    };
