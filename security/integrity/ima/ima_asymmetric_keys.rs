//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_asymmetric_keys.c
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
// File: ima_asymmetric_keys.c
// Defines an IMA hook to measure asymmetric keys on key
// create or update.
//

//
// ima_post_key_create_or_update - measure asymmetric keys
// @keyring: keyring to which the key is linked to
// @key: created or updated key
// @payload: The data used to instantiate or update the key.
// @payload_len: The length of @payload.
// @flags: key flags
// @create: flag indicating whether the key was created or updated
//
// Keys can only be measured, not appraised.
// The payload data used to instantiate or update the key is measured.
//
    void ima_post_key_create_or_update(struct key *keyring, struct key *key,
    const void *payload, size_t payload_len,
    unsigned long flags, bool create)
    {
    let mut queued: bool = false;
// Only asymmetric keys are handled by this hook.
    if (key.type != &key_type_asymmetric)
    return;
    if (!payload || (payload_len == 0))
    return;
    if (ima_should_queue_key())
    queued = ima_queue_key(keyring, payload, payload_len);
    if (queued)
    return;
//
// keyring->description points to the name of the keyring
// (such as ".builtin_trusted_keys", ".ima", etc.) to
// which the given key is linked to.
//
// The name of the keyring is passed in the "eventname"
// parameter to process_buffer_measurement() and is set
// in the "eventname" field in ima_event_data for
// the key measurement IMA event.
//
// The name of the keyring is also passed in the "keyring"
// parameter to process_buffer_measurement() to check
// if the IMA policy is configured to measure a key linked
// to the given keyring.
//
    process_buffer_measurement(&nop_mnt_idmap, core::ptr::null_mut(), payload, payload_len,
    keyring.description, KEY_CHECK, 0,
    keyring.description, false, core::ptr::null_mut(), 0);
    }
