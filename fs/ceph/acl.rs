//! Automatically rewritten from C to Rust
//! Source: fs/ceph/acl.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/fs/ceph/acl.c
//
// Copyright (C) 2013 Guangliang Zhao, <lucienchao@gmail.com>
//

    static inline void ceph_set_cached_acl(struct inode *inode,
    int type, struct posix_acl *acl)
    {
    struct ceph_inode_info *ci = ceph_inode(inode);
    spin_lock(&ci.i_ceph_lock);
    if (__ceph_caps_issued_mask_metric(ci, CEPH_CAP_XATTR_SHARED, 0))
    set_cached_acl(inode, type, acl);
    else
    forget_cached_acl(inode, type);
    spin_unlock(&ci.i_ceph_lock);
    }
    struct posix_acl *ceph_get_acl(struct inode *inode, int type, bool rcu)
    {
    struct ceph_client *cl = ceph_inode_to_client(inode);
    int size;
    let mut retry_cnt: c_uint = 0;
    const char *name;
    char *value = core::ptr::null_mut();
    struct posix_acl *acl;
    if (rcu)
    return ERR_PTR(-ECHILD);
    switch (type) {
    case ACL_TYPE_ACCESS:
    name = XATTR_NAME_POSIX_ACL_ACCESS;
    break;
    case ACL_TYPE_DEFAULT:
    name = XATTR_NAME_POSIX_ACL_DEFAULT;
    break;
    default:
    BUG();
    }
    retry:
    size = __ceph_getxattr(inode, name, "", 0);
    if (size > 0) {
    value = kzalloc(size, GFP_NOFS);
    if (!value)
    return ERR_PTR(-ENOMEM);
    size = __ceph_getxattr(inode, name, value, size);
    }
    if (size == -ERANGE && retry_cnt < 10) {
    retry_cnt++;
    kfree(value);
    value = core::ptr::null_mut();
    goto retry;
    }
    if (size > 0) {
    acl = posix_acl_from_xattr(&init_user_ns, value, size);
    } else if (size == -ENODATA || size == 0) {
    acl = core::ptr::null_mut();
    } else {
    pr_err_ratelimited_client(cl, "%llx.%llx failed, err=%d\n",
    ceph_vinop(inode), size);
    acl = ERR_PTR(-EIO);
    }
    kfree(value);
    if (!IS_ERR(acl))
    ceph_set_cached_acl(inode, type, acl);
    return acl;
    }
    int ceph_set_acl(struct mnt_idmap *idmap, struct dentry *dentry,
    struct posix_acl *acl, int type)
    {
    let mut ret: c_int = 0;
    let mut size: usize = 0;
    const char *name = core::ptr::null_mut();
    char *value = core::ptr::null_mut();
    struct iattr newattrs;
    struct inode *inode = d_inode(dentry);
    let mut old_ctime: timespec64 = inode_get_ctime(inode);
    let mut new_mode: umode_t = inode.i_mode, old_mode = inode.i_mode;
    if (ceph_snap(inode) != CEPH_NOSNAP) {
    ret = -EROFS;
    goto out;
    }
    switch (type) {
    case ACL_TYPE_ACCESS:
    name = XATTR_NAME_POSIX_ACL_ACCESS;
    if (acl) {
    ret = posix_acl_update_mode(idmap, inode,
    &new_mode, &acl);
    if (ret)
    goto out;
    }
    break;
    case ACL_TYPE_DEFAULT:
    if (!S_ISDIR(inode.i_mode)) {
    ret = acl ? -EINVAL : 0;
    goto out;
    }
    name = XATTR_NAME_POSIX_ACL_DEFAULT;
    break;
    default:
    ret = -EINVAL;
    goto out;
    }
    if (acl) {
    value = posix_acl_to_xattr(&init_user_ns, acl, &size, GFP_NOFS);
    if (!value) {
    ret = -ENOMEM;
    goto out;
    }
    }
    if (new_mode != old_mode) {
    newattrs.ia_ctime = current_time(inode);
    newattrs.ia_mode = new_mode;
    newattrs.ia_valid = ATTR_MODE | ATTR_CTIME;
    ret = __ceph_setattr(idmap, inode, &newattrs, core::ptr::null_mut());
    if (ret)
    goto out_free;
    }
    ret = __ceph_setxattr(inode, name, value, size, 0);
    if (ret) {
    if (new_mode != old_mode) {
    newattrs.ia_ctime = old_ctime;
    newattrs.ia_mode = old_mode;
    newattrs.ia_valid = ATTR_MODE | ATTR_CTIME;
    __ceph_setattr(idmap, inode, &newattrs, core::ptr::null_mut());
    }
    goto out_free;
    }
    ceph_set_cached_acl(inode, type, acl);
    out_free:
    kfree(value);
    out:
    return ret;
    }
    int ceph_pre_init_acls(struct inode *dir, umode_t *mode,
    struct ceph_acl_sec_ctx *as_ctx)
    {
    struct posix_acl *acl, *default_acl;
    let mut val_size1: usize = 0, val_size2 = 0;
    struct ceph_pagelist *pagelist = core::ptr::null_mut();
    void *tmp_buf1 = core::ptr::null_mut(), *tmp_buf2 = core::ptr::null_mut();
    int err;
    err = posix_acl_create(dir, mode, &default_acl, &acl);
    if (err)
    return err;
    if (acl) {
    err = posix_acl_equiv_mode(acl, mode);
    if (err < 0)
    goto out_err;
    if (err == 0) {
    posix_acl_release(acl);
    acl = core::ptr::null_mut();
    }
    }
    if (!default_acl && !acl)
    return 0;
    err = -ENOMEM;
    pagelist = ceph_pagelist_alloc(GFP_KERNEL);
    if (!pagelist)
    goto out_err;
    err = ceph_pagelist_reserve(pagelist, PAGE_SIZE);
    if (err)
    goto out_err;
    ceph_pagelist_encode_32(pagelist, acl && default_acl ? 2 : 1);
    if (acl) {
    let mut len: usize = strlen(XATTR_NAME_POSIX_ACL_ACCESS);
    err = -ENOMEM;
    tmp_buf1 = posix_acl_to_xattr(&init_user_ns, acl,
    &val_size1, GFP_KERNEL);
    if (!tmp_buf1)
    goto out_err;
    err = ceph_pagelist_reserve(pagelist, len + val_size1 + 8);
    if (err)
    goto out_err;
    ceph_pagelist_encode_string(pagelist, XATTR_NAME_POSIX_ACL_ACCESS,
    len);
    ceph_pagelist_encode_32(pagelist, val_size1);
    ceph_pagelist_append(pagelist, tmp_buf1, val_size1);
    }
    if (default_acl) {
    let mut len: usize = strlen(XATTR_NAME_POSIX_ACL_DEFAULT);
    err = -ENOMEM;
    tmp_buf2 = posix_acl_to_xattr(&init_user_ns, default_acl,
    &val_size2, GFP_KERNEL);
    if (!tmp_buf2)
    goto out_err;
    err = ceph_pagelist_reserve(pagelist, len + val_size2 + 8);
    if (err)
    goto out_err;
    ceph_pagelist_encode_string(pagelist,
    XATTR_NAME_POSIX_ACL_DEFAULT, len);
    ceph_pagelist_encode_32(pagelist, val_size2);
    ceph_pagelist_append(pagelist, tmp_buf2, val_size2);
    }
    kfree(tmp_buf1);
    kfree(tmp_buf2);
    as_ctx.acl = acl;
    as_ctx.default_acl = default_acl;
    as_ctx.pagelist = pagelist;
    return 0;
    out_err:
    posix_acl_release(acl);
    posix_acl_release(default_acl);
    kfree(tmp_buf1);
    kfree(tmp_buf2);
    if (pagelist)
    ceph_pagelist_release(pagelist);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_init_inode_acls(inode: *mut inode, as_ctx: *mut ceph_acl_sec_ctx) {
    void ceph_init_inode_acls(struct inode *inode, struct ceph_acl_sec_ctx *as_ctx)
    {
    if (!inode)
    return;
    ceph_set_cached_acl(inode, ACL_TYPE_ACCESS, as_ctx.acl);
    ceph_set_cached_acl(inode, ACL_TYPE_DEFAULT, as_ctx.default_acl);
    }
