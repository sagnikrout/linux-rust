//! Automatically rewritten from C to Rust
//! Source: fs/9p/vfs_dentry.c
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
// This file contians vfs dentry ops for the 9P2000 protocol.
//
// Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

//
// v9fs_ndentry_is_expired - Check if negative dentry lookup has expired
//
// This should be called to know if a negative dentry should be removed from
// cache.
//
// @dentry: dentry in question
//
#[no_mangle]
unsafe extern "C" fn v9fs_ndentry_is_expired(dentry: *const dentry) -> bool {
    static bool v9fs_ndentry_is_expired(struct dentry const *dentry)
    {
    struct v9fs_session_info *v9ses = v9fs_dentry2v9ses(dentry);
    struct v9fs_dentry *v9fs_dentry = to_v9fs_dentry(dentry);
    if (v9ses.ndentry_timeout_ms == NDENTRY_TIMEOUT_NEVER)
    return false;
    return time_before_eq64(v9fs_dentry.expire_time, get_jiffies_64());
    }
//
// v9fs_ndentry_refresh_timeout - Refresh negative dentry lookup cache timeout
//
// This should be called when a look up yields a negative entry.
//
// @dentry: dentry in question
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_ndentry_refresh_timeout(dentry: *mut dentry) {
    void v9fs_ndentry_refresh_timeout(struct dentry *dentry)
    {
    struct v9fs_session_info *v9ses = v9fs_dentry2v9ses(dentry);
    struct v9fs_dentry *v9fs_dentry = to_v9fs_dentry(dentry);
    if (v9ses.ndentry_timeout_ms == NDENTRY_TIMEOUT_NEVER)
    return;
    v9fs_dentry.expire_time = get_jiffies_64() +
    msecs_to_jiffies(v9ses.ndentry_timeout_ms);
    }
//
// v9fs_cached_dentry_delete - called when dentry refcount equals 0
// @dentry:  dentry in question
//
#[no_mangle]
unsafe extern "C" fn v9fs_cached_dentry_delete(dentry: *const dentry) -> c_int {
    static int v9fs_cached_dentry_delete(const struct dentry *dentry)
    {
    p9_debug(P9_DEBUG_VFS, " dentry: %pd (%p)\n",
    dentry, dentry);
    if (!d_really_is_negative(dentry))
    return 0;
    return v9fs_ndentry_is_expired(dentry);
    }
#[no_mangle]
unsafe extern "C" fn __v9fs_dentry_fid_remove(dentry: *mut dentry) {
    static void __v9fs_dentry_fid_remove(struct dentry *dentry)
    {
    struct v9fs_dentry *v9fs_dentry = to_v9fs_dentry(dentry);
    struct hlist_node *p, *n;
    struct hlist_head head;
    p9_debug(P9_DEBUG_VFS, " dentry: %pd (%p)\n",
    dentry, dentry);
    spin_lock(&dentry.d_lock);
    hlist_move_list(&v9fs_dentry.head, &head);
    spin_unlock(&dentry.d_lock);
    hlist_for_each_safe(p, n, &head)
    p9_fid_put(hlist_entry(p, struct p9_fid, dlist));
    }
//
// v9fs_dentry_fid_remove - Release all dentry's fids
// @dentry: dentry in question
//
#[no_mangle]
pub unsafe extern "C" fn v9fs_dentry_fid_remove(dentry: *mut dentry) {
    void v9fs_dentry_fid_remove(struct dentry *dentry)
    {
    __v9fs_dentry_fid_remove(dentry);
    }
//
// v9fs_dentry_init - Initialize v9fs dentry data
// @dentry: dentry in question
//
#[no_mangle]
unsafe extern "C" fn v9fs_dentry_init(dentry: *mut dentry) -> c_int {
    static int v9fs_dentry_init(struct dentry *dentry)
    {
    struct v9fs_dentry *v9fs_dentry = kzalloc_obj(*v9fs_dentry);
    if (!v9fs_dentry)
    return -ENOMEM;
    INIT_HLIST_HEAD(&v9fs_dentry.head);
    dentry.d_fsdata = (void *)v9fs_dentry;
    return 0;
    }
//
// v9fs_dentry_release - called when dentry is going to be freed
// @dentry:  dentry that is being released
//
#[no_mangle]
unsafe extern "C" fn v9fs_dentry_release(dentry: *mut dentry) {
    static void v9fs_dentry_release(struct dentry *dentry)
    {
    struct v9fs_dentry *v9fs_dentry = to_v9fs_dentry(dentry);
    __v9fs_dentry_fid_remove(dentry);
    kfree_rcu(v9fs_dentry, rcu);
    }
#[no_mangle]
unsafe extern "C" fn __v9fs_lookup_revalidate(dentry: *mut dentry, flags: c_uint) -> c_int {
    static int __v9fs_lookup_revalidate(struct dentry *dentry, unsigned int flags)
    {
    struct p9_fid *fid;
    struct inode *inode;
    struct v9fs_inode *v9inode;
    if (flags & LOOKUP_RCU)
    return -ECHILD;
    inode = d_inode(dentry);
    if (!inode)
    return !v9fs_ndentry_is_expired(dentry);
    v9inode = V9FS_I(inode);
    if (v9inode.cache_validity & V9FS_INO_INVALID_ATTR) {
    int retval;
    struct v9fs_session_info *v9ses;
    fid = v9fs_fid_lookup(dentry);
    if (IS_ERR(fid)) {
    p9_debug(
    P9_DEBUG_VFS,
    "v9fs_fid_lookup: dentry = %pd (%p), got error %pe\n",
    dentry, dentry, fid);
    return PTR_ERR(fid);
    }
    v9ses = v9fs_inode2v9ses(inode);
    if (v9fs_proto_dotl(v9ses))
    retval = v9fs_refresh_inode_dotl(fid, inode);
    else
    retval = v9fs_refresh_inode(fid, inode);
    p9_fid_put(fid);
    if (retval == -ENOENT) {
    p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) invalidated due to ENOENT\n",
    dentry, dentry);
    return 0;
    }
    if (v9inode.cache_validity & V9FS_INO_INVALID_ATTR) {
    p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) invalidated due to type change\n",
    dentry, dentry);
    return 0;
    }
    if (retval < 0) {
    p9_debug(P9_DEBUG_VFS,
    "refresh inode: dentry = %pd (%p), got error %pe\n",
    dentry, dentry, ERR_PTR(retval));
    return retval;
    }
    }
    p9_debug(P9_DEBUG_VFS, "dentry: %pd (%p) is valid\n", dentry, dentry);
    return 1;
    }
    static int v9fs_lookup_revalidate(struct inode *dir, const struct qstr *name,
    struct dentry *dentry, unsigned int flags)
    {
    return __v9fs_lookup_revalidate(dentry, flags);
    }
#[no_mangle]
unsafe extern "C" fn v9fs_dentry_unalias_trylock(dentry: *const dentry) -> bool {
    static bool v9fs_dentry_unalias_trylock(const struct dentry *dentry)
    {
    struct v9fs_session_info *v9ses = v9fs_dentry2v9ses(dentry);
    return down_write_trylock(&v9ses.rename_sem);
    }
#[no_mangle]
unsafe extern "C" fn v9fs_dentry_unalias_unlock(dentry: *const dentry) {
    static void v9fs_dentry_unalias_unlock(const struct dentry *dentry)
    {
    struct v9fs_session_info *v9ses = v9fs_dentry2v9ses(dentry);
    up_write(&v9ses.rename_sem);
    }
    const struct dentry_operations v9fs_cached_dentry_operations = {
    .d_revalidate = v9fs_lookup_revalidate,
    .d_weak_revalidate = __v9fs_lookup_revalidate,
    .d_delete = v9fs_cached_dentry_delete,
    .d_init = v9fs_dentry_init,
    .d_release = v9fs_dentry_release,
    .d_unalias_trylock = v9fs_dentry_unalias_trylock,
    .d_unalias_unlock = v9fs_dentry_unalias_unlock,
    };
    const struct dentry_operations v9fs_dentry_operations = {
    .d_init = v9fs_dentry_init,
    .d_release = v9fs_dentry_release,
    .d_unalias_trylock = v9fs_dentry_unalias_trylock,
    .d_unalias_unlock = v9fs_dentry_unalias_unlock,
    };
