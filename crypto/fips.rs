//! Automatically rewritten from C to Rust
//! Source: crypto/fips.c
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
// FIPS 200 support.
//
// Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
//

    int fips_enabled;
    EXPORT_SYMBOL_GPL(fips_enabled);
    ATOMIC_NOTIFIER_HEAD(fips_fail_notif_chain);
    EXPORT_SYMBOL_GPL(fips_fail_notif_chain);
// Process kernel command-line parameter at boot time. fips=0 or fips=1
#[no_mangle]
unsafe extern "C" fn fips_enable(str: *mut c_char) -> int __init {
    static int __init fips_enable(char *str)
    {
    if (kstrtoint(str, 0, &fips_enabled))
    return 0;
    fips_enabled = !!fips_enabled;
    pr_info("fips mode: %s\n", str_enabled_disabled(fips_enabled));
    return 1;
    }
    __setup("fips=", fips_enable);

    static char fips_name[] = FIPS_MODULE_NAME;
    static char fips_version[] = FIPS_MODULE_VERSION;
    static const struct ctl_table crypto_sysctl_table[] = {
    {
    .procname	= "fips_enabled",
    .data		= &fips_enabled,
    .maxlen		= sizeof(int),
    .mode		= 0444,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "fips_name",
    .data		= &fips_name,
    .maxlen		= 64,
    .mode		= 0444,
    .proc_handler	= proc_dostring
    },
    {
    .procname	= "fips_version",
    .data		= &fips_version,
    .maxlen		= 64,
    .mode		= 0444,
    .proc_handler	= proc_dostring
    },
    };
    static struct ctl_table_header *crypto_sysctls;
#[no_mangle]
unsafe extern "C" fn crypto_proc_fips_init() {
    static void crypto_proc_fips_init(void)
    {
    crypto_sysctls = register_sysctl("crypto", crypto_sysctl_table);
    }
#[no_mangle]
unsafe extern "C" fn crypto_proc_fips_exit() {
    static void crypto_proc_fips_exit(void)
    {
    unregister_sysctl_table(crypto_sysctls);
    }
#[no_mangle]
pub unsafe extern "C" fn fips_fail_notify() {
    void fips_fail_notify(void)
    {
    if (fips_enabled)
    atomic_notifier_call_chain(&fips_fail_notif_chain, 0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(fips_fail_notify);
#[no_mangle]
unsafe extern "C" fn fips_init() -> int __init {
    static int __init fips_init(void)
    {
    crypto_proc_fips_init();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fips_exit() -> void __exit {
    static void __exit fips_exit(void)
    {
    crypto_proc_fips_exit();
    }
    module_init(fips_init);
    module_exit(fips_exit);
