//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/tool_pmu.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

#[no_mangle]
unsafe extern "C" fn do_test(ev: enum tool_pmu_event, with_pmu: bool) -> c_int {
    static int do_test(enum tool_pmu_event ev, bool with_pmu)
    {
    struct evlist *evlist = evlist__new();
    struct evsel *evsel;
    struct parse_events_error err;
    int ret;
    char str[128];
    let mut found: bool = false;
    if (!evlist) {
    pr_err("evlist allocation failed\n");
    return TEST_FAIL;
    }
    if (with_pmu)
    snprintf(str, sizeof(str), "tool/%s/", tool_pmu__event_to_str(ev));
    else
    snprintf(str, sizeof(str), "%s", tool_pmu__event_to_str(ev));
    parse_events_error__init(&err);
    ret = parse_events(evlist, str, &err);
    if (ret) {
    if (!tool_pmu__event_to_str(ev)) {
    ret = TEST_OK;
    goto out;
    }
    pr_debug("FAILED %s:%d failed to parse event '%s', err %d\n",
    __FILE__, __LINE__, str, ret);
    parse_events_error__print(&err, str);
    ret = TEST_FAIL;
    goto out;
    }
    ret = TEST_OK;
    if (with_pmu ? (evlist__nr_entries(evlist) != 1)
    : (evlist__nr_entries(evlist) < 1)) {
    pr_debug("FAILED %s:%d Unexpected number of events for '%s' of %d\n",
    __FILE__, __LINE__, str, evlist__nr_entries(evlist));
    ret = TEST_FAIL;
    goto out;
    }
    evlist__for_each_entry(evlist, evsel) {
    if (perf_pmu__is_tool(evsel.pmu)) {
    if (evsel.core.attr.config != ev) {
    pr_debug("FAILED %s:%d Unexpected config for '%s', %lld != %d\n",
    __FILE__, __LINE__, str, evsel.core.attr.config, ev);
    ret = TEST_FAIL;
    goto out;
    }
    found = true;
    }
    }
    if (!found && tool_pmu__event_to_str(ev)) {
    pr_debug("FAILED %s:%d Didn't find tool event '%s' in parsed evsels\n",
    __FILE__, __LINE__, str);
    ret = TEST_FAIL;
    }
    out:
    parse_events_error__exit(&err);
    evlist__put(evlist);
    return ret;
    }
    static int test__tool_pmu_without_pmu(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    int i;
    tool_pmu__for_each_event(i) {
    let mut ret: c_int = do_test(i, /*with_pmu=*/false);
    if (ret != TEST_OK)
    return ret;
    }
    return TEST_OK;
    }
    static int test__tool_pmu_with_pmu(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    int i;
    tool_pmu__for_each_event(i) {
    let mut ret: c_int = do_test(i, /*with_pmu=*/true);
    if (ret != TEST_OK)
    return ret;
    }
    return TEST_OK;
    }
    static struct test_case tests__tool_pmu[] = {
    TEST_CASE("Parsing without PMU name", tool_pmu_without_pmu),
    TEST_CASE("Parsing with PMU name", tool_pmu_with_pmu),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__tool_pmu = {
    .desc = "Tool PMU",
    .test_cases = tests__tool_pmu,
    };
