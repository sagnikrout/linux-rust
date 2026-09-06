//! Automatically rewritten from C to Rust
//! Source: security/lockdown/lockdown.c
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


// SPDX-License-Identifier: GPL-2.0
// Lock down the kernel
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public Licence
// as published by the Free Software Foundation; either version
// 2 of the Licence, or (at your option) any later version.
//

    static enum lockdown_reason kernel_locked_down;
    static const enum lockdown_reason lockdown_levels[] = {LOCKDOWN_NONE,
    LOCKDOWN_INTEGRITY_MAX,
    LOCKDOWN_CONFIDENTIALITY_MAX};
//
// Put the kernel into lock-down mode.
//
#[no_mangle]
unsafe extern "C" fn lock_kernel_down(where: *const c_char, level: enum lockdown_reason) -> c_int {
    static int lock_kernel_down(const char *where, enum lockdown_reason level)
    {
    if (kernel_locked_down >= level)
    return -EPERM;
    kernel_locked_down = level;
    pr_notice("Kernel is locked down from %s; see man kernel_lockdown.7\n",
    where);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lockdown_param(level: *mut c_char) -> int __init {
    static int __init lockdown_param(char *level)
    {
    if (!level)
    return -EINVAL;
    if (strcmp(level, "integrity") == 0)
    lock_kernel_down("command line", LOCKDOWN_INTEGRITY_MAX);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(level, 0: "confidentiality") ==) -> else {
    else if (strcmp(level, "confidentiality") == 0)
    lock_kernel_down("command line", LOCKDOWN_CONFIDENTIALITY_MAX);
    else
    return -EINVAL;
    return 0;
    }
    early_param("lockdown", lockdown_param);
//
// lockdown_is_locked_down - Find out if the kernel is locked down
// @what: Tag to use in notice generated if lockdown is in effect
//
#[no_mangle]
unsafe extern "C" fn lockdown_is_locked_down(what: enum lockdown_reason) -> c_int {
    static int lockdown_is_locked_down(enum lockdown_reason what)
    {
    if (WARN(what >= LOCKDOWN_CONFIDENTIALITY_MAX,
    "Invalid lockdown reason"))
    return -EPERM;
    if (kernel_locked_down >= what) {
    if (lockdown_reasons[what])
    pr_notice_ratelimited("Lockdown: %s: %s is restricted; see man kernel_lockdown.7\n",
    current.comm, lockdown_reasons[what]);
    return -EPERM;
    }
    return 0;
    }
    static struct security_hook_list lockdown_hooks[] __ro_after_init = {
    LSM_HOOK_INIT(locked_down, lockdown_is_locked_down),
    };
    static const struct lsm_id lockdown_lsmid = {
    .name = "lockdown",
    .id = LSM_ID_LOCKDOWN,
    };
#[no_mangle]
unsafe extern "C" fn lockdown_lsm_init() -> int __init {
    static int __init lockdown_lsm_init(void)
    {

    lock_kernel_down("Kernel configuration", LOCKDOWN_INTEGRITY_MAX);

    lock_kernel_down("Kernel configuration", LOCKDOWN_CONFIDENTIALITY_MAX);

    security_add_hooks(lockdown_hooks, ARRAY_SIZE(lockdown_hooks),
    &lockdown_lsmid);
    return 0;
    }
    static ssize_t lockdown_read(struct file *filp, char __user *buf, size_t count,
    loff_t *ppos)
    {
    char temp[80] = "";
    int i, offset = 0;
    for (i = 0; i < ARRAY_SIZE(lockdown_levels); i++) {
    let mut level: enum lockdown_reason = lockdown_levels[i];
    if (lockdown_reasons[level]) {
    const char *label = lockdown_reasons[level];
    if (kernel_locked_down == level)
    offset += sprintf(temp+offset, "[%s] ", label);
    else
    offset += sprintf(temp+offset, "%s ", label);
    }
    }
// Convert the last space to a newline if needed.
    if (offset > 0)
    temp[offset-1] = '\n';
    return simple_read_from_buffer(buf, count, ppos, temp, strlen(temp));
    }
    static ssize_t lockdown_write(struct file *file, const char __user *buf,
    size_t n, loff_t *ppos)
    {
    char *state;
    int i, len, err = -EINVAL;
    state = memdup_user_nul(buf, n);
    if (IS_ERR(state))
    return PTR_ERR(state);
    len = strlen(state);
    if (len && state[len-1] == '\n') {
    state[len-1] = '\0';
    len--;
    }
    for (i = 0; i < ARRAY_SIZE(lockdown_levels); i++) {
    let mut level: enum lockdown_reason = lockdown_levels[i];
    const char *label = lockdown_reasons[level];
    if (label && !strcmp(state, label))
    err = lock_kernel_down("securityfs", level);
    }
    kfree(state);
    return err ? err : n;
    }
    static const struct file_operations lockdown_ops = {
    .read  = lockdown_read,
    .write = lockdown_write,
    };
#[no_mangle]
unsafe extern "C" fn lockdown_secfs_init() -> int __init {
    static int __init lockdown_secfs_init(void)
    {
    struct dentry *dentry;
    dentry = securityfs_create_file("lockdown", 0644, core::ptr::null_mut(), core::ptr::null_mut(),
    &lockdown_ops);
    return PTR_ERR_OR_ZERO(dentry);
    }

    DEFINE_EARLY_LSM(lockdown) = {

    DEFINE_LSM(lockdown) = {

    .id = &lockdown_lsmid,
    .init = lockdown_lsm_init,
    .initcall_core = lockdown_secfs_init,
    };
