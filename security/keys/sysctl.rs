//! Automatically rewritten from C to Rust
//! Source: security/keys/sysctl.c
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
// Key management controls
//
// Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static const struct ctl_table key_sysctls[] = {
    {
    .procname = "maxkeys",
    .data = &key_quota_maxkeys,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ONE,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },
    {
    .procname = "maxbytes",
    .data = &key_quota_maxbytes,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ONE,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },
    {
    .procname = "root_maxkeys",
    .data = &key_quota_root_maxkeys,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ONE,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },
    {
    .procname = "root_maxbytes",
    .data = &key_quota_root_maxbytes,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ONE,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },
    {
    .procname = "gc_delay",
    .data = &key_gc_delay,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ZERO,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },

    {
    .procname = "persistent_keyring_expiry",
    .data = &persistent_keyring_expiry,
    .maxlen = sizeof(unsigned),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (void *) SYSCTL_ZERO,
    .extra2 = (void *) SYSCTL_INT_MAX,
    },

    };
#[no_mangle]
unsafe extern "C" fn init_security_keys_sysctls() -> int __init {
    static int __init init_security_keys_sysctls(void)
    {
    register_sysctl_init("kernel/keys", key_sysctls);
    return 0;
    }
    early_initcall(init_security_keys_sysctls);
