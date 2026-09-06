//! Automatically rewritten from C to Rust
//! Source: fs/kernfs/symlink.c
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
// fs/kernfs/symlink.c - kernfs symlink implementation
//
// Copyright (c) 2001-3 Patrick Mochel
// Copyright (c) 2007 SUSE Linux Products GmbH
// Copyright (c) 2007, 2013 Tejun Heo <tj@kernel.org>
//

//
// kernfs_create_link - create a symlink
// @parent: directory to create the symlink in
// @name: name of the symlink
// @target: target node for the symlink to point to
//
// Return: the created node on success, ERR_PTR() value on error.
// Ownership of the link matches ownership of the target.
//
    struct kernfs_node *kernfs_create_link(struct kernfs_node *parent,
    const char *name,
    struct kernfs_node *target)
    {
    struct kernfs_node *kn;
    int error;
    let mut uid: kuid_t = GLOBAL_ROOT_UID;
    let mut gid: kgid_t = GLOBAL_ROOT_GID;
    if (target.iattr) {
    uid = target.iattr.ia_uid;
    gid = target.iattr.ia_gid;
    }
    kn = kernfs_new_node(parent, name, S_IFLNK|0777, uid, gid, KERNFS_LINK);
    if (!kn)
    return ERR_PTR(-ENOMEM);
    if (kernfs_ns_enabled(parent))
    kn.ns = target.ns;
    kn.symlink.target_kn = target;
    kernfs_get(target);	/* ref owned by symlink */
    error = kernfs_add_one(kn);
    if (!error)
    return kn;
    kernfs_put(kn);
    return ERR_PTR(error);
    }
    static int kernfs_get_target_path(struct kernfs_node *parent,
    struct kernfs_node *target, char *path)
    {
    struct kernfs_node *base, *kn;
    char *s = path;
    let mut len: c_int = 0;
// go up to the root, stop at the base
    base = parent;
    while (kernfs_parent(base)) {
    kn = kernfs_parent(target);
    while (kernfs_parent(kn) && base != kn)
    kn = kernfs_parent(kn);
    if (base == kn)
    break;
    if ((s - path) + 3 >= PATH_MAX)
    return -ENAMETOOLONG;
    memcpy(s, "../", 4);
    s += 3;
    base = kernfs_parent(base);
    }
// determine end of target string for reverse fillup
    kn = target;
    while (kernfs_parent(kn) && kn != base) {
    len += strlen(kernfs_rcu_name(kn)) + 1;
    kn = kernfs_parent(kn);
    }
// check limits
    if (len < 2)
    return -EINVAL;
    len--;
    if ((s - path) + len >= PATH_MAX)
    return -ENAMETOOLONG;
// reverse fillup of target string from target to base
    kn = target;
    while (kernfs_parent(kn) && kn != base) {
    const char *name = kernfs_rcu_name(kn);
    let mut slen: c_int = strlen(name);
    len -= slen;
    memcpy(s + len, name, slen);
    if (len)
    s[--len] = '/';
    kn = kernfs_parent(kn);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kernfs_getlink(inode: *mut inode, path: *mut c_char) -> c_int {
    static int kernfs_getlink(struct inode *inode, char *path)
    {
    struct kernfs_node *kn = inode.i_private;
    struct kernfs_node *parent;
    struct kernfs_node *target = kn.symlink.target_kn;
    struct kernfs_root *root = kernfs_root(kn);
    int error;
    down_read(&root.kernfs_rwsem);
    parent = kernfs_parent(kn);
    error = kernfs_get_target_path(parent, target, path);
    up_read(&root.kernfs_rwsem);
    return error;
    }
    static const char *kernfs_iop_get_link(struct dentry *dentry,
    struct inode *inode,
    struct delayed_call *done)
    {
    char *body;
    int error;
    if (!dentry)
    return ERR_PTR(-ECHILD);
    body = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if (!body)
    return ERR_PTR(-ENOMEM);
    error = kernfs_getlink(inode, body);
    if (unlikely(error < 0)) {
    kfree(body);
    return ERR_PTR(error);
    }
    set_delayed_call(done, kfree_link, body);
    return body;
    }
    const struct inode_operations kernfs_symlink_iops = {
    .listxattr	= kernfs_iop_listxattr,
    .get_link	= kernfs_iop_get_link,
    .setattr	= kernfs_iop_setattr,
    .getattr	= kernfs_iop_getattr,
    .permission	= kernfs_iop_permission,
    };
