//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_fprobe.c
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
// test_fprobe.c - simple sanity test for fprobe
//

pub const div_factor: c_int = 3;
    static struct kunit *current_test;
    static u32 rand1, entry_only_val, entry_val, exit_val;
    static u32 entry_only_count, entry_count, exit_count;
// Use indirect calls to avoid inlining the target functions
    static u32 (*target)(u32 value);
    static u32 (*target2)(u32 value);
    static unsigned long target_ip;
    static unsigned long target2_ip;
    static int entry_return_value;
#[no_mangle]
unsafe extern "C" fn fprobe_selftest_target(value: u32) -> noinline u32 {
    static noinline u32 fprobe_selftest_target(u32 value)
    {
    return (value / div_factor);
    }
#[no_mangle]
unsafe extern "C" fn fprobe_selftest_target2(value: u32) -> noinline u32 {
    static noinline u32 fprobe_selftest_target2(u32 value)
    {
    return (value / div_factor) + 1;
    }
    static notrace int fp_entry_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs, void *data)
    {
    KUNIT_EXPECT_FALSE(current_test, preemptible());
// This can be called on the fprobe_selftest_target and the fprobe_selftest_target2
    if (ip != target_ip)
    KUNIT_EXPECT_EQ(current_test, ip, target2_ip);
    entry_val = (rand1 / div_factor);
    if (fp.entry_data_size) {
    KUNIT_EXPECT_NOT_NULL(current_test, data);
    if (data)
// (u32 *)data = entry_val;
    } else
    KUNIT_EXPECT_NULL(current_test, data);
    return entry_return_value;
    }
    static notrace void fp_exit_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs, void *data)
    {
    let mut ret: c_ulong = ftrace_regs_get_return_value(fregs);
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    if (ip != target_ip) {
    KUNIT_EXPECT_EQ(current_test, ip, target2_ip);
    KUNIT_EXPECT_EQ(current_test, ret, (rand1 / div_factor) + 1);
    } else
    KUNIT_EXPECT_EQ(current_test, ret, (rand1 / div_factor));
    KUNIT_EXPECT_EQ(current_test, entry_val, (rand1 / div_factor));
    exit_val = entry_val + div_factor;
    if (fp.entry_data_size) {
    KUNIT_EXPECT_NOT_NULL(current_test, data);
    if (data)
    KUNIT_EXPECT_EQ(current_test, *(u32 *)data, entry_val);
    } else
    KUNIT_EXPECT_NULL(current_test, data);
    }
// Test entry only (no rethook)
#[no_mangle]
unsafe extern "C" fn test_fprobe_entry(test: *mut kunit) {
    static void test_fprobe_entry(struct kunit *test)
    {
    struct fprobe fp_entry = {
    .entry_handler = fp_entry_handler,
    };
    current_test = test;
// Before register, unregister should be failed.
    KUNIT_EXPECT_NE(test, 0, unregister_fprobe(&fp_entry));
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp_entry, "fprobe_selftest_target*", core::ptr::null_mut()));
    entry_val = 0;
    exit_val = 0;
    target(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, 0, exit_val);
    entry_val = 0;
    exit_val = 0;
    target2(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, 0, exit_val);
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp_entry));
    }
#[no_mangle]
unsafe extern "C" fn test_fprobe(test: *mut kunit) {
    static void test_fprobe(struct kunit *test)
    {
    struct fprobe fp = {
    .entry_handler = fp_entry_handler,
    .exit_handler = fp_exit_handler,
    };
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp, "fprobe_selftest_target*", core::ptr::null_mut()));
    entry_val = 0;
    exit_val = 0;
    target(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, entry_val + div_factor, exit_val);
    entry_val = 0;
    exit_val = 0;
    target2(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, entry_val + div_factor, exit_val);
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp));
    }
#[no_mangle]
unsafe extern "C" fn test_fprobe_syms(test: *mut kunit) {
    static void test_fprobe_syms(struct kunit *test)
    {
    static const char *syms[] = {"fprobe_selftest_target", "fprobe_selftest_target2"};
    struct fprobe fp = {
    .entry_handler = fp_entry_handler,
    .exit_handler = fp_exit_handler,
    };
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_fprobe_syms(&fp, syms, 2));
    entry_val = 0;
    exit_val = 0;
    target(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, entry_val + div_factor, exit_val);
    entry_val = 0;
    exit_val = 0;
    target2(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, entry_val + div_factor, exit_val);
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp));
    }
// Test private entry_data
#[no_mangle]
unsafe extern "C" fn test_fprobe_data(test: *mut kunit) {
    static void test_fprobe_data(struct kunit *test)
    {
    struct fprobe fp = {
    .entry_handler = fp_entry_handler,
    .exit_handler = fp_exit_handler,
    .entry_data_size = sizeof(u32),
    };
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp, "fprobe_selftest_target", core::ptr::null_mut()));
    target(rand1);
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp));
    }
