//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_modules/test_klp_mod_patch.c
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
// Copyright (C) 2026 Pablo Hugen <phugen@redhat.com>

#[no_mangle]
unsafe extern "C" fn livepatch_mod_target_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int livepatch_mod_target_show(struct seq_file *m, void *v)
    {
    seq_printf(m, "%s: %s\n", THIS_MODULE.name,
    "this has been live patched");
    return 0;
    }
    static struct klp_func funcs[] = {
    {
    .old_name = "test_klp_mod_target_show",
    .new_func = livepatch_mod_target_show,
    },
    {},
    };
    static struct klp_object objs[] = {
    {
    .name = "test_klp_mod_target",
    .funcs = funcs,
    },
    {},
    };
    static struct klp_patch patch = {
    .mod = THIS_MODULE,
    .objs = objs,
    };
#[no_mangle]
unsafe extern "C" fn test_klp_mod_patch_init() -> c_int {
    static int test_klp_mod_patch_init(void)
    {
    return klp_enable_patch(&patch);
    }
#[no_mangle]
unsafe extern "C" fn test_klp_mod_patch_exit() {
    static void test_klp_mod_patch_exit(void)
    {
    }
    module_init(test_klp_mod_patch_init);
    module_exit(test_klp_mod_patch_exit);
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
    MODULE_AUTHOR("Pablo Hugen <phugen@redhat.com>");
    MODULE_DESCRIPTION("Livepatch test: patch for module-provided function");
