//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/trace_printk.c
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
// Copyright (c) 2020, Oracle and/or its affiliates.

#[no_mangle]
unsafe extern "C" fn trace_pipe_cb(str: *const c_char, data: *mut c_void) {
    static void trace_pipe_cb(const char *str, void *data)
    {
    if (strstr(str, SEARCHMSG) != core::ptr::null_mut())
    ((int *)data)[0]++;
    if (strstr(str, SEARCHMSG_UTF8))
    ((int *)data)[1]++;
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_trace_printk() {
    void serial_test_trace_printk(void)
    {
    struct trace_printk_lskel__bss *bss;
    struct trace_printk_lskel *skel;
    let mut err: c_int = 0, found[2] = {};
    skel = trace_printk_lskel__open();
    if (!ASSERT_OK_PTR(skel, "trace_printk__open"))
    return;
    ASSERT_EQ(skel.rodata.fmt[0], 'T', "skel.rodata.fmt[0]");
    skel.rodata.fmt[0] = 't';
    err = trace_printk_lskel__load(skel);
    if (!ASSERT_OK(err, "trace_printk__load"))
    goto cleanup;
    bss = skel.bss;
    err = trace_printk_lskel__attach(skel);
    if (!ASSERT_OK(err, "trace_printk__attach"))
    goto cleanup;
// wait for tracepoint to trigger
    usleep(1);
    trace_printk_lskel__detach(skel);
    if (!ASSERT_GT(bss.trace_printk_ran, 0, "bss.trace_printk_ran"))
    goto cleanup;
    if (!ASSERT_GT(bss.trace_printk_ret, 0, "bss.trace_printk_ret"))
    goto cleanup;
    if (!ASSERT_GT(bss.trace_printk_utf8_ran, 0, "bss.trace_printk_utf8_ran"))
    goto cleanup;
    if (!ASSERT_GT(bss.trace_printk_utf8_ret, 0, "bss.trace_printk_utf8_ret"))
    goto cleanup;
    if (!ASSERT_LT(bss.trace_printk_invalid_spec_ret, 0,
    "bss.trace_printk_invalid_spec_ret"))
    goto cleanup;
// verify our search strings are in the trace buffer
    ASSERT_OK(read_trace_pipe_iter(trace_pipe_cb, found, 1000),
    "read_trace_pipe_iter");
    if (!ASSERT_EQ(found[0], bss.trace_printk_ran, "found"))
    goto cleanup;
    if (!ASSERT_EQ(found[1], bss.trace_printk_utf8_ran, "found_utf8"))
    goto cleanup;
    cleanup:
    trace_printk_lskel__destroy(skel);
    }
