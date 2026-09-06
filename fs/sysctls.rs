//! Automatically rewritten from C to Rust
//! Source: fs/sysctls.c
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
// /proc/sys/fs shared sysctls
//
// These sysctls are shared between different filesystems.
//

    static const struct ctl_table fs_shared_sysctls[] = {
    {
    .procname	= "overflowuid",
    .data		= &fs_overflowuid,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_MAXOLDUID,
    },
    {
    .procname	= "overflowgid",
    .data		= &fs_overflowgid,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_MAXOLDUID,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_fs_sysctls() -> int __init {
    static int __init init_fs_sysctls(void)
    {
    register_sysctl_init("fs", fs_shared_sysctls);
    return 0;
    }
    early_initcall(init_fs_sysctls);
