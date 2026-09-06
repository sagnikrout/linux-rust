//! Automatically rewritten from C to Rust
//! Source: arch/x86/virt/svm/cmdline.c
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
// AMD SVM-SEV command line parsing support
//
// Copyright (C) 2023 - 2024 Advanced Micro Devices, Inc.
//
// Author: Michael Roth <michael.roth@amd.com>
//

    struct sev_config sev_cfg __read_mostly;
#[no_mangle]
unsafe extern "C" fn init_sev_config(str: *mut c_char) -> int __init {
    static int __init init_sev_config(char *str)
    {
    char *s;
    while ((s = strsep(&str, ","))) {
    if (!strcmp(s, "debug")) {
    sev_cfg.debug = true;
    continue;
    }
    if (!strcmp(s, "nosnp")) {
    if (!cpu_feature_enabled(X86_FEATURE_HYPERVISOR)) {
    setup_clear_cpu_cap(X86_FEATURE_SEV_SNP);
    cc_platform_clear(CC_ATTR_HOST_SEV_SNP);
    continue;
    } else {
    goto warn;
    }
    }
    warn:
    pr_info("SEV command-line option '%s' was not recognized\n", s);
    }
    return 1;
    }
    __setup("sev=", init_sev_config);
