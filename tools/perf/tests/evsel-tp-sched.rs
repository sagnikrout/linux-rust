//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/evsel-tp-sched.c
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

#[no_mangle]
unsafe extern "C" fn evsel__test_field(evsel: *mut evsel, name: *const c_char, size: c_int, should_be_signed: bool) -> c_int {
    static int evsel__test_field(struct evsel *evsel, const char *name, int size, bool should_be_signed)
    {
    struct tep_format_field *field = evsel__field(evsel, name);
    int is_signed;
    let mut ret: c_int = 0;
    if (field == core::ptr::null_mut()) {
    pr_debug("%s: \"%s\" field not found!\n", evsel.name, name);
    return -1;
    }
    is_signed = !!(field.flags & TEP_FIELD_IS_SIGNED);
    if (should_be_signed && !is_signed) {
    pr_debug("%s: \"%s\" signedness(%d) is wrong, should be %d\n",
    evsel.name, name, is_signed, should_be_signed);
    ret = -1;
    }
    if (field.size != size) {
    pr_debug("%s: \"%s\" size (%d) should be %d!\n",
    evsel.name, name, field.size, size);
    ret = -1;
    }
    return ret;
    }
    static int test__perf_evsel__tp_sched_test(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct evsel *evsel = evsel__newtp("sched", "sched_switch");
    let mut ret: c_int = TEST_OK;
    if (IS_ERR(evsel)) {
    pr_debug("evsel__newtp failed with %ld\n", PTR_ERR(evsel));
    return PTR_ERR(evsel) == -EACCES ? TEST_SKIP : TEST_FAIL;
    }
    if (evsel__test_field(evsel, "prev_comm", 16, false))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "prev_pid", 4, true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "prev_prio", 4, true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "prev_state", sizeof(long), true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "next_comm", 16, false))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "next_pid", 4, true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "next_prio", 4, true))
    ret = TEST_FAIL;
    evsel__put(evsel);
    evsel = evsel__newtp("sched", "sched_wakeup");
    if (IS_ERR(evsel)) {
    pr_debug("evsel__newtp failed with %ld\n", PTR_ERR(evsel));
    return TEST_FAIL;
    }
    if (evsel__test_field(evsel, "comm", 16, false))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "pid", 4, true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "prio", 4, true))
    ret = TEST_FAIL;
    if (evsel__test_field(evsel, "target_cpu", 4, true))
    ret = TEST_FAIL;
    evsel__put(evsel);
    return ret;
    }
    static struct test_case tests__perf_evsel__tp_sched_test[] = {
    TEST_CASE_REASON("Parse sched tracepoints fields",
    perf_evsel__tp_sched_test,
    "permissions"),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__perf_evsel__tp_sched_test = {
    .desc = "Parse sched tracepoints fields",
    .test_cases = tests__perf_evsel__tp_sched_test,
    };
