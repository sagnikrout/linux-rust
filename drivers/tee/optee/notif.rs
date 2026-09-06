//! Automatically rewritten from C to Rust
//! Source: drivers/tee/optee/notif.c
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
// Copyright (c) 2015-2021, Linaro Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_entry {
    pub link: list_head,
    pub c: completion,
    pub key: u_int,
}

#[no_mangle]
unsafe extern "C" fn have_key(optee: *mut optee, key: u_int) -> bool {
    static bool have_key(struct optee *optee, u_int key)
    {
    struct notif_entry *entry;
    list_for_each_entry(entry, &optee.notif.db, link)
    if (entry.key == key)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_notif_wait(optee: *mut optee, key: u_int, timeout: u32) -> c_int {
    int optee_notif_wait(struct optee *optee, u_int key, u32 timeout)
    {
    unsigned long flags;
    struct notif_entry *entry;
    let mut rc: c_int = 0;
    if (key > optee.notif.max_key)
    return -EINVAL;
    entry = kmalloc_obj(*entry);
    if (!entry)
    return -ENOMEM;
    init_completion(&entry.c);
    entry.key = key;
    spin_lock_irqsave(&optee.notif.lock, flags);
//
// If the bit is already set it means that the key has already
// been posted and we must not wait.
//
    if (test_bit(key, optee.notif.bitmap)) {
    clear_bit(key, optee.notif.bitmap);
    goto out;
    }
//
// Check if someone is already waiting for this key. If there is
// it's a programming error.
//
    if (have_key(optee, key)) {
    rc = -EBUSY;
    goto out;
    }
    list_add_tail(&entry.link, &optee.notif.db);
//
// Unlock temporarily and wait for completion.
//
    spin_unlock_irqrestore(&optee.notif.lock, flags);
    if (timeout != 0) {
    if (!wait_for_completion_timeout(&entry.c, timeout))
    rc = -ETIMEDOUT;
    } else {
    wait_for_completion(&entry.c);
    }
    spin_lock_irqsave(&optee.notif.lock, flags);
    list_del(&entry.link);
    out:
    spin_unlock_irqrestore(&optee.notif.lock, flags);
    kfree(entry);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_notif_send(optee: *mut optee, key: u_int) -> c_int {
    int optee_notif_send(struct optee *optee, u_int key)
    {
    unsigned long flags;
    struct notif_entry *entry;
    if (key > optee.notif.max_key)
    return -EINVAL;
    spin_lock_irqsave(&optee.notif.lock, flags);
    list_for_each_entry(entry, &optee.notif.db, link)
    if (entry.key == key) {
    complete(&entry.c);
    goto out;
    }
// Only set the bit in case there where nobody waiting
    set_bit(key, optee.notif.bitmap);
    out:
    spin_unlock_irqrestore(&optee.notif.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_notif_init(optee: *mut optee, max_key: u_int) -> c_int {
    int optee_notif_init(struct optee *optee, u_int max_key)
    {
    spin_lock_init(&optee.notif.lock);
    INIT_LIST_HEAD(&optee.notif.db);
    optee.notif.bitmap = bitmap_zalloc(max_key, GFP_KERNEL);
    if (!optee.notif.bitmap)
    return -ENOMEM;
    optee.notif.max_key = max_key;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_notif_uninit(optee: *mut optee) {
    void optee_notif_uninit(struct optee *optee)
    {
    bitmap_free(optee.notif.bitmap);
    }
