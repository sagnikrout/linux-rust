//! Automatically rewritten from C to Rust
//! Source: kernel/backtracetest.c
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
// Simple stack backtrace regression test module
//
// (C) Copyright 2008 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn backtrace_test_normal() {
    static void backtrace_test_normal(void)
    {
    pr_info("Testing a backtrace from process context.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    dump_stack();
    }
#[no_mangle]
unsafe extern "C" fn backtrace_test_bh_workfn(work: *mut work_struct) {
    static void backtrace_test_bh_workfn(struct work_struct *work)
    {
    dump_stack();
    }
    static DECLARE_WORK(backtrace_bh_work, &backtrace_test_bh_workfn);
#[no_mangle]
unsafe extern "C" fn backtrace_test_bh() {
    static void backtrace_test_bh(void)
    {
    pr_info("Testing a backtrace from BH context.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    queue_work(system_bh_wq, &backtrace_bh_work);
    flush_work(&backtrace_bh_work);
    }

#[no_mangle]
unsafe extern "C" fn backtrace_test_saved() {
    static void backtrace_test_saved(void)
    {
    unsigned long entries[8];
    unsigned int nr_entries;
    pr_info("Testing a saved backtrace.\n");
    pr_info("The following trace is a kernel self test and not a bug!\n");
    nr_entries = stack_trace_save(entries, ARRAY_SIZE(entries), 0);
    stack_trace_print(entries, nr_entries, 0);
    }

#[no_mangle]
unsafe extern "C" fn backtrace_test_saved() {
    static void backtrace_test_saved(void)
    {
    pr_info("Saved backtrace test skipped.\n");
    }

#[no_mangle]
unsafe extern "C" fn backtrace_regression_test() -> c_int {
    static int backtrace_regression_test(void)
    {
    pr_info("====[ backtrace testing ]===========\n");
    backtrace_test_normal();
    backtrace_test_bh();
    backtrace_test_saved();
    pr_info("====[ end of backtrace testing ]====\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exitf() {
    static void exitf(void)
    {
    }
    module_init(backtrace_regression_test);
    module_exit(exitf);
    MODULE_DESCRIPTION("Simple stack backtrace regression test module");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Arjan van de Ven <arjan@linux.intel.com>");
