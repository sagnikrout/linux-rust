//! Automatically rewritten from C to Rust
//! Source: fs/verity/init.c
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
// fs-verity module initialization and logging
//
// Copyright 2019 Google LLC
//
// Macro flag: #define CREATE_TRACE_POINTS

    static const struct ctl_table fsverity_sysctl_table[] = {

    {
    .procname       = "require_signatures",
    .data           = &fsverity_require_signatures,
    .maxlen         = sizeof(int),
    .mode           = 0644,
    .proc_handler   = proc_dointvec_minmax,
    .extra1         = SYSCTL_ZERO,
    .extra2         = SYSCTL_ONE,
    },

    };
#[no_mangle]
unsafe extern "C" fn fsverity_init_sysctl() -> void __init {
    static void __init fsverity_init_sysctl(void)
    {
    register_sysctl_init("fs/verity", fsverity_sysctl_table);
    }

#[no_mangle]
pub unsafe extern "C" fn fsverity_init_sysctl() {
    static inline void fsverity_init_sysctl(void)
    {
    }

    void fsverity_msg(const struct inode *inode, const char *level,
    const char *fmt, ...)
    {
    static DEFINE_RATELIMIT_STATE(rs, DEFAULT_RATELIMIT_INTERVAL,
    DEFAULT_RATELIMIT_BURST);
    struct va_format vaf;
    va_list args;
    if (!__ratelimit(&rs))
    return;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    if (inode)
    printk("%sfs-verity (%s, inode %llu): %pV\n",
    level, inode.i_sb.s_id, inode.i_ino, &vaf);
    else
    printk("%sfs-verity: %pV\n", level, &vaf);
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn fsverity_init() -> int __init {
    static int __init fsverity_init(void)
    {
    fsverity_check_hash_algs();
    fsverity_init_info_cache();
    fsverity_init_workqueue();
    fsverity_init_sysctl();
    fsverity_init_signature();
    fsverity_init_bpf();
    return 0;
    }
    late_initcall(fsverity_init)
