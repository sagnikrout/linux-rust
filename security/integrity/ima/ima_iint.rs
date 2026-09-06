//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_iint.c
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
// Copyright (C) 2008 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
//
// File: ima_iint.c
// - implements the IMA hook: ima_inode_free
// - cache integrity information in the inode security blob
//

    static struct kmem_cache *ima_iint_cache __ro_after_init;
//
// ima_iint_find - Return the iint associated with an inode
// @inode: Pointer to the inode
//
// Return the IMA integrity information (iint) associated with an inode, if the
// inode was processed by IMA.
//
// Return: Found iint or NULL.
//
    struct ima_iint_cache *ima_iint_find(struct inode *inode)
    {
    if (!IS_IMA(inode))
    return core::ptr::null_mut();
    return ima_inode_get_iint(inode);
    }

//
// It is not clear that IMA should be nested at all, but as long is it measures
// files both on overlayfs and on underlying fs, we need to annotate the iint
// mutex to avoid lockdep false positives related to IMA + overlayfs.
// See ovl_lockdep_annotate_inode_mutex_key() for more details.
//
    static inline void ima_iint_lockdep_annotate(struct ima_iint_cache *iint,
    struct inode *inode)
    {

    static struct lock_class_key ima_iint_mutex_key[IMA_MAX_NESTING];
    let mut depth: c_int = inode.i_sb.s_stack_depth;
    if (WARN_ON_ONCE(depth < 0 || depth >= IMA_MAX_NESTING))
    depth = 0;
    lockdep_set_class(&iint.mutex, &ima_iint_mutex_key[depth]);

    }
    static void ima_iint_init_always(struct ima_iint_cache *iint,
    struct inode *inode)
    {
    iint.ima_hash = core::ptr::null_mut();
    iint.real_inode.version = 0;
    iint.flags = 0UL;
    iint.atomic_flags = 0UL;
    iint.ima_file_status = INTEGRITY_UNKNOWN;
    iint.ima_mmap_status = INTEGRITY_UNKNOWN;
    iint.ima_bprm_status = INTEGRITY_UNKNOWN;
    iint.ima_read_status = INTEGRITY_UNKNOWN;
    iint.ima_creds_status = INTEGRITY_UNKNOWN;
    iint.measured_pcrs = 0;
    mutex_init(&iint.mutex);
    ima_iint_lockdep_annotate(iint, inode);
    }
#[no_mangle]
unsafe extern "C" fn ima_iint_free(iint: *mut ima_iint_cache) {
    static void ima_iint_free(struct ima_iint_cache *iint)
    {
    kfree(iint.ima_hash);
    mutex_destroy(&iint.mutex);
    kmem_cache_free(ima_iint_cache, iint);
    }
//
// ima_inode_get - Find or allocate an iint associated with an inode
// @inode: Pointer to the inode
//
// Find an iint associated with an inode, and allocate a new one if not found.
// Caller must lock i_mutex.
//
// Return: An iint on success, NULL on error.
//
    struct ima_iint_cache *ima_inode_get(struct inode *inode)
    {
    struct ima_iint_cache *iint;
    iint = ima_iint_find(inode);
    if (iint)
    return iint;
    iint = kmem_cache_alloc(ima_iint_cache, GFP_NOFS);
    if (!iint)
    return core::ptr::null_mut();
    ima_iint_init_always(iint, inode);
    inode.i_flags |= S_IMA;
    ima_inode_set_iint(inode, iint);
    return iint;
    }
//
// ima_inode_free_rcu - Called to free an inode via a RCU callback
// @inode_security: The inode->i_security pointer
//
// Free the IMA data associated with an inode.
//
#[no_mangle]
pub unsafe extern "C" fn ima_inode_free_rcu(inode_security: *mut c_void) {
    void ima_inode_free_rcu(void *inode_security)
    {
    struct ima_iint_cache **iint_p = inode_security + ima_blob_sizes.lbs_inode;
// *iint_p should be NULL if !IS_IMA(inode)
    if (*iint_p)
    ima_iint_free(*iint_p);
    }
#[no_mangle]
unsafe extern "C" fn ima_iint_init_once(foo: *mut c_void) {
    static void ima_iint_init_once(void *foo)
    {
    struct ima_iint_cache *iint = (struct ima_iint_cache *)foo;
    memset(iint, 0, sizeof(*iint));
    }
#[no_mangle]
pub unsafe extern "C" fn ima_iintcache_init() -> void __init {
    void __init ima_iintcache_init(void)
    {
    ima_iint_cache =
    kmem_cache_create("ima_iint_cache", sizeof(struct ima_iint_cache),
    0, SLAB_PANIC, ima_iint_init_once);
    }
