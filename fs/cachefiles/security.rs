//! Automatically rewritten from C to Rust
//! Source: fs/cachefiles/security.c
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
// CacheFiles security management
//
// Copyright (C) 2007, 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// determine the security context within which we access the cache from within
// the kernel
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_get_security_ID(cache: *mut cachefiles_cache) -> c_int {
    int cachefiles_get_security_ID(struct cachefiles_cache *cache)
    {
    struct cred *new;
    int ret;
    _enter("{%u}", cache.have_secid ? cache.secid : 0);
    new = prepare_kernel_cred(current);
    if (!new) {
    ret = -ENOMEM;
    goto error;
    }
    if (cache.have_secid) {
    ret = set_security_override(new, cache.secid);
    if (ret < 0) {
    put_cred(new);
    pr_err("Security denies permission to nominate security context: error %d\n",
    ret);
    goto error;
    }
    }
    cache.cache_cred = new;
    ret = 0;
    error:
    _leave(" = %d", ret);
    return ret;
    }
//
// see if mkdir and create can be performed in the root directory
//
    static int cachefiles_check_cache_dir(struct cachefiles_cache *cache,
    struct dentry *root)
    {
    int ret;
    ret = security_inode_mkdir(d_backing_inode(root), root, 0);
    if (ret < 0) {
    pr_err("Security denies permission to make dirs: error %d",
    ret);
    return ret;
    }
    ret = security_inode_create(d_backing_inode(root), root, 0);
    if (ret < 0)
    pr_err("Security denies permission to create files: error %d",
    ret);
    return ret;
    }
//
// check the security details of the on-disk cache
// - must be called with security override in force
// - must return with a security override in force - even in the case of an
// error
//
    int cachefiles_determine_cache_security(struct cachefiles_cache *cache,
    struct dentry *root,
    const struct cred **_saved_cred)
    {
    struct cred *new;
    int ret;
    _enter("");
// duplicate the cache creds for COW (the override is currently in
// force, so we can use prepare_creds() to do this)
    new = prepare_creds();
    if (!new)
    return -ENOMEM;
    cachefiles_end_secure(cache, *_saved_cred);
// use the cache root dir's security context as the basis with
// which create files
    ret = set_create_files_as(new, d_backing_inode(root));
    if (ret < 0) {
    abort_creds(new);
    cachefiles_begin_secure(cache, _saved_cred);
    _leave(" = %d [cfa]", ret);
    return ret;
    }
    put_cred(cache.cache_cred);
    cache.cache_cred = new;
    cachefiles_begin_secure(cache, _saved_cred);
    ret = cachefiles_check_cache_dir(cache, root);
    if (ret == -EOPNOTSUPP)
    ret = 0;
    _leave(" = %d", ret);
    return ret;
    }
