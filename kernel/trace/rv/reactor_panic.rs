//! Automatically rewritten from C to Rust
//! Source: kernel/trace/rv/reactor_panic.c
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
// Copyright (C) 2019-2022 Red Hat, Inc. Daniel Bristot de Oliveira <bristot@kernel.org>
//
// Panic RV reactor:
// Prints the exception msg to the kernel message log and panic().
//

    __printf(1, 0) static void rv_panic_reaction(const char *msg, va_list args)
    {
    vpanic(msg, args);
    }
    static struct rv_reactor rv_panic = {
    .name = "panic",
    .description = "panic the system if an exception is found.",
    .react = rv_panic_reaction
    };
#[no_mangle]
unsafe extern "C" fn register_react_panic() -> int __init {
    static int __init register_react_panic(void)
    {
    rv_register_reactor(&rv_panic);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_react_panic() -> void __exit {
    static void __exit unregister_react_panic(void)
    {
    rv_unregister_reactor(&rv_panic);
    }
    module_init(register_react_panic);
    module_exit(unregister_react_panic);
    MODULE_AUTHOR("Daniel Bristot de Oliveira");
    MODULE_DESCRIPTION("panic rv reactor: panic if an exception is found.");
