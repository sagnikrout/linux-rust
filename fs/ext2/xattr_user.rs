//! Automatically rewritten from C to Rust
//! Source: fs/ext2/xattr_user.c
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
// linux/fs/ext2/xattr_user.c
// Handler for extended user attributes.
//
// Copyright (C) 2001 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
//

    static bool
    ext2_xattr_user_list(struct dentry *dentry)
    {
    return test_opt(dentry.d_sb, XATTR_USER);
    }
    static int
    ext2_xattr_user_get(const struct xattr_handler *handler,
    struct dentry *unused, struct inode *inode,
    const char *name, void *buffer, size_t size)
    {
    if (!test_opt(inode.i_sb, XATTR_USER))
    return -EOPNOTSUPP;
    return ext2_xattr_get(inode, EXT2_XATTR_INDEX_USER,
    name, buffer, size);
    }
    static int
    ext2_xattr_user_set(const struct xattr_handler *handler,
    struct mnt_idmap *idmap,
    struct dentry *unused, struct inode *inode,
    const char *name, const void *value,
    size_t size, int flags)
    {
    if (!test_opt(inode.i_sb, XATTR_USER))
    return -EOPNOTSUPP;
    return ext2_xattr_set(inode, EXT2_XATTR_INDEX_USER,
    name, value, size, flags);
    }
    const struct xattr_handler ext2_xattr_user_handler = {
    .prefix	= XATTR_USER_PREFIX,
    .list	= ext2_xattr_user_list,
    .get	= ext2_xattr_user_get,
    .set	= ext2_xattr_user_set,
    };
