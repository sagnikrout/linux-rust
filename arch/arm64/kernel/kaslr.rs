//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/kaslr.c
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
// Copyright (C) 2016 Linaro Ltd <ard.biesheuvel@linaro.org>
//

    let mut __kaslr_is_enabled: bool __ro_after_init = false;
#[no_mangle]
pub unsafe extern "C" fn kaslr_init() -> void __init {
    void __init kaslr_init(void)
    {
    if (kaslr_disabled_cmdline()) {
    pr_info("KASLR disabled on command line\n");
    return;
    }
//
// The KASLR offset modulo MIN_KIMG_ALIGN is taken from the physical
// placement of the image rather than from the seed, so a displacement
// of less than MIN_KIMG_ALIGN means that no seed was provided.
//
    if (kaslr_offset() < MIN_KIMG_ALIGN) {
    pr_warn("KASLR disabled due to lack of seed\n");
    return;
    }
    pr_info("KASLR enabled\n");
    __kaslr_is_enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn parse_nokaslr(unused: *mut c_char) -> int __init {
    static int __init parse_nokaslr(char *unused)
    {
// nokaslr param handling is done by early cpufeature code
    return 0;
    }
    early_param("nokaslr", parse_nokaslr);
