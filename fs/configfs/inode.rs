//! Automatically rewritten from C to Rust
//! Source: fs/configfs/inode.c
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
// inode.c - basic inode and dentry operations.
//
// Based on sysfs:
// sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
//
// configfs Copyright (C) 2005 Oracle.  All rights reserved.
//
// Please see Documentation/filesystems/configfs.rst for more
// information.
//

    static struct lock_class_key default_group_class[MAX_LOCK_DEPTH];

    static const struct inode_operations configfs_inode_operations ={
    .setattr	= configfs_setattr,
    };
    int configfs_setattr(struct mnt_idmap *idmap, struct dentry *dentry,
    struct iattr *iattr)
    {
    let mut inode: *mut inode = d_inode(dentry);
    let mut sd: *mut configfs_dirent = dentry.d_fsdata;
    struct iattr * sd_iattr;
    let mut ia_valid: c_uint = iattr.ia_valid;
    int error;
    if (!sd)
    return -EINVAL;
    sd_iattr = sd.s_iattr;
    if (!sd_iattr) {
// setting attributes for the first time, allocate now
    sd_iattr = kzalloc_obj(struct iattr);
    if (!sd_iattr)
    return -ENOMEM;
// assign default attributes
    sd_iattr.ia_mode = sd.s_mode;
    sd_iattr.ia_uid = GLOBAL_ROOT_UID;
    sd_iattr.ia_gid = GLOBAL_ROOT_GID;
    sd_iattr.ia_atime = sd_iattr.ia_mtime =
    sd_iattr.ia_ctime = current_time(inode);
    sd.s_iattr = sd_iattr;
    }
// attributes were changed atleast once in past
    error = simple_setattr(idmap, dentry, iattr);
    if (error)
    return error;
    if (ia_valid & ATTR_UID)
    sd_iattr.ia_uid = iattr.ia_uid;
    if (ia_valid & ATTR_GID)
    sd_iattr.ia_gid = iattr.ia_gid;
    if (ia_valid & ATTR_ATIME)
    sd_iattr.ia_atime = iattr.ia_atime;
    if (ia_valid & ATTR_MTIME)
    sd_iattr.ia_mtime = iattr.ia_mtime;
    if (ia_valid & ATTR_CTIME)
    sd_iattr.ia_ctime = iattr.ia_ctime;
    if (ia_valid & ATTR_MODE) {
    let mut mode: umode_t = iattr.ia_mode;
    if (!in_group_p(inode.i_gid) && !capable(CAP_FSETID))
    mode &= ~S_ISGID;
    sd_iattr.ia_mode = sd.s_mode = mode;
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn set_default_inode_attr(inode: *mut *mut inode, mode: umode_t) {
    static inline void set_default_inode_attr(struct inode * inode, umode_t mode)
    {
    inode.i_mode = mode;
    simple_inode_init_ts(inode);
    }
#[no_mangle]
pub unsafe extern "C" fn set_inode_attr(inode: *mut *mut inode, iattr: *mut *mut iattr) {
    static inline void set_inode_attr(struct inode * inode, struct iattr * iattr)
    {
    inode.i_mode = iattr.ia_mode;
    inode.i_uid = iattr.ia_uid;
    inode.i_gid = iattr.ia_gid;
    inode_set_atime_to_ts(inode, iattr.ia_atime);
    inode_set_mtime_to_ts(inode, iattr.ia_mtime);
    inode_set_ctime_to_ts(inode, iattr.ia_ctime);
    }
    struct inode *configfs_new_inode(umode_t mode, struct configfs_dirent *sd,
    struct super_block *s)
    {
    let mut inode: *mut inode = new_inode(s);
    if (inode) {
    inode.i_ino = get_next_ino();
    inode.i_mapping.a_ops = &ram_aops;
    inode.i_op = &configfs_inode_operations;
    if (sd.s_iattr) {
// sysfs_dirent has non-default attributes
// get them for the new inode from persistent copy
// in sysfs_dirent
//
    set_inode_attr(inode, sd.s_iattr);
    } else
    set_default_inode_attr(inode, mode);
    }
    return inode;
    }

    static void configfs_set_inode_lock_class(struct configfs_dirent *sd,
    struct inode *inode)
    {
    let mut depth: c_int = sd.s_depth;
    if (depth > 0) {
    if (depth <= ARRAY_SIZE(default_group_class)) {
    lockdep_set_class(&inode.i_rwsem,
    &default_group_class[depth - 1]);
    } else {
//
// In practice the maximum level of locking depth is
// already reached. Just inform about possible reasons.
//
    pr_info("Too many levels of inodes for the locking correctness validator.\n");
    pr_info("Spurious warnings may appear.\n");
    }
    }
    }

    static void configfs_set_inode_lock_class(struct configfs_dirent *sd,
    struct inode *inode)
    {
    }

    struct inode *configfs_create(struct dentry *dentry, umode_t mode)
    {
    struct inode *inode = core::ptr::null_mut();
    struct configfs_dirent *sd;
    if (!dentry)
    return ERR_PTR(-ENOENT);
    if (d_really_is_positive(dentry))
    return ERR_PTR(-EEXIST);
    sd = dentry.d_fsdata;
    inode = configfs_new_inode(mode, sd, dentry.d_sb);
    if (!inode)
    return ERR_PTR(-ENOMEM);
    configfs_set_inode_lock_class(sd, inode);
    return inode;
    }
//
// Get the name for corresponding element represented by the given configfs_dirent
//
#[no_mangle]
pub unsafe extern "C" fn configfs_get_name(sd: *mut configfs_dirent) -> *const c_uchar {
    const unsigned char * configfs_get_name(struct configfs_dirent *sd)
    {
    struct configfs_attribute *attr;
    BUG_ON(!sd || !sd.s_element);
// These always have a dentry, so use that
    if (sd.s_type & (CONFIGFS_DIR | CONFIGFS_ITEM_LINK))
    return sd.s_dentry.d_name.name;
    if (sd.s_type & (CONFIGFS_ITEM_ATTR | CONFIGFS_ITEM_BIN_ATTR)) {
    attr = sd.s_element;
    return attr.ca_name;
    }
    return core::ptr::null_mut();
    }
