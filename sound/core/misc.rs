//! Automatically rewritten from C to Rust
//! Source: sound/core/misc.c
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
// Misc and compatibility things
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

#[no_mangle]
pub unsafe extern "C" fn release_and_free_resource(res: *mut resource) {
    void release_and_free_resource(struct resource *res)
    {
    if (res) {
    release_resource(res);
    kfree(res);
    }
    }
    EXPORT_SYMBOL(release_and_free_resource);

//
// snd_pci_quirk_lookup_id - look up a PCI SSID quirk list
// @vendor: PCI SSV id
// @device: PCI SSD id
// @list: quirk list, terminated by a null entry
//
// Look through the given quirk list and finds a matching entry
// with the same PCI SSID.  When subdevice is 0, all subdevice
// values may match.
//
// Returns the matched entry pointer, or NULL if nothing matched.
//
    const struct snd_pci_quirk *
    snd_pci_quirk_lookup_id(u16 vendor, u16 device,
    const struct snd_pci_quirk *list)
    {
    const struct snd_pci_quirk *q;
    for (q = list; q.subvendor || q.subdevice; q++) {
    if (q.subvendor != vendor)
    continue;
    if (!q.subdevice ||
    (device & q.subdevice_mask) == q.subdevice)
    return q;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(snd_pci_quirk_lookup_id);
//
// snd_pci_quirk_lookup - look up a PCI SSID quirk list
// @pci: pci_dev handle
// @list: quirk list, terminated by a null entry
//
// Look through the given quirk list and finds a matching entry
// with the same PCI SSID.  When subdevice is 0, all subdevice
// values may match.
//
// Returns the matched entry pointer, or NULL if nothing matched.
//
    const struct snd_pci_quirk *
    snd_pci_quirk_lookup(struct pci_dev *pci, const struct snd_pci_quirk *list)
    {
    if (!pci)
    return core::ptr::null_mut();
    return snd_pci_quirk_lookup_id(pci.subsystem_vendor,
    pci.subsystem_device,
    list);
    }
    EXPORT_SYMBOL(snd_pci_quirk_lookup);

//
// Deferred async signal helpers
//
// Below are a few helper functions to wrap the async signal handling
// in the deferred work.  The main purpose is to avoid the messy deadlock
// around tasklist_lock and co at the kill_fasync() invocation.
// fasync_helper() and kill_fasync() are replaced with snd_fasync_helper()
// and snd_kill_fasync(), respectively.  In addition, snd_fasync_free() has
// to be called at releasing the relevant file object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_fasync {
    pub fasync: *mut fasync_struct,
    pub signal: c_int,
    pub poll: c_int,
    pub on: c_int,
    pub list: list_head,
}

    static DEFINE_SPINLOCK(snd_fasync_lock);
    static LIST_HEAD(snd_fasync_list);
#[no_mangle]
unsafe extern "C" fn snd_fasync_work_fn(work: *mut work_struct) {
    static void snd_fasync_work_fn(struct work_struct *work)
    {
    struct snd_fasync *fasync;
    int signal, poll;
    spin_lock_irq(&snd_fasync_lock);
    while (!list_empty(&snd_fasync_list)) {
    fasync = list_first_entry(&snd_fasync_list, struct snd_fasync, list);
    list_del_init(&fasync.list);
    if (!fasync.on)
    continue;
    signal = fasync.signal;
    poll = fasync.poll;
    spin_unlock_irq(&snd_fasync_lock);
    kill_fasync(&fasync.fasync, signal, poll);
    spin_lock_irq(&snd_fasync_lock);
    }
    spin_unlock_irq(&snd_fasync_lock);
    }
    static DECLARE_WORK(snd_fasync_work, snd_fasync_work_fn);
    int snd_fasync_helper(int fd, struct file *file, int on,
    struct snd_fasync **fasyncp)
    {
    struct snd_fasync *fasync = core::ptr::null_mut();
    if (on) {
    fasync = kzalloc_obj(*fasync);
    if (!fasync)
    return -ENOMEM;
    INIT_LIST_HEAD(&fasync.list);
    }
    scoped_guard(spinlock_irq, &snd_fasync_lock) {
    if (*fasyncp) {
    kfree(fasync);
    fasync = *fasyncp;
    } else {
    if (!fasync)
    return 0;
// fasyncp = fasync;
    }
    fasync.on = on;
    }
    return fasync_helper(fd, file, on, &fasync.fasync);
    }
    EXPORT_SYMBOL_GPL(snd_fasync_helper);
#[no_mangle]
pub unsafe extern "C" fn snd_kill_fasync(fasync: *mut snd_fasync, signal: c_int, poll: c_int) {
    void snd_kill_fasync(struct snd_fasync *fasync, int signal, int poll)
    {
    if (!fasync)
    return;
    guard(spinlock_irqsave)(&snd_fasync_lock);
    if (!fasync.on)
    return;
    fasync.signal = signal;
    fasync.poll = poll;
    list_move(&fasync.list, &snd_fasync_list);
    schedule_work(&snd_fasync_work);
    }
    EXPORT_SYMBOL_GPL(snd_kill_fasync);
#[no_mangle]
pub unsafe extern "C" fn snd_fasync_free(fasync: *mut snd_fasync) {
    void snd_fasync_free(struct snd_fasync *fasync)
    {
    if (!fasync)
    return;
    scoped_guard(spinlock_irq, &snd_fasync_lock) {
    fasync.on = 0;
    list_del_init(&fasync.list);
    }
    flush_work(&snd_fasync_work);
    kfree(fasync);
    }
    EXPORT_SYMBOL_GPL(snd_fasync_free);
//
// generic refcount helper
//
#[no_mangle]
pub unsafe extern "C" fn snd_refcount_init(ref: *mut snd_refcount) {
    void snd_refcount_init(struct snd_refcount *ref)
    {
    atomic_set(&ref.count, 0);
    init_waitqueue_head(&ref.waiter);
    }
    EXPORT_SYMBOL_GPL(snd_refcount_init);
#[no_mangle]
pub unsafe extern "C" fn snd_refcount_put(ref: *mut snd_refcount) {
    void snd_refcount_put(struct snd_refcount *ref)
    {
    if (atomic_dec_and_test(&ref.count))
    wake_up(&ref.waiter);
    }
    EXPORT_SYMBOL_GPL(snd_refcount_put);
#[no_mangle]
pub unsafe extern "C" fn snd_refcount_sync(ref: *mut snd_refcount) {
    void snd_refcount_sync(struct snd_refcount *ref)
    {
    wait_event(ref.waiter, !atomic_read(&ref.count));
    }
    EXPORT_SYMBOL_GPL(snd_refcount_sync);
