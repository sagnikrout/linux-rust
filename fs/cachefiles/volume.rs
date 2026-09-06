//! Automatically rewritten from C to Rust
//! Source: fs/cachefiles/volume.c
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
// Volume handling.
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Allocate and set up a volume representation.  We make sure all the fanout
// directories are created and pinned.
//
#[no_mangle]
pub unsafe extern "C" fn cachefiles_acquire_volume(vcookie: *mut fscache_volume) {
    void cachefiles_acquire_volume(struct fscache_volume *vcookie)
    {
    struct cachefiles_volume *volume;
    struct cachefiles_cache *cache = vcookie.cache.cache_priv;
    const struct cred *saved_cred;
    struct dentry *vdentry, *fan;
    size_t len;
    char *name;
    let mut is_new: bool = false;
    int ret, n_accesses, i;
    _enter("");
    volume = kzalloc_obj(struct cachefiles_volume);
    if (!volume)
    return;
    volume.vcookie = vcookie;
    volume.cache = cache;
    INIT_LIST_HEAD(&volume.cache_link);
    cachefiles_begin_secure(cache, &saved_cred);
    len = vcookie.key[0];
    name = kmalloc(len + 3, GFP_NOFS);
    if (!name)
    goto error_vol;
    name[0] = 'I';
    memcpy(name + 1, vcookie.key + 1, len);
    name[len + 1] = 0;
    retry:
    vdentry = cachefiles_get_directory(cache, cache.store, name, &is_new);
    if (IS_ERR(vdentry))
    goto error_name;
    volume.dentry = vdentry;
    if (is_new) {
    if (!cachefiles_set_volume_xattr(volume))
    goto error_dir;
    } else {
    ret = cachefiles_check_volume_xattr(volume);
    if (ret < 0) {
    if (ret != -ESTALE)
    goto error_dir;
    vdentry = start_removing_dentry(cache.store, vdentry);
    if (!IS_ERR(vdentry))
    cachefiles_bury_object(cache, core::ptr::null_mut(), cache.store,
    vdentry,
    FSCACHE_VOLUME_IS_WEIRD);
    cachefiles_put_directory(volume.dentry);
    cond_resched();
    goto retry;
    }
    }
    for (i = 0; i < 256; i++) {
    sprintf(name, "@%02x", i);
    fan = cachefiles_get_directory(cache, vdentry, name, core::ptr::null_mut());
    if (IS_ERR(fan))
    goto error_fan;
    volume.fanout[i] = fan;
    }
    cachefiles_end_secure(cache, saved_cred);
    vcookie.cache_priv = volume;
    n_accesses = atomic_inc_return(&vcookie.n_accesses); /* Stop wakeups on dec-to-0 */
    trace_fscache_access_volume(vcookie.debug_id, 0,
    refcount_read(&vcookie.ref),
    n_accesses, fscache_access_cache_pin);
    spin_lock(&cache.object_list_lock);
    list_add(&volume.cache_link, &volume.cache.volumes);
    spin_unlock(&cache.object_list_lock);
    kfree(name);
    return;
    error_fan:
    for (i = 0; i < 256; i++)
    cachefiles_put_directory(volume.fanout[i]);
    error_dir:
    cachefiles_put_directory(volume.dentry);
    error_name:
    kfree(name);
    error_vol:
    kfree(volume);
    cachefiles_end_secure(cache, saved_cred);
    }
//
// Release a volume representation.
//
#[no_mangle]
unsafe extern "C" fn __cachefiles_free_volume(volume: *mut cachefiles_volume) {
    static void __cachefiles_free_volume(struct cachefiles_volume *volume)
    {
    int i;
    _enter("");
    volume.vcookie.cache_priv = core::ptr::null_mut();
    for (i = 0; i < 256; i++)
    cachefiles_put_directory(volume.fanout[i]);
    cachefiles_put_directory(volume.dentry);
    kfree(volume);
    }
#[no_mangle]
pub unsafe extern "C" fn cachefiles_free_volume(vcookie: *mut fscache_volume) {
    void cachefiles_free_volume(struct fscache_volume *vcookie)
    {
    struct cachefiles_volume *volume = vcookie.cache_priv;
    if (volume) {
    spin_lock(&volume.cache.object_list_lock);
    list_del_init(&volume.cache_link);
    spin_unlock(&volume.cache.object_list_lock);
    __cachefiles_free_volume(volume);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cachefiles_withdraw_volume(volume: *mut cachefiles_volume) {
    void cachefiles_withdraw_volume(struct cachefiles_volume *volume)
    {
    cachefiles_set_volume_xattr(volume);
    __cachefiles_free_volume(volume);
    }
