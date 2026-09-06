//! Automatically rewritten from C to Rust
//! Source: security/tomoyo/load_policy.c
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
//
// security/tomoyo/load_policy.c
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

//
// Path to the policy loader. (default = CONFIG_SECURITY_TOMOYO_POLICY_LOADER)
//
    static const char *tomoyo_loader;
//
// tomoyo_loader_setup - Set policy loader.
//
// @str: Program to use as a policy loader (e.g. /sbin/tomoyo-init ).
//
// Returns 0.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_loader_setup(str: *mut c_char) -> int __init {
    static int __init tomoyo_loader_setup(char *str)
    {
    tomoyo_loader = str;
    return 1;
    }
    __setup("TOMOYO_loader=", tomoyo_loader_setup);
//
// tomoyo_policy_loader_exists - Check whether /sbin/tomoyo-init exists.
//
// Returns true if /sbin/tomoyo-init exists, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_policy_loader_exists() -> bool {
    static bool tomoyo_policy_loader_exists(void)
    {
    struct path path;
    if (!tomoyo_loader)
    tomoyo_loader = CONFIG_SECURITY_TOMOYO_POLICY_LOADER;
    if (kern_path(tomoyo_loader, LOOKUP_FOLLOW, &path)) {
    pr_info("Not activating Mandatory Access Control as %s does not exist.\n",
    tomoyo_loader);
    return false;
    }
    path_put(&path);
    return true;
    }
//
// Path to the trigger. (default = CONFIG_SECURITY_TOMOYO_ACTIVATION_TRIGGER)
//
    static const char *tomoyo_trigger;
//
// tomoyo_trigger_setup - Set trigger for activation.
//
// @str: Program to use as an activation trigger (e.g. /sbin/init ).
//
// Returns 0.
//
#[no_mangle]
unsafe extern "C" fn tomoyo_trigger_setup(str: *mut c_char) -> int __init {
    static int __init tomoyo_trigger_setup(char *str)
    {
    tomoyo_trigger = str;
    return 1;
    }
    __setup("TOMOYO_trigger=", tomoyo_trigger_setup);
//
// tomoyo_load_policy - Run external policy loader to load policy.
//
// @filename: The program about to start.
//
// This function checks whether @filename is /sbin/init , and if so
// invoke /sbin/tomoyo-init and wait for the termination of /sbin/tomoyo-init
// and then continues invocation of /sbin/init.
// /sbin/tomoyo-init reads policy files in /etc/tomoyo/ directory and
// writes to /sys/kernel/security/tomoyo/ interfaces.
//
// Returns nothing.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_load_policy(filename: *const c_char) {
    void tomoyo_load_policy(const char *filename)
    {
    static bool done;
    char *argv[2];
    char *envp[3];
    if (tomoyo_policy_loaded || done)
    return;
    if (!tomoyo_trigger)
    tomoyo_trigger = CONFIG_SECURITY_TOMOYO_ACTIVATION_TRIGGER;
    if (strcmp(filename, tomoyo_trigger))
    return;
    if (!tomoyo_policy_loader_exists())
    return;
    done = true;
    pr_info("Calling %s to load policy. Please wait.\n", tomoyo_loader);
    argv[0] = (char *) tomoyo_loader;
    argv[1] = core::ptr::null_mut();
    envp[0] = "HOME=/";
    envp[1] = "PATH=/sbin:/bin:/usr/sbin:/usr/bin";
    envp[2] = core::ptr::null_mut();
    call_usermodehelper(argv[0], argv, envp, UMH_WAIT_PROC);
    tomoyo_check_profile();
    }
