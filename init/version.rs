//! Automatically rewritten from C to Rust
//! Source: init/version.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/init/version.c
//
// Copyright (C) 1992  Theodore Ts'o
//
// May be freely distributed as part of Linux.
//

#[no_mangle]
unsafe extern "C" fn early_hostname(arg: *mut c_char) -> int __init {
    static int __init early_hostname(char *arg)
    {
    let mut bufsize: usize = sizeof(init_uts_ns.name.nodename);
    let mut maxlen: usize = bufsize - 1;
    ssize_t arglen;
    arglen = strscpy(init_uts_ns.name.nodename, arg, bufsize);
    if (arglen < 0) {
    pr_warn("hostname parameter exceeds %zd characters and will be truncated",
    maxlen);
    }
    return 0;
    }
    early_param("hostname", early_hostname);
    const char linux_proc_banner[] =
    "%s version %s"
    " (" LINUX_COMPILE_BY "@" LINUX_COMPILE_HOST ")"
    " (" LINUX_COMPILER ") %s\n";
    BUILD_SALT;
    BUILD_LTO_INFO;
//
// init_uts_ns and linux_banner contain the build version and timestamp,
// which are really fixed at the very last step of build process.
// They are compiled with __weak first, and without __weak later.
//
    struct uts_namespace init_uts_ns __weak;
    const char linux_banner[] __weak;

    EXPORT_SYMBOL_GPL(init_uts_ns);
