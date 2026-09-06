//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/evsel-roundtrip-name.c
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
unsafe extern "C" fn perf_evsel__roundtrip_cache_name_test() -> c_int {
    static int perf_evsel__roundtrip_cache_name_test(void)
    {
    let mut ret: c_int = TEST_OK;
    for (int type = 0; type < PERF_COUNT_HW_CACHE_MAX; type++) {
    for (int op = 0; op < PERF_COUNT_HW_CACHE_OP_MAX; op++) {
// skip invalid cache type
    if (!evsel__is_cache_op_valid(type, op))
    continue;
    for (int res = 0; res < PERF_COUNT_HW_CACHE_RESULT_MAX; res++) {
    char name[128];
    struct evlist *evlist = evlist__new();
    struct evsel *evsel;
    int err;
    if (evlist == core::ptr::null_mut()) {
    pr_debug("Failed to alloc evlist");
    return TEST_FAIL;
    }
    __evsel__hw_cache_type_op_res_name(type, op, res,
    name, sizeof(name));
    err = parse_event(evlist, name);
    if (err) {
    pr_debug("Failure to parse cache event '%s' possibly as PMUs don't support it",
    name);
    evlist__put(evlist);
    continue;
    }
    evlist__for_each_entry(evlist, evsel) {
    if (!evsel__name_is(evsel, name)) {
    pr_debug("%s != %s\n", evsel__name(evsel), name);
    ret = TEST_FAIL;
    }
    }
    evlist__put(evlist);
    }
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn perf_evsel__name_array_test(names[]: *const *const c_char, nr_names: c_int) -> c_int {
    static int perf_evsel__name_array_test(const char *const names[], int nr_names)
    {
    let mut ret: c_int = TEST_OK;
    for (int i = 0; i < nr_names; ++i) {
    struct evlist *evlist = evlist__new();
    struct evsel *evsel;
    int err;
    if (evlist == core::ptr::null_mut()) {
    pr_debug("Failed to alloc evlist");
    return TEST_FAIL;
    }
    err = parse_event(evlist, names[i]);
    if (err) {
    pr_debug("failed to parse event '%s', err %d\n",
    names[i], err);
    evlist__put(evlist);
    ret = TEST_FAIL;
    continue;
    }
    evlist__for_each_entry(evlist, evsel) {
    if (!evsel__name_is(evsel, names[i])) {
    pr_debug("%s != %s\n", evsel__name(evsel), names[i]);
    ret = TEST_FAIL;
    }
    }
    evlist__put(evlist);
    }
    return ret;
    }
    static int test__perf_evsel__roundtrip_name_test(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    let mut err: c_int = 0, ret = TEST_OK;
    err = perf_evsel__name_array_test(evsel__hw_names, PERF_COUNT_HW_MAX);
    if (err)
    ret = err;
    err = perf_evsel__name_array_test(evsel__sw_names, PERF_COUNT_SW_DUMMY + 1);
    if (err)
    ret = err;
    err = perf_evsel__roundtrip_cache_name_test();
    if (err)
    ret = err;
    return ret;
    }
    DEFINE_SUITE("Roundtrip evsel.name", perf_evsel__roundtrip_name_test);
