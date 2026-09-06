//! Automatically rewritten from C to Rust
//! Source: fs/cachefiles/error_inject.c
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
// Error injection handling.
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    unsigned int cachefiles_error_injection_state;
    static struct ctl_table_header *cachefiles_sysctl;
    static const struct ctl_table cachefiles_sysctls[] = {
    {
    .procname	= "error_injection",
    .data		= &cachefiles_error_injection_state,
    .maxlen		= sizeof(unsigned int),
    .mode		= 0644,
    .proc_handler	= proc_douintvec,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn cachefiles_register_error_injection() -> int __init {
    int __init cachefiles_register_error_injection(void)
    {
    cachefiles_sysctl = register_sysctl("cachefiles", cachefiles_sysctls);
    if (!cachefiles_sysctl)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cachefiles_unregister_error_injection() {
    void cachefiles_unregister_error_injection(void)
    {
    unregister_sysctl_table(cachefiles_sysctl);
    }
