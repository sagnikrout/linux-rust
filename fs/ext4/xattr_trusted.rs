//! Automatically rewritten from C to Rust
//! Source: fs/ext4/xattr_trusted.c
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
// linux/fs/ext4/xattr_trusted.c
// Handler for trusted extended attributes.
//
// Copyright (C) 2003 by Andreas Gruenbacher, <a.gruenbacher@computer.org>
//

    static bool
    ext4_xattr_trusted_list(struct dentry *dentry)
    {
    return capable(CAP_SYS_ADMIN);
    }
    static int
    ext4_xattr_trusted_get(const struct xattr_handler *handler,
    struct dentry *unused, struct inode *inode,
    const char *name, void *buffer, size_t size)
    {
    return ext4_xattr_get(inode, EXT4_XATTR_INDEX_TRUSTED,
    name, buffer, size);
    }
    static int
    ext4_xattr_trusted_set(const struct xattr_handler *handler,
    struct mnt_idmap *idmap,
    struct dentry *unused, struct inode *inode,
    const char *name, const void *value,
    size_t size, int flags)
    {
    return ext4_xattr_set(inode, EXT4_XATTR_INDEX_TRUSTED,
    name, value, size, flags);
    }
    const struct xattr_handler ext4_xattr_trusted_handler = {
    .prefix	= XATTR_TRUSTED_PREFIX,
    .list	= ext4_xattr_trusted_list,
    .get	= ext4_xattr_trusted_get,
    .set	= ext4_xattr_trusted_set,
    };
