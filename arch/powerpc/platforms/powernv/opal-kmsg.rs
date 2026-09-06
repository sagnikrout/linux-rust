//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-kmsg.c
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
// kmsg dumper that ensures the OPAL console fully flushes panic messages
//
// Author: Russell Currey <ruscur@russell.cc>
//
// Copyright 2015 IBM Corporation.
//

//
// Console output is controlled by OPAL firmware.  The kernel regularly calls
// OPAL_POLL_EVENTS, which flushes some console output.  In a panic state,
// however, the kernel no longer calls OPAL_POLL_EVENTS and the panic message
// may not be completely printed.  This function does not actually dump the
// message, it just ensures that OPAL completely flushes the console buffer.
//
    static void kmsg_dump_opal_console_flush(struct kmsg_dumper *dumper,
    struct kmsg_dump_detail *detail)
    {
//
// Outside of a panic context the pollers will continue to run,
// so we don't need to do any special flushing.
//
    if (detail.reason != KMSG_DUMP_PANIC)
    return;
    opal_flush_console(0);
    }
    static struct kmsg_dumper opal_kmsg_dumper = {
    .dump = kmsg_dump_opal_console_flush
    };
#[no_mangle]
pub unsafe extern "C" fn opal_kmsg_init() -> void __init {
    void __init opal_kmsg_init(void)
    {
    int rc;
// Add our dumper to the list
    rc = kmsg_dump_register(&opal_kmsg_dumper);
    if (rc != 0)
    pr_err("opal: kmsg_dump_register failed; returned %d\n", rc);
    }
