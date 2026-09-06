//! Automatically rewritten from C to Rust
//! Source: fs/hfsplus/xattr_user.c
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
// linux/fs/hfsplus/xattr_user.c
//
// Vyacheslav Dubeyko <slava@dubeyko.com>
//
// Handler for user extended attributes.
//

    static int hfsplus_user_getxattr(const struct xattr_handler *handler,
    struct dentry *unused, struct inode *inode,
    const char *name, void *buffer, size_t size)
    {
    return hfsplus_getxattr(inode, name, buffer, size,
    XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN);
    }
    static int hfsplus_user_setxattr(const struct xattr_handler *handler,
    struct mnt_idmap *idmap,
    struct dentry *unused, struct inode *inode,
    const char *name, const void *buffer,
    size_t size, int flags)
    {
    return hfsplus_setxattr(inode, name, buffer, size, flags,
    XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN);
    }
    const struct xattr_handler hfsplus_xattr_user_handler = {
    .prefix	= XATTR_USER_PREFIX,
    .get	= hfsplus_user_getxattr,
    .set	= hfsplus_user_setxattr,
    };
