//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/kselftest.h
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
// kselftest.h:	low-level kselftest framework to include from
// selftest programs. When possible, please use
// kselftest_harness.h instead.
//
// Copyright (c) 2014 Shuah Khan <shuahkh@osg.samsung.com>
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
//
// Using this API consists of first counting how many tests your code
// has to run, and then starting up the reporting:
//
// ksft_print_header();
// ksft_set_plan(total_number_of_tests);
//
// For each test, report any progress, debugging, etc with:
//
// ksft_print_msg(fmt, ...);
// ksft_perror(msg);
//
// and finally report the pass/fail/skip/xfail/xpass state of the test
// with one of:
//
// ksft_test_result(condition, fmt, ...);
// ksft_test_result_report(result, fmt, ...);
// ksft_test_result_pass(fmt, ...);
// ksft_test_result_fail(fmt, ...);
// ksft_test_result_skip(fmt, ...);
// ksft_test_result_xfail(fmt, ...);
// ksft_test_result_xpass(fmt, ...);
// ksft_test_result_error(fmt, ...);
// ksft_test_result_code(exit_code, test_name, fmt, ...);
//
// When all tests are finished, clean up and exit the program with one of:
//
// ksft_finished();
// ksft_exit(condition);
// ksft_exit_pass();
// ksft_exit_fail();
//
// If the program wants to report details on why the entire program has
// failed, it can instead exit with a message (this is usually done when
// the program is aborting before finishing all tests):
//
// ksft_exit_fail_msg(fmt, ...);
// ksft_exit_fail_perror(msg);
//

//
// gcc cpuid.h provides __cpuid_count() since v4.4.
// Clang/LLVM cpuid.h provides  __cpuid_count() since v3.4.0.
//
// Provide local define for tests needing __cpuid_count() because
// selftests need to work in older environments that do not yet
// have __cpuid_count().
//

// define kselftest exit codes
pub const KSFT_PASS: c_int = 0;
pub const KSFT_FAIL: c_int = 1;
pub const KSFT_XFAIL: c_int = 2;
pub const KSFT_XPASS: c_int = 3;
pub const KSFT_SKIP: c_int = 4;

// counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksft_count {
    pub ksft_pass: c_uint,
    pub ksft_fail: c_uint,
    pub ksft_xfail: c_uint,
    pub ksft_xpass: c_uint,
    pub ksft_xskip: c_uint,
    pub ksft_error: c_uint,
}

//
// Force line buffering; If stdout is not connected to a terminal, it
// will otherwise default to fully buffered, which can cause output
// duplication if there is content in the buffer when fork()ing. If
// there is a crash, line buffering also means the most recent output
// line will be visible.
//
// ksft_test_result() - Report test success based on truth of condition
//
// @condition: if true, report test success, otherwise failure.
//

// TODO: how does "error" differ from "fail" or "skip"?
// Docs seem to call for double space if directive is absent
//
// ksft_test_result_report() - Report test result based on a kselftest exit code
//
// @result: a kselftest exit code
//

//
// ksft_exit() - Exit selftest based on truth of condition
//
// @condition: if true, exit self test with success, otherwise fail.
//

//
// ksft_finished() - Exit selftest with success if all tests passed
//

//
// FIXME: several tests misuse ksft_exit_skip so produce
// something sensible if some tests have already been run
// or a plan has been printed.  Those tests should use
// ksft_test_result_skip or ksft_exit_fail_msg instead.
//
