//! Automatically rewritten from C to Rust
//! Source: drivers/w1/w1_family.c
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
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

    DEFINE_SPINLOCK(w1_flock);
    static LIST_HEAD(w1_families);
//
// w1_register_family() - register a device family driver
// @newf:	family to register
//
#[no_mangle]
pub unsafe extern "C" fn w1_register_family(newf: *mut w1_family) -> c_int {
    int w1_register_family(struct w1_family *newf)
    {
    struct list_head *ent, *n;
    struct w1_family *f;
    let mut ret: c_int = 0;
    spin_lock(&w1_flock);
    list_for_each_safe(ent, n, &w1_families) {
    f = list_entry(ent, struct w1_family, family_entry);
    if (f.fid == newf.fid) {
    ret = -EEXIST;
    break;
    }
    }
    if (!ret) {
    atomic_set(&newf.refcnt, 0);
    list_add_tail(&newf.family_entry, &w1_families);
    }
    spin_unlock(&w1_flock);
// check default devices against the new set of drivers
    w1_reconnect_slaves(newf, 1);
    return ret;
    }
    EXPORT_SYMBOL(w1_register_family);
//
// w1_unregister_family() - unregister a device family driver
// @fent:	family to unregister
//
#[no_mangle]
pub unsafe extern "C" fn w1_unregister_family(fent: *mut w1_family) {
    void w1_unregister_family(struct w1_family *fent)
    {
    struct list_head *ent, *n;
    struct w1_family *f;
    spin_lock(&w1_flock);
    list_for_each_safe(ent, n, &w1_families) {
    f = list_entry(ent, struct w1_family, family_entry);
    if (f.fid == fent.fid) {
    list_del(&fent.family_entry);
    break;
    }
    }
    spin_unlock(&w1_flock);
// deatch devices using this family code
    w1_reconnect_slaves(fent, 0);
    while (atomic_read(&fent.refcnt)) {
    pr_info("Waiting for family %u to become free: refcnt=%d.\n",
    fent.fid, atomic_read(&fent.refcnt));
    if (msleep_interruptible(1000))
    flush_signals(current);
    }
    }
    EXPORT_SYMBOL(w1_unregister_family);
//
// Should be called under w1_flock held.
//
#[no_mangle]
pub unsafe extern "C" fn w1_family_registered(fid: u8) -> *mut w1_family {
    struct w1_family * w1_family_registered(u8 fid)
    {
    struct list_head *ent, *n;
    struct w1_family *f = core::ptr::null_mut();
    let mut ret: c_int = 0;
    list_for_each_safe(ent, n, &w1_families) {
    f = list_entry(ent, struct w1_family, family_entry);
    if (f.fid == fid) {
    ret = 1;
    break;
    }
    }
    return (ret) ? f : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __w1_family_put(f: *mut w1_family) {
    static void __w1_family_put(struct w1_family *f)
    {
    atomic_dec(&f.refcnt);
    }
#[no_mangle]
pub unsafe extern "C" fn w1_family_put(f: *mut w1_family) {
    void w1_family_put(struct w1_family *f)
    {
    spin_lock(&w1_flock);
    __w1_family_put(f);
    spin_unlock(&w1_flock);
    }

#[no_mangle]
pub unsafe extern "C" fn w1_family_get(f: *mut w1_family) {
    void w1_family_get(struct w1_family *f)
    {
    spin_lock(&w1_flock);
    __w1_family_get(f);
    spin_unlock(&w1_flock);
    }

#[no_mangle]
pub unsafe extern "C" fn __w1_family_get(f: *mut w1_family) {
    void __w1_family_get(struct w1_family *f)
    {
    smp_mb__before_atomic();
    atomic_inc(&f.refcnt);
    smp_mb__after_atomic();
    }
