//! Automatically rewritten from C to Rust
//! Source: security/keys/user_defined.c
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
// user_defined.c: user defined key type
//
// Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static int logon_vet_description(const char *desc);
//
// user defined keys take an arbitrary string as the description and an
// arbitrary blob of data as the payload
//
    struct key_type key_type_user = {
    .name			= "user",
    .preparse		= user_preparse,
    .free_preparse		= user_free_preparse,
    .instantiate		= generic_key_instantiate,
    .update			= user_update,
    .revoke			= user_revoke,
    .destroy		= user_destroy,
    .describe		= user_describe,
    .read			= user_read,
    };
    EXPORT_SYMBOL_GPL(key_type_user);
//
// This key type is essentially the same as key_type_user, but it does
// not define a .read op. This is suitable for storing username and
// password pairs in the keyring that you do not want to be readable
// from userspace.
//
    struct key_type key_type_logon = {
    .name			= "logon",
    .preparse		= user_preparse,
    .free_preparse		= user_free_preparse,
    .instantiate		= generic_key_instantiate,
    .update			= user_update,
    .revoke			= user_revoke,
    .destroy		= user_destroy,
    .describe		= user_describe,
    .vet_description	= logon_vet_description,
    };
    EXPORT_SYMBOL_GPL(key_type_logon);
//
// Preparse a user defined key payload
//
#[no_mangle]
pub unsafe extern "C" fn user_preparse(prep: *mut key_preparsed_payload) -> c_int {
    int user_preparse(struct key_preparsed_payload *prep)
    {
    struct user_key_payload *upayload;
    let mut datalen: usize = prep.datalen;
    if (datalen == 0 || datalen > 32767 || !prep.data)
    return -EINVAL;
    upayload = kmalloc_flex(*upayload, data, datalen);
    if (!upayload)
    return -ENOMEM;
// attach the data
    prep.quotalen = datalen;
    prep.payload.data[0] = upayload;
    upayload.datalen = datalen;
    memcpy(upayload.data, prep.data, datalen);
    return 0;
    }
    EXPORT_SYMBOL_GPL(user_preparse);
//
// Free a preparse of a user defined key payload
//
#[no_mangle]
pub unsafe extern "C" fn user_free_preparse(prep: *mut key_preparsed_payload) {
    void user_free_preparse(struct key_preparsed_payload *prep)
    {
    kfree_sensitive(prep.payload.data[0]);
    }
    EXPORT_SYMBOL_GPL(user_free_preparse);
#[no_mangle]
unsafe extern "C" fn user_free_payload_rcu(head: *mut rcu_head) {
    static void user_free_payload_rcu(struct rcu_head *head)
    {
    struct user_key_payload *payload;
    payload = container_of(head, struct user_key_payload, rcu);
    kfree_sensitive(payload);
    }
//
// update a user defined key
// - the key's semaphore is write-locked
//
#[no_mangle]
pub unsafe extern "C" fn user_update(key: *mut key, prep: *mut key_preparsed_payload) -> c_int {
    int user_update(struct key *key, struct key_preparsed_payload *prep)
    {
    struct user_key_payload *zap = core::ptr::null_mut();
    int ret;
// check the quota and attach the new data
    ret = key_payload_reserve(key, prep.datalen);
    if (ret < 0)
    return ret;
// attach the new data, displacing the old
    key.expiry = prep.expiry;
    if (key_is_positive(key))
    zap = dereference_key_locked(key);
    rcu_assign_keypointer(key, prep.payload.data[0]);
    prep.payload.data[0] = core::ptr::null_mut();
    if (zap)
    call_rcu(&zap.rcu, user_free_payload_rcu);
    return ret;
    }
    EXPORT_SYMBOL_GPL(user_update);
//
// dispose of the links from a revoked keyring
// - called with the key sem write-locked
//
#[no_mangle]
pub unsafe extern "C" fn user_revoke(key: *mut key) {
    void user_revoke(struct key *key)
    {
    struct user_key_payload *upayload = user_key_payload_locked(key);
// clear the quota
    key_payload_reserve(key, 0);
    if (upayload) {
    rcu_assign_keypointer(key, core::ptr::null_mut());
    call_rcu(&upayload.rcu, user_free_payload_rcu);
    }
    }
    EXPORT_SYMBOL(user_revoke);
//
// dispose of the data dangling from the corpse of a user key
//
#[no_mangle]
pub unsafe extern "C" fn user_destroy(key: *mut key) {
    void user_destroy(struct key *key)
    {
    struct user_key_payload *upayload = key.payload.data[0];
    kfree_sensitive(upayload);
    }
    EXPORT_SYMBOL_GPL(user_destroy);
//
// describe the user key
//
#[no_mangle]
pub unsafe extern "C" fn user_describe(key: *const key, m: *mut seq_file) {
    void user_describe(const struct key *key, struct seq_file *m)
    {
    seq_puts(m, key.description);
    if (key_is_positive(key))
    seq_printf(m, ": %u", key.datalen);
    }
    EXPORT_SYMBOL_GPL(user_describe);
//
// read the key data
// - the key's semaphore is read-locked
//
#[no_mangle]
pub unsafe extern "C" fn user_read(key: *const key, buffer: *mut c_char, buflen: usize) -> c_long {
    long user_read(const struct key *key, char *buffer, size_t buflen)
    {
    const struct user_key_payload *upayload;
    long ret;
    upayload = user_key_payload_locked(key);
    ret = upayload.datalen;
// we can return the data as is
    if (buffer && buflen > 0) {
    if (buflen > upayload.datalen)
    buflen = upayload.datalen;
    memcpy(buffer, upayload.data, buflen);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(user_read);
// Vet the description for a "logon" key
#[no_mangle]
unsafe extern "C" fn logon_vet_description(desc: *const c_char) -> c_int {
    static int logon_vet_description(const char *desc)
    {
    char *p;
// require a "qualified" description string
    p = strchr(desc, ':');
    if (!p)
    return -EINVAL;
// also reject description with ':' as first char
    if (p == desc)
    return -EINVAL;
    return 0;
    }
