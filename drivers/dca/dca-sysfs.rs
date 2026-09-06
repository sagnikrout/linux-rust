//! Automatically rewritten from C to Rust
//! Source: drivers/dca/dca-sysfs.c
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
// Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
//

    static const struct class dca_class = {
    .name = "dca",
    };
    static struct idr dca_idr;
    static spinlock_t dca_idr_lock;
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_add_req(dca: *mut dca_provider, dev: *mut device, slot: c_int) -> c_int {
    int dca_sysfs_add_req(struct dca_provider *dca, struct device *dev, int slot)
    {
    struct device *cd;
    static int req_count;
    cd = device_create(&dca_class, dca.cd, MKDEV(0, slot + 1), core::ptr::null_mut(),
    "requester%d", req_count++);
    return PTR_ERR_OR_ZERO(cd);
    }
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_remove_req(dca: *mut dca_provider, slot: c_int) {
    void dca_sysfs_remove_req(struct dca_provider *dca, int slot)
    {
    device_destroy(&dca_class, MKDEV(0, slot + 1));
    }
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_add_provider(dca: *mut dca_provider, dev: *mut device) -> c_int {
    int dca_sysfs_add_provider(struct dca_provider *dca, struct device *dev)
    {
    struct device *cd;
    int ret;
    idr_preload(GFP_KERNEL);
    spin_lock(&dca_idr_lock);
    ret = idr_alloc(&dca_idr, dca, 0, 0, GFP_NOWAIT);
    if (ret >= 0)
    dca.id = ret;
    spin_unlock(&dca_idr_lock);
    idr_preload_end();
    if (ret < 0)
    return ret;
    cd = device_create(&dca_class, dev, MKDEV(0, 0), core::ptr::null_mut(), "dca%d", dca.id);
    if (IS_ERR(cd)) {
    spin_lock(&dca_idr_lock);
    idr_remove(&dca_idr, dca.id);
    spin_unlock(&dca_idr_lock);
    return PTR_ERR(cd);
    }
    dca.cd = cd;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_remove_provider(dca: *mut dca_provider) {
    void dca_sysfs_remove_provider(struct dca_provider *dca)
    {
    device_unregister(dca.cd);
    dca.cd = core::ptr::null_mut();
    spin_lock(&dca_idr_lock);
    idr_remove(&dca_idr, dca.id);
    spin_unlock(&dca_idr_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_init() -> int __init {
    int __init dca_sysfs_init(void)
    {
    int err;
    idr_init(&dca_idr);
    spin_lock_init(&dca_idr_lock);
    err = class_register(&dca_class);
    if (err) {
    idr_destroy(&dca_idr);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dca_sysfs_exit() -> void __exit {
    void __exit dca_sysfs_exit(void)
    {
    class_unregister(&dca_class);
    idr_destroy(&dca_idr);
    }
