//! Automatically rewritten from C to Rust
//! Source: fs/coda/cache.c
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
// Cache operations for Coda.
// For Linux 2.1: (C) 1997 Carnegie Mellon University
// For Linux 2.3: (C) 2000 Carnegie Mellon University
//
// Carnegie Mellon encourages users of this code to contribute improvements
// to the Coda project http://www.coda.cs.cmu.edu/ <coda@cs.cmu.edu>.
//

    let mut permission_epoch: static atomic_t = ATOMIC_INIT(0);
// replace or extend an acl cache hit
#[no_mangle]
pub unsafe extern "C" fn coda_cache_enter(inode: *mut inode, mask: c_int) {
    void coda_cache_enter(struct inode *inode, int mask)
    {
    struct coda_inode_info *cii = ITOC(inode);
    spin_lock(&cii.c_lock);
    cii.c_cached_epoch = atomic_read(&permission_epoch);
    if (!uid_eq(cii.c_uid, current_fsuid())) {
    cii.c_uid = current_fsuid();
    cii.c_cached_perm = mask;
    } else
    cii.c_cached_perm |= mask;
    spin_unlock(&cii.c_lock);
    }
// remove cached acl from an inode
#[no_mangle]
pub unsafe extern "C" fn coda_cache_clear_inode(inode: *mut inode) {
    void coda_cache_clear_inode(struct inode *inode)
    {
    struct coda_inode_info *cii = ITOC(inode);
    spin_lock(&cii.c_lock);
    cii.c_cached_epoch = atomic_read(&permission_epoch) - 1;
    spin_unlock(&cii.c_lock);
    }
// remove all acl caches
#[no_mangle]
pub unsafe extern "C" fn coda_cache_clear_all(sb: *mut super_block) {
    void coda_cache_clear_all(struct super_block *sb)
    {
    atomic_inc(&permission_epoch);
    }
// check if the mask has been matched against the acl already
#[no_mangle]
pub unsafe extern "C" fn coda_cache_check(inode: *mut inode, mask: c_int) -> c_int {
    int coda_cache_check(struct inode *inode, int mask)
    {
    struct coda_inode_info *cii = ITOC(inode);
    int hit;
    spin_lock(&cii.c_lock);
    hit = (mask & cii.c_cached_perm) == mask &&
    uid_eq(cii.c_uid, current_fsuid()) &&
    cii.c_cached_epoch == atomic_read(&permission_epoch);
    spin_unlock(&cii.c_lock);
    return hit;
    }
// Purging dentries and children
// The following routines drop dentries which are not
    in use and flag dentries which are in use to be
    zapped later.
    The flags are detected by:
    - coda_dentry_revalidate (for lookups) if the flag is C_PURGE
    - coda_dentry_delete: to remove dentry from the cache when d_count
    falls to zero
    - an inode method coda_revalidate (for attributes) if the
    flag is C_VATTR
//
// this won't do any harm: just flag all children
#[no_mangle]
unsafe extern "C" fn coda_flag_children(parent: *mut dentry, flag: c_int) {
    static void coda_flag_children(struct dentry *parent, int flag)
    {
    struct dentry *de;
    spin_lock(&parent.d_lock);
    rcu_read_lock();
    hlist_for_each_entry(de, &parent.d_children, d_sib) {
    struct inode *inode = d_inode_rcu(de);
// don't know what to do with negative dentries
    if (inode)
    coda_flag_inode(inode, flag);
    }
    rcu_read_unlock();
    spin_unlock(&parent.d_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn coda_flag_inode_children(inode: *mut inode, flag: c_int) {
    void coda_flag_inode_children(struct inode *inode, int flag)
    {
    struct dentry *alias_de;
    if ( !inode || !S_ISDIR(inode.i_mode))
    return;
    alias_de = d_find_alias(inode);
    if (!alias_de)
    return;
    coda_flag_children(alias_de, flag);
    shrink_dcache_parent(alias_de);
    dput(alias_de);
    }
