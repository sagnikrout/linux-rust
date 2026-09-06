//! Automatically rewritten from C to Rust
//! Source: fs/ext2/xattr_security.c
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
// linux/fs/ext2/xattr_security.c
// Handler for storing security labels as extended attributes.
//

    static int
    ext2_xattr_security_get(const struct xattr_handler *handler,
    struct dentry *unused, struct inode *inode,
    const char *name, void *buffer, size_t size)
    {
    return ext2_xattr_get(inode, EXT2_XATTR_INDEX_SECURITY, name,
    buffer, size);
    }
    static int
    ext2_xattr_security_set(const struct xattr_handler *handler,
    struct mnt_idmap *idmap,
    struct dentry *unused, struct inode *inode,
    const char *name, const void *value,
    size_t size, int flags)
    {
    return ext2_xattr_set(inode, EXT2_XATTR_INDEX_SECURITY, name,
    value, size, flags);
    }
    static int ext2_initxattrs(struct inode *inode, const struct xattr *xattr_array,
    void *fs_info)
    {
    const struct xattr *xattr;
    let mut err: c_int = 0;
    for (xattr = xattr_array; xattr.name != core::ptr::null_mut(); xattr++) {
    err = ext2_xattr_set(inode, EXT2_XATTR_INDEX_SECURITY,
    xattr.name, xattr.value,
    xattr.value_len, 0);
    if (err < 0)
    break;
    }
    return err;
    }
    int
    ext2_init_security(struct inode *inode, struct inode *dir,
    const struct qstr *qstr)
    {
    return security_inode_init_security(inode, dir, qstr,
    &ext2_initxattrs, core::ptr::null_mut());
    }
    const struct xattr_handler ext2_xattr_security_handler = {
    .prefix	= XATTR_SECURITY_PREFIX,
    .get	= ext2_xattr_security_get,
    .set	= ext2_xattr_security_set,
    };
