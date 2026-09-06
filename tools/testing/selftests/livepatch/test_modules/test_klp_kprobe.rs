//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_modules/test_klp_kprobe.c
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
// Copyright (C) 2024 Marcos Paulo de Souza <mpdesouza@suse.com>
// Copyright (C) 2024 Michael Vetter <mvetter@suse.com>

    let mut has_post_handler: static bool = true;
    module_param(has_post_handler, bool, 0444);
    static void __kprobes post_handler(struct kprobe *p, struct pt_regs *regs,
    unsigned long flags)
    {
    }
    static struct kprobe kp = {
    .symbol_name = "cmdline_proc_show",
    };
#[no_mangle]
unsafe extern "C" fn kprobe_init() -> int __init {
    static int __init kprobe_init(void)
    {
    if (has_post_handler)
    kp.post_handler = post_handler;
    return register_kprobe(&kp);
    }
#[no_mangle]
unsafe extern "C" fn kprobe_exit() -> void __exit {
    static void __exit kprobe_exit(void)
    {
    unregister_kprobe(&kp);
    }
    module_init(kprobe_init)
    module_exit(kprobe_exit)
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Michael Vetter <mvetter@suse.com>");
    MODULE_DESCRIPTION("Livepatch test: kprobe function");
