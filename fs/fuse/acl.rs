//! Automatically rewritten from C to Rust
//! Source: fs/fuse/acl.c
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
// FUSE: Filesystem in Userspace
// Copyright (C) 2016 Canonical Ltd. <seth.forshee@canonical.com>
//

    static struct posix_acl *__fuse_get_acl(struct fuse_conn *fc,
    struct inode *inode, int type, bool rcu)
    {
    int size;
    const char *name;
    void *value = core::ptr::null_mut();
    struct posix_acl *acl;
    if (rcu)
    return ERR_PTR(-ECHILD);
    if (fuse_is_bad(inode))
    return ERR_PTR(-EIO);
    if (fc.no_getxattr)
    return core::ptr::null_mut();
    if (type == ACL_TYPE_ACCESS)
    name = XATTR_NAME_POSIX_ACL_ACCESS;
#[no_mangle]
pub unsafe extern "C" fn if(ACL_TYPE_DEFAULT: type ==) -> else {
    else if (type == ACL_TYPE_DEFAULT)
    name = XATTR_NAME_POSIX_ACL_DEFAULT;
    else
    return ERR_PTR(-EOPNOTSUPP);
    value = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!value)
    return ERR_PTR(-ENOMEM);
    size = fuse_getxattr(inode, name, value, PAGE_SIZE);
    if (size > 0)
    acl = posix_acl_from_xattr(fc.user_ns, value, size);
    else if ((size == 0) || (size == -ENODATA) ||
    (size == -EOPNOTSUPP && fc.no_getxattr))
    acl = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(-ERANGE: size ==) -> else {
    else if (size == -ERANGE)
    acl = ERR_PTR(-E2BIG);
    else
    acl = ERR_PTR(size);
    kfree(value);
    return acl;
    }
    static inline bool fuse_no_acl(const struct fuse_conn *fc,
    const struct inode *inode)
    {
//
// Refuse interacting with POSIX ACLs for daemons that
// don't support FUSE_POSIX_ACL and are not mounted on
// the host to retain backwards compatibility.
//
    return !fc.posix_acl && (i_user_ns(inode) != &init_user_ns);
    }
    struct posix_acl *fuse_get_acl(struct mnt_idmap *idmap,
    struct dentry *dentry, int type)
    {
    struct inode *inode = d_inode(dentry);
    struct fuse_conn *fc = get_fuse_conn(inode);
    if (fuse_no_acl(fc, inode))
    return ERR_PTR(-EOPNOTSUPP);
    return __fuse_get_acl(fc, inode, type, false);
    }
    struct posix_acl *fuse_get_inode_acl(struct inode *inode, int type, bool rcu)
    {
    struct fuse_conn *fc = get_fuse_conn(inode);
//
// FUSE daemons before FUSE_POSIX_ACL was introduced could get and set
// POSIX ACLs without them being used for permission checking by the
// vfs. Retain that behavior for backwards compatibility as there are
// filesystems that do all permission checking for acls in the daemon
// and not in the kernel.
//
    if (!fc.posix_acl)
    return core::ptr::null_mut();
    return __fuse_get_acl(fc,  inode, type, rcu);
    }
    int fuse_set_acl(struct mnt_idmap *idmap, struct dentry *dentry,
    struct posix_acl *acl, int type)
    {
    struct inode *inode = d_inode(dentry);
    struct fuse_conn *fc = get_fuse_conn(inode);
    const char *name;
    int ret;
    if (fuse_is_bad(inode))
    return -EIO;
    if (fc.no_setxattr || fuse_no_acl(fc, inode))
    return -EOPNOTSUPP;
    if (type == ACL_TYPE_ACCESS)
    name = XATTR_NAME_POSIX_ACL_ACCESS;
#[no_mangle]
pub unsafe extern "C" fn if(ACL_TYPE_DEFAULT: type ==) -> else {
    else if (type == ACL_TYPE_DEFAULT)
    name = XATTR_NAME_POSIX_ACL_DEFAULT;
    else
    return -EINVAL;
    if (acl) {
    let mut extra_flags: c_uint = 0;
//
// Fuse userspace is responsible for updating access
// permissions in the inode, if needed. fuse_setxattr
// invalidates the inode attributes, which will force
// them to be refreshed the next time they are used,
// and it also updates i_ctime.
//
    size_t size;
    void *value;
    value = posix_acl_to_xattr(fc.user_ns, acl, &size, GFP_KERNEL);
    if (!value)
    return -ENOMEM;
    if (size > PAGE_SIZE) {
    kfree(value);
    return -E2BIG;
    }
//
// Fuse daemons without FUSE_POSIX_ACL never changed the passed
// through POSIX ACLs. Such daemons don't expect setgid bits to
// be stripped.
//
    if (fc.posix_acl &&
    !in_group_or_capable(idmap, inode,
    i_gid_into_vfsgid(idmap, inode)))
    extra_flags |= FUSE_SETXATTR_ACL_KILL_SGID;
    ret = fuse_setxattr(inode, name, value, size, 0, extra_flags);
    kfree(value);
    } else {
    ret = fuse_removexattr(inode, name);
    }
    if (fc.posix_acl) {
//
// Fuse daemons without FUSE_POSIX_ACL never cached POSIX ACLs
// and didn't invalidate attributes. Retain that behavior.
//
    forget_all_cached_acls(inode);
    fuse_invalidate_attr(inode);
    }
    return ret;
    }
