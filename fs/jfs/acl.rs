//! Automatically rewritten from C to Rust
//! Source: fs/jfs/acl.c
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
// Copyright (C) International Business Machines  Corp., 2002-2004
// Copyright (C) Andreas Gruenbacher, 2001
// Copyright (C) Linus Torvalds, 1991, 1992
//

    struct posix_acl *jfs_get_acl(struct inode *inode, int type, bool rcu)
    {
    struct posix_acl *acl;
    char *ea_name;
    int size;
    char *value = core::ptr::null_mut();
    if (rcu)
    return ERR_PTR(-ECHILD);
    switch(type) {
    case ACL_TYPE_ACCESS:
    ea_name = XATTR_NAME_POSIX_ACL_ACCESS;
    break;
    case ACL_TYPE_DEFAULT:
    ea_name = XATTR_NAME_POSIX_ACL_DEFAULT;
    break;
    default:
    return ERR_PTR(-EINVAL);
    }
    size = __jfs_getxattr(inode, ea_name, core::ptr::null_mut(), 0);
    if (size > 0) {
    value = kmalloc(size, GFP_KERNEL);
    if (!value)
    return ERR_PTR(-ENOMEM);
    size = __jfs_getxattr(inode, ea_name, value, size);
    }
    if (size < 0) {
    if (size == -ENODATA)
    acl = core::ptr::null_mut();
    else
    acl = ERR_PTR(size);
    } else {
    acl = posix_acl_from_xattr(&init_user_ns, value, size);
    }
    kfree(value);
    return acl;
    }
    static int __jfs_set_acl(tid_t tid, struct inode *inode, int type,
    struct posix_acl *acl)
    {
    char *ea_name;
    int rc;
    let mut size: usize = 0;
    char *value = core::ptr::null_mut();
    switch (type) {
    case ACL_TYPE_ACCESS:
    ea_name = XATTR_NAME_POSIX_ACL_ACCESS;
    break;
    case ACL_TYPE_DEFAULT:
    ea_name = XATTR_NAME_POSIX_ACL_DEFAULT;
    break;
    default:
    return -EINVAL;
    }
    if (acl) {
    value = posix_acl_to_xattr(&init_user_ns, acl, &size, GFP_KERNEL);
    if (!value)
    return -ENOMEM;
    }
    rc = __jfs_setxattr(tid, inode, ea_name, value, size, 0);
    kfree(value);
    if (!rc)
    set_cached_acl(inode, type, acl);
    return rc;
    }
    int jfs_set_acl(struct mnt_idmap *idmap, struct dentry *dentry,
    struct posix_acl *acl, int type)
    {
    int rc;
    tid_t tid;
    let mut update_mode: c_int = 0;
    struct inode *inode = d_inode(dentry);
    let mut mode: umode_t = inode.i_mode;
    tid = txBegin(inode.i_sb, 0);
    mutex_lock(&JFS_IP(inode).commit_mutex);
    if (type == ACL_TYPE_ACCESS && acl) {
    rc = posix_acl_update_mode(&nop_mnt_idmap, inode, &mode, &acl);
    if (rc)
    goto end_tx;
    if (mode != inode.i_mode)
    update_mode = 1;
    }
    rc = __jfs_set_acl(tid, inode, type, acl);
    if (!rc) {
    if (update_mode) {
    inode.i_mode = mode;
    inode_set_ctime_current(inode);
    mark_inode_dirty(inode);
    }
    rc = txCommit(tid, 1, &inode, 0);
    }
    end_tx:
    txEnd(tid);
    mutex_unlock(&JFS_IP(inode).commit_mutex);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_init_acl(tid: tid_t, inode: *mut inode, dir: *mut inode) -> c_int {
    int jfs_init_acl(tid_t tid, struct inode *inode, struct inode *dir)
    {
    struct posix_acl *default_acl, *acl;
    let mut rc: c_int = 0;
    rc = posix_acl_create(dir, &inode.i_mode, &default_acl, &acl);
    if (rc)
    return rc;
    if (default_acl) {
    rc = __jfs_set_acl(tid, inode, ACL_TYPE_DEFAULT, default_acl);
    posix_acl_release(default_acl);
    } else {
    inode.i_default_acl = core::ptr::null_mut();
    }
    if (acl) {
    if (!rc)
    rc = __jfs_set_acl(tid, inode, ACL_TYPE_ACCESS, acl);
    posix_acl_release(acl);
    } else {
    inode.i_acl = core::ptr::null_mut();
    }
    JFS_IP(inode).mode2 = (JFS_IP(inode).mode2 & 0xffff0000) |
    inode.i_mode;
    return rc;
    }
