//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/gtk/setup.c
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

    extern struct perf_error_ops perf_gtk_eops;
#[no_mangle]
pub unsafe extern "C" fn perf_gtk__init() -> c_int {
    int perf_gtk__init(void)
    {
    perf_error__register(&perf_gtk_eops);
    perf_gtk__init_helpline();
    gtk_ui_progress__init();
    perf_gtk__init_hpp();
    return gtk_init_check(core::ptr::null_mut(), core::ptr::null_mut()) ? 0 : -1;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_gtk__exit(__maybe_unused: bool wait_for_ok) {
    void perf_gtk__exit(bool wait_for_ok __maybe_unused)
    {
    if (!perf_gtk__is_active_context(pgctx))
    return;
    perf_error__unregister(&perf_gtk_eops);
    gtk_main_quit();
    }