#[no_mangle]
unsafe extern "C" fn test_fprobe_skip(test: *mut kunit) {
    static void test_fprobe_skip(struct kunit *test)
    {
    struct fprobe fp = {
    .entry_handler = fp_entry_handler,
    .exit_handler = fp_exit_handler,
    };
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp, "fprobe_selftest_target", core::ptr::null_mut()));
    entry_return_value = 1;
    entry_val = 0;
    exit_val = 0;
    target(rand1);
    KUNIT_EXPECT_NE(test, 0, entry_val);
    KUNIT_EXPECT_EQ(test, 0, exit_val);
    KUNIT_EXPECT_EQ(test, 0, fp.nmissed);
    entry_return_value = 0;
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp));
    }
// Handler for fprobe entry only case
    static notrace int entry_only_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs, void *data)
    {
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, ip, target_ip);
    entry_only_count++;
    entry_only_val = (rand1 / div_factor);
    return 0;
    }
    static notrace int fprobe_entry_multi_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs,
    void *data)
    {
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, ip, target_ip);
    entry_count++;
    entry_val = (rand1 / div_factor);
    return 0;
    }
    static notrace void fprobe_exit_multi_handler(struct fprobe *fp, unsigned long ip,
    unsigned long ret_ip,
    struct ftrace_regs *fregs,
    void *data)
    {
    let mut ret: c_ulong = ftrace_regs_get_return_value(fregs);
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, ip, target_ip);
    KUNIT_EXPECT_EQ(current_test, ret, (rand1 / div_factor));
    exit_count++;
    exit_val = ret;
    }
#[no_mangle]
unsafe extern "C" fn check_fprobe_multi(test: *mut kunit) {
    static void check_fprobe_multi(struct kunit *test)
    {
    entry_only_count = entry_count = exit_count = 0;
    entry_only_val = entry_val = exit_val = 0;
    target(rand1);
// Verify all handlers were called
    KUNIT_EXPECT_EQ(test, 1, entry_only_count);
    KUNIT_EXPECT_EQ(test, 1, entry_count);
    KUNIT_EXPECT_EQ(test, 1, exit_count);
// Verify values are correct
    KUNIT_EXPECT_EQ(test, (rand1 / div_factor), entry_only_val);
    KUNIT_EXPECT_EQ(test, (rand1 / div_factor), entry_val);
    KUNIT_EXPECT_EQ(test, (rand1 / div_factor), exit_val);
    }
// Test multiple fprobes hooking the same target function
#[no_mangle]
unsafe extern "C" fn test_fprobe_multi(test: *mut kunit) {
    static void test_fprobe_multi(struct kunit *test)
    {
    struct fprobe fp1 = {
    .entry_handler = fprobe_entry_multi_handler,
    .exit_handler = fprobe_exit_multi_handler,
    };
    struct fprobe fp2 = {
    .entry_handler = entry_only_handler,
    };
    current_test = test;
// Test Case 1: Register in order 1 -> 2
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp1, "fprobe_selftest_target", core::ptr::null_mut()));
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp2, "fprobe_selftest_target", core::ptr::null_mut()));
    check_fprobe_multi(test);
// Unregister all
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp1));
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp2));
// Test Case 2: Register in order 2 -> 1
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp2, "fprobe_selftest_target", core::ptr::null_mut()));
    KUNIT_EXPECT_EQ(test, 0, register_fprobe(&fp1, "fprobe_selftest_target", core::ptr::null_mut()));
    check_fprobe_multi(test);
// Unregister all
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp1));
    KUNIT_EXPECT_EQ(test, 0, unregister_fprobe(&fp2));
    }
#[no_mangle]
unsafe extern "C" fn get_ftrace_location(func: *mut c_void) -> c_ulong {
    static unsigned long get_ftrace_location(void *func)
    {
    unsigned long size, addr = (unsigned long)func;
    if (!kallsyms_lookup_size_offset(addr, &size, core::ptr::null_mut()) || !size)
    return 0;
    return ftrace_location_range(addr, addr + size - 1);
    }
#[no_mangle]
unsafe extern "C" fn fprobe_test_init(test: *mut kunit) -> c_int {
    static int fprobe_test_init(struct kunit *test)
    {
    rand1 = get_random_u32_above(div_factor);
    target = fprobe_selftest_target;
    target2 = fprobe_selftest_target2;
    target_ip = get_ftrace_location(target);
    target2_ip = get_ftrace_location(target2);
    return 0;
    }
    static struct kunit_case fprobe_testcases[] = {
    KUNIT_CASE(test_fprobe_entry),
    KUNIT_CASE(test_fprobe),
    KUNIT_CASE(test_fprobe_syms),
    KUNIT_CASE(test_fprobe_data),
    KUNIT_CASE(test_fprobe_skip),
    KUNIT_CASE(test_fprobe_multi),
    {}
    };
    static struct kunit_suite fprobe_test_suite = {
    .name = "fprobe_test",
    .init = fprobe_test_init,
    .test_cases = fprobe_testcases,
    };
    kunit_test_suites(&fprobe_test_suite);
