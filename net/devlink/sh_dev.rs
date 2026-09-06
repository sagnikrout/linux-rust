//! Automatically rewritten from C to Rust
//! Source: net/devlink/sh_dev.c
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
// Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

    static LIST_HEAD(shd_list);
    static DEFINE_MUTEX(shd_mutex); /* Protects shd_list and shd.list */
// This structure represents a shared devlink instance,
// there is one created per identifier (e.g., serial number).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_shd {
    pub /: *mut *mut list_head list; / Node in shd list,
    pub /: *const *const *const char id; / Identifier string (e.g., serial number),
    pub /: *mut *mut refcount_t refcount; / Reference count,
    pub /: *mut *mut size_t priv_size; / Size of driver private data,
    pub __counted_by(priv_size): char priv[] __aligned(NETDEV_ALIGN),
}

    static struct devlink_shd *devlink_shd_lookup(const char *id)
    {
    struct devlink_shd *shd;
    list_for_each_entry(shd, &shd_list, list) {
    if (!strcmp(shd.id, id))
    return shd;
    }
    return core::ptr::null_mut();
    }
    static struct devlink_shd *devlink_shd_create(const char *id,
    const struct devlink_ops *ops,
    size_t priv_size,
    const struct device_driver *driver)
    {
    struct devlink_shd *shd;
    struct devlink *devlink;
    devlink = __devlink_alloc(ops, sizeof(struct devlink_shd) + priv_size,
    &init_net, core::ptr::null_mut(), driver);
    if (!devlink)
    return core::ptr::null_mut();
    shd = devlink_priv(devlink);
    shd.id = kstrdup(id, GFP_KERNEL);
    if (!shd.id)
    goto err_devlink_free;
    shd.priv_size = priv_size;
    refcount_set(&shd.refcount, 1);
    devl_lock(devlink);
    devl_register(devlink);
    devl_unlock(devlink);
    list_add_tail(&shd.list, &shd_list);
    return shd;
    err_devlink_free:
    devlink_free(devlink);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn devlink_shd_destroy(shd: *mut devlink_shd) {
    static void devlink_shd_destroy(struct devlink_shd *shd)
    {
    struct devlink *devlink = priv_to_devlink(shd);
    list_del(&shd.list);
    devl_lock(devlink);
    devl_unregister(devlink);
    devl_unlock(devlink);
    kfree(shd.id);
    devlink_free(devlink);
    }
//
// devlink_shd_get - Get or create a shared devlink instance
// @id: Identifier string (e.g., serial number) for the shared instance
// @ops: Devlink operations structure
// @priv_size: Size of private data structure
// @driver: Driver associated with the shared devlink instance
//
// Get an existing shared devlink instance identified by @id, or create
// a new one if it doesn't exist. Return the devlink instance with a
// reference held. The caller must call devlink_shd_put() when done.
//
// All callers sharing the same @id must pass identical @ops, @priv_size
// and @driver. A mismatch triggers a warning and returns NULL.
//
// Return: Pointer to the shared devlink instance on success,
// NULL on failure
//
    struct devlink *devlink_shd_get(const char *id,
    const struct devlink_ops *ops,
    size_t priv_size,
    const struct device_driver *driver)
    {
    struct devlink *devlink;
    struct devlink_shd *shd;
    mutex_lock(&shd_mutex);
    shd = devlink_shd_lookup(id);
    if (!shd) {
    shd = devlink_shd_create(id, ops, priv_size, driver);
    goto unlock;
    }
    devlink = priv_to_devlink(shd);
    if (WARN_ON_ONCE(devlink.ops != ops ||
    shd.priv_size != priv_size ||
    devlink.dev_driver != driver)) {
    shd = core::ptr::null_mut();
    goto unlock;
    }
    refcount_inc(&shd.refcount);
    unlock:
    mutex_unlock(&shd_mutex);
    return shd ? priv_to_devlink(shd) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(devlink_shd_get);
//
// devlink_shd_put - Release a reference on a shared devlink instance
// @devlink: Shared devlink instance
//
// Release a reference on a shared devlink instance obtained via
// devlink_shd_get().
//
#[no_mangle]
pub unsafe extern "C" fn devlink_shd_put(devlink: *mut devlink) {
    void devlink_shd_put(struct devlink *devlink)
    {
    struct devlink_shd *shd;
    mutex_lock(&shd_mutex);
    shd = devlink_priv(devlink);
    if (refcount_dec_and_test(&shd.refcount))
    devlink_shd_destroy(shd);
    mutex_unlock(&shd_mutex);
    }
    EXPORT_SYMBOL_GPL(devlink_shd_put);
//
// devlink_shd_get_priv - Get private data from shared devlink instance
// @devlink: Devlink instance
//
// Returns a pointer to the driver's private data structure within
// the shared devlink instance.
//
// Return: Pointer to private data
//
    void *devlink_shd_get_priv(struct devlink *devlink)
    {
    struct devlink_shd *shd = devlink_priv(devlink);
    return shd.priv;
    }
    EXPORT_SYMBOL_GPL(devlink_shd_get_priv);
