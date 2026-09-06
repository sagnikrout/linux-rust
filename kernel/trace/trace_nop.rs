//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_nop.c
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
// nop tracer
//
// Copyright (C) 2008 Steven Noonan <steven@uplinklabs.net>
//

// Our two options
    enum {
    TRACE_NOP_OPT_ACCEPT = 0x1,
    TRACE_NOP_OPT_REFUSE = 0x2
    };
// Options for the tracer (see trace_options file)
    static struct tracer_opt nop_opts[] = {
// Option that will be accepted by set_flag callback
    { TRACER_OPT(test_nop_accept, TRACE_NOP_OPT_ACCEPT) },
// Option that will be refused by set_flag callback
    { TRACER_OPT(test_nop_refuse, TRACE_NOP_OPT_REFUSE) },
    { } /* Always set a last empty entry */
    };
    static struct tracer_flags nop_flags = {
// You can check your flags value here when you want.
    .val = 0, /* By default: all flags disabled */
    .opts = nop_opts
    };
    static struct trace_array	*ctx_trace;
#[no_mangle]
unsafe extern "C" fn start_nop_trace(tr: *mut trace_array) {
    static void start_nop_trace(struct trace_array *tr)
    {
// Nothing to do!
    }
#[no_mangle]
unsafe extern "C" fn stop_nop_trace(tr: *mut trace_array) {
    static void stop_nop_trace(struct trace_array *tr)
    {
// Nothing to do!
    }
#[no_mangle]
unsafe extern "C" fn nop_trace_init(tr: *mut trace_array) -> c_int {
    static int nop_trace_init(struct trace_array *tr)
    {
    ctx_trace = tr;
    start_nop_trace(tr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nop_trace_reset(tr: *mut trace_array) {
    static void nop_trace_reset(struct trace_array *tr)
    {
    stop_nop_trace(tr);
    }
// It only serves as a signal handler and a callback to
// accept or refuse the setting of a flag.
// If you don't implement it, then the flag setting will be
// automatically accepted.
//
#[no_mangle]
unsafe extern "C" fn nop_set_flag(tr: *mut trace_array, old_flags: u32, bit: u32, set: c_int) -> c_int {
    static int nop_set_flag(struct trace_array *tr, u32 old_flags, u32 bit, int set)
    {
//
// Note that you don't need to update nop_flags.val yourself.
// The tracing Api will do it automatically if you return 0
//
    if (bit == TRACE_NOP_OPT_ACCEPT) {
    printk(KERN_DEBUG "nop_test_accept flag set to %d: we accept."
    " Now cat trace_options to see the result\n",
    set);
    return 0;
    }
    if (bit == TRACE_NOP_OPT_REFUSE) {
    printk(KERN_DEBUG "nop_test_refuse flag set to %d: we refuse."
    " Now cat trace_options to see the result\n",
    set);
    return -EINVAL;
    }
    return 0;
    }
    struct tracer nop_trace __read_mostly =
    {
    .name		= "nop",
    .init		= nop_trace_init,
    .reset		= nop_trace_reset,

    .selftest	= trace_selftest_startup_nop,

    .flags		= &nop_flags,
    .set_flag	= nop_set_flag,
    .allow_instances = true,
    };
