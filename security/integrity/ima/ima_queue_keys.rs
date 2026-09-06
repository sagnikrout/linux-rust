//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_queue_keys.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019 Microsoft Corporation
//
// Author: Lakshmi Ramasubramanian (nramas@linux.microsoft.com)
//
// File: ima_queue_keys.c
// Enables deferred processing of keys
//

//
// Flag to indicate whether a key can be processed
// right away or should be queued for processing later.
//
    static bool ima_process_keys;
//
// To synchronize access to the list of keys that need to be measured
//
    static DEFINE_MUTEX(ima_keys_lock);
    static LIST_HEAD(ima_keys);
//
// If custom IMA policy is not loaded then keys queued up
// for measurement should be freed. This worker is used
// for handling this scenario.
//
    static long ima_key_queue_timeout = 300000; /* 5 Minutes */
    static void ima_keys_handler(struct work_struct *work);
    static DECLARE_DELAYED_WORK(ima_keys_delayed_work, ima_keys_handler);
    static bool timer_expired;
//
// This worker function frees keys that may still be
// queued up in case custom IMA policy was not loaded.
//
#[no_mangle]
unsafe extern "C" fn ima_keys_handler(work: *mut work_struct) {
    static void ima_keys_handler(struct work_struct *work)
    {
    timer_expired = true;
    ima_process_queued_keys();
    }
//
// This function sets up a worker to free queued keys in case
// custom IMA policy was never loaded.
//
#[no_mangle]
pub unsafe extern "C" fn ima_init_key_queue() {
    void ima_init_key_queue(void)
    {
    schedule_delayed_work(&ima_keys_delayed_work,
    msecs_to_jiffies(ima_key_queue_timeout));
    }
#[no_mangle]
unsafe extern "C" fn ima_free_key_entry(entry: *mut ima_key_entry) {
    static void ima_free_key_entry(struct ima_key_entry *entry)
    {
    if (entry) {
    kfree(entry.payload);
    kfree(entry.keyring_name);
    kfree(entry);
    }
    }
    static struct ima_key_entry *ima_alloc_key_entry(struct key *keyring,
    const void *payload,
    size_t payload_len)
    {
    let mut rc: c_int = 0;
    const char *audit_cause = "ENOMEM";
    struct ima_key_entry *entry;
    entry = kzalloc_obj(*entry);
    if (entry) {
    entry.payload = kmemdup(payload, payload_len, GFP_KERNEL);
    entry.keyring_name = kstrdup(keyring.description,
    GFP_KERNEL);
    entry.payload_len = payload_len;
    }
    if ((entry == core::ptr::null_mut()) || (entry.payload == core::ptr::null_mut()) ||
    (entry.keyring_name == core::ptr::null_mut())) {
    rc = -ENOMEM;
    goto out;
    }
    INIT_LIST_HEAD(&entry.list);
    out:
    if (rc) {
    integrity_audit_message(AUDIT_INTEGRITY_PCR, core::ptr::null_mut(),
    keyring.description,
    func_measure_str(KEY_CHECK),
    audit_cause, rc, 0, rc);
    ima_free_key_entry(entry);
    entry = core::ptr::null_mut();
    }
    return entry;
    }
    bool ima_queue_key(struct key *keyring, const void *payload,
    size_t payload_len)
    {
    let mut queued: bool = false;
    struct ima_key_entry *entry;
    entry = ima_alloc_key_entry(keyring, payload, payload_len);
    if (!entry)
    return false;
    mutex_lock(&ima_keys_lock);
    if (!ima_process_keys) {
    list_add_tail(&entry.list, &ima_keys);
    queued = true;
    }
    mutex_unlock(&ima_keys_lock);
    if (!queued)
    ima_free_key_entry(entry);
    return queued;
    }
//
// ima_process_queued_keys() - process keys queued for measurement
//
// This function sets ima_process_keys to true and processes queued keys.
// From here on keys will be processed right away (not queued).
//
#[no_mangle]
pub unsafe extern "C" fn ima_process_queued_keys() {
    void ima_process_queued_keys(void)
    {
    struct ima_key_entry *entry, *tmp;
    let mut process: bool = false;
    if (ima_process_keys)
    return;
//
// Since ima_process_keys is set to true, any new key will be
// processed immediately and not be queued to ima_keys list.
// First one setting the ima_process_keys flag to true will
// process the queued keys.
//
    mutex_lock(&ima_keys_lock);
    if (!ima_process_keys) {
    ima_process_keys = true;
    process = true;
    }
    mutex_unlock(&ima_keys_lock);
    if (!process)
    return;
    if (!timer_expired)
    cancel_delayed_work_sync(&ima_keys_delayed_work);
    list_for_each_entry_safe(entry, tmp, &ima_keys, list) {
    if (!timer_expired)
    process_buffer_measurement(&nop_mnt_idmap, core::ptr::null_mut(),
    entry.payload,
    entry.payload_len,
    entry.keyring_name,
    KEY_CHECK, 0,
    entry.keyring_name,
    false, core::ptr::null_mut(), 0);
    list_del(&entry.list);
    ima_free_key_entry(entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ima_should_queue_key() -> bool {
    inline bool ima_should_queue_key(void)
    {
    return !ima_process_keys;
    }
