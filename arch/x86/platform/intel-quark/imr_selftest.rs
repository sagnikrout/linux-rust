//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/intel-quark/imr_selftest.c
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
// imr_selftest.c -- Intel Isolated Memory Region self-test driver
//
// Copyright(c) 2013 Intel Corporation.
// Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
//
// IMR self test. The purpose of this module is to run a set of tests on the
// IMR API to validate its sanity. We check for overlapping, reserved
// addresses and setup/teardown sanity.
//

//
// imr_self_test_result - Print result string for self test.
//
// @res:	result code - true if test passed false otherwise.
// @fmt:	format string.
// ...		variadic argument list.
//
#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 2, _arg: 3) -> static {
    static __printf(2, 3)
#[no_mangle]
pub unsafe extern "C" fn imr_self_test_result(res: c_int, fmt: *const c_char, ...) -> void __init {
    void __init imr_self_test_result(int res, const char *fmt, ...)
    {
    va_list vlist;
// Print pass/fail.
    if (res)
    pr_info(SELFTEST "pass ");
    else
    pr_info(SELFTEST "fail ");
// Print variable string.
    va_start(vlist, fmt);
    vprintk(fmt, vlist);
    va_end(vlist);
// Optional warning.
    WARN(res == 0, "test failed");
    }

//
// imr_self_test - perform the IMR self-test
//
// Verify IMR self_test with some simple tests to verify overlap,
// zero sized allocations and 1 KiB sized areas.
//
#[no_mangle]
unsafe extern "C" fn imr_self_test() -> void __init {
    static void __init imr_self_test(void)
    {
    let mut base: phys_addr_t = virt_to_phys(&_text);
    let mut size: usize = virt_to_phys(&__end_rodata) - base;
    const char *fmt_over = "overlapped IMR @ (0x%08lx - 0x%08lx)\n";
    int ret;
// Test zero zero.
    ret = imr_add_range(0, 0, 0, 0);
    imr_self_test_result(ret < 0, "zero sized IMR\n");
// Test exact overlap.
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result(ret < 0, fmt_over, __va(base), __va(base + size));
// Test overlap with base inside of existing.
    base += size - IMR_ALIGN;
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result(ret < 0, fmt_over, __va(base), __va(base + size));
// Test overlap with end inside of existing.
    base -= size + IMR_ALIGN * 2;
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result(ret < 0, fmt_over, __va(base), __va(base + size));
// Test that a 1 KiB IMR @ zero with read/write all will bomb out.
    ret = imr_add_range(0, IMR_ALIGN, IMR_READ_ACCESS_ALL,
    IMR_WRITE_ACCESS_ALL);
    imr_self_test_result(ret < 0, "1KiB IMR @ 0x00000000 - access-all\n");
// Test that a 1 KiB IMR @ zero with CPU only will work.
    ret = imr_add_range(0, IMR_ALIGN, IMR_CPU, IMR_CPU);
    imr_self_test_result(ret >= 0, "1KiB IMR @ 0x00000000 - cpu-access\n");
    if (ret >= 0) {
    ret = imr_remove_range(0, IMR_ALIGN);
    imr_self_test_result(ret == 0, "teardown - cpu-access\n");
    }
// Test 2 KiB works.
    size = IMR_ALIGN * 2;
    ret = imr_add_range(0, size, IMR_READ_ACCESS_ALL, IMR_WRITE_ACCESS_ALL);
    imr_self_test_result(ret >= 0, "2KiB IMR @ 0x00000000\n");
    if (ret >= 0) {
    ret = imr_remove_range(0, size);
    imr_self_test_result(ret == 0, "teardown 2KiB\n");
    }
    }
    static const struct x86_cpu_id imr_ids[] __initconst = {
    X86_MATCH_VFM(INTEL_QUARK_X1000, core::ptr::null_mut()),
    {}
    };
//
// imr_self_test_init - entry point for IMR driver.
//
// return: -ENODEV for no IMR support 0 if good to go.
//
#[no_mangle]
unsafe extern "C" fn imr_self_test_init() -> int __init {
    static int __init imr_self_test_init(void)
    {
    if (x86_match_cpu(imr_ids))
    imr_self_test();
    return 0;
    }
    device_initcall(imr_self_test_init);
