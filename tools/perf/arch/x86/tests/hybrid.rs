//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/tests/hybrid.c
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
unsafe extern "C" fn test_config(evsel: *const evsel, expected_config: __u64) -> bool {
    static bool test_config(const struct evsel *evsel, __u64 expected_config)
    {
    return (evsel.core.attr.config & PERF_HW_EVENT_MASK) == expected_config;
    }
#[no_mangle]
unsafe extern "C" fn test_perf_config(evsel: *const perf_evsel, expected_config: __u64) -> bool {
    static bool test_perf_config(const struct perf_evsel *evsel, __u64 expected_config)
    {
    return (evsel.attr.config & PERF_HW_EVENT_MASK) == expected_config;
    }
#[no_mangle]
unsafe extern "C" fn test_hybrid_type(evsel: *const evsel, expected_config: __u64) -> bool {
    static bool test_hybrid_type(const struct evsel *evsel, __u64 expected_config)
    {
    return (evsel.core.attr.config >> PERF_PMU_TYPE_SHIFT) == expected_config;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_hw_event_with_pmu(evlist: *mut evlist) -> c_int {
    static int test__hybrid_hw_event_with_pmu(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_hw_group_event(evlist: *mut evlist) -> c_int {
    static int test__hybrid_hw_group_event(struct evlist *evlist)
    {
    struct evsel *evsel, *leader;
    evsel = leader = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_BRANCH_INSTRUCTIONS));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_sw_hw_group_event(evlist: *mut evlist) -> c_int {
    static int test__hybrid_sw_hw_group_event(struct evlist *evlist)
    {
    struct evsel *evsel, *leader;
    evsel = leader = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_SOFTWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_hw_sw_group_event(evlist: *mut evlist) -> c_int {
    static int test__hybrid_hw_sw_group_event(struct evlist *evlist)
    {
    struct evsel *evsel, *leader;
    evsel = leader = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_SOFTWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_group_modifier1(evlist: *mut evlist) -> c_int {
    static int test__hybrid_group_modifier1(struct evlist *evlist)
    {
    struct evsel *evsel, *leader;
    evsel = leader = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    TEST_ASSERT_VAL("wrong exclude_user", evsel.core.attr.exclude_user);
    TEST_ASSERT_VAL("wrong exclude_kernel", !evsel.core.attr.exclude_kernel);
    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_BRANCH_INSTRUCTIONS));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    TEST_ASSERT_VAL("wrong exclude_user", !evsel.core.attr.exclude_user);
    TEST_ASSERT_VAL("wrong exclude_kernel", evsel.core.attr.exclude_kernel);
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_raw1(evlist: *mut evlist) -> c_int {
    static int test__hybrid_raw1(struct evlist *evlist)
    {
    struct perf_evsel *evsel;
    perf_evlist__for_each_evsel(evlist__core(evlist), evsel) {
    struct perf_pmu *pmu = perf_pmus__find_by_type(evsel.attr.type);
    TEST_ASSERT_VAL("missing pmu", pmu);
    TEST_ASSERT_VAL("unexpected pmu", !strncmp(pmu.name, "cpu_", 4));
    TEST_ASSERT_VAL("wrong config", test_perf_config(evsel, 0x1a));
    }
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_raw2(evlist: *mut evlist) -> c_int {
    static int test__hybrid_raw2(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_RAW == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong config", test_config(evsel, 0x1a));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_cache_event(evlist: *mut evlist) -> c_int {
    static int test__hybrid_cache_event(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HW_CACHE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong config", 0x2 == (evsel.core.attr.config & 0xffffffff));
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__checkevent_pmu(evlist: *mut evlist) -> c_int {
    static int test__checkevent_pmu(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 1 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_RAW == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong config",    10 == evsel.core.attr.config);
    TEST_ASSERT_VAL("wrong config1",    1 == evsel.core.attr.config1);
    TEST_ASSERT_VAL("wrong config2",    3 == evsel.core.attr.config2);
    TEST_ASSERT_VAL("wrong config3",    0 == evsel.core.attr.config3);
//
// The period value gets configured within evlist__config,
// while this test executes only parse events method.
//
    TEST_ASSERT_VAL("wrong period",     0 == evsel.core.attr.sample_period);
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test__hybrid_hw_group_event_2(evlist: *mut evlist) -> c_int {
    static int test__hybrid_hw_group_event_2(struct evlist *evlist)
    {
    struct evsel *evsel, *leader;
    evsel = leader = evlist__first(evlist);
    TEST_ASSERT_VAL("wrong number of entries", 2 == evlist__nr_entries(evlist));
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_HARDWARE == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong hybrid type", test_hybrid_type(evsel, PERF_TYPE_RAW));
    TEST_ASSERT_VAL("wrong config", test_config(evsel, PERF_COUNT_HW_CPU_CYCLES));
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    evsel = evsel__next(evsel);
    TEST_ASSERT_VAL("wrong type", PERF_TYPE_RAW == evsel.core.attr.type);
    TEST_ASSERT_VAL("wrong config", evsel.core.attr.config == 0x3c);
    TEST_ASSERT_VAL("wrong leader", evsel__has_leader(evsel, leader));
    return TEST_OK;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evlist_test {
    pub name: *const c_char,
    pub (*valid)(void): *mut bool,
    pub evlist): *mut *mut int (check)(struct evlist,
}

    static const struct evlist_test test__hybrid_events[] = {
    {
    .name  = "cpu_core/cycles/",
    .check = test__hybrid_hw_event_with_pmu,
// 0
    },
    {
    .name  = "{cpu_core/cycles/,cpu_core/branches/}",
    .check = test__hybrid_hw_group_event,
// 1
    },
    {
    .name  = "{cpu-clock,cpu_core/cycles/}",
    .check = test__hybrid_sw_hw_group_event,
// 2
    },
    {
    .name  = "{cpu_core/cycles/,cpu-clock}",
    .check = test__hybrid_hw_sw_group_event,
// 3
    },
    {
    .name  = "{cpu_core/cycles/k,cpu_core/branches/u}",
    .check = test__hybrid_group_modifier1,
// 4
    },
    {
    .name  = "r1a",
    .check = test__hybrid_raw1,
// 5
    },
    {
    .name  = "cpu_core/r1a/",
    .check = test__hybrid_raw2,
// 6
    },
    {
    .name  = "cpu_core/config=10,config1,config2=3,period=1000/u",
    .check = test__checkevent_pmu,
// 7
    },
    {
    .name  = "cpu_core/LLC-loads/",
    .check = test__hybrid_cache_event,
// 8
    },
    {
    .name  = "{cpu_core/cycles/,cpu_core/cpu-cycles/}",
    .check = test__hybrid_hw_group_event_2,
// 9
    },
    };
#[no_mangle]
unsafe extern "C" fn test_event(e: *const evlist_test) -> c_int {
    static int test_event(const struct evlist_test *e)
    {
    struct parse_events_error err;
    struct evlist *evlist;
    int ret;
    if (e.valid && !e.valid()) {
    pr_debug("... SKIP\n");
    return TEST_OK;
    }
    evlist = evlist__new();
    if (evlist == core::ptr::null_mut()) {
    pr_err("Failed allocation");
    return TEST_FAIL;
    }
    parse_events_error__init(&err);
    ret = parse_events(evlist, e.name, &err);
    if (ret) {
    pr_debug("failed to parse event '%s', err %d\n", e.name, ret);
    parse_events_error__print(&err, e.name);
    ret = TEST_FAIL;
    if (parse_events_error__contains(&err, "can't access trace events"))
    ret = TEST_SKIP;
    } else {
    ret = e.check(evlist);
    }
    parse_events_error__exit(&err);
    evlist__put(evlist);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn combine_test_results(existing: c_int, latest: c_int) -> c_int {
    static int combine_test_results(int existing, int latest)
    {
    if (existing == TEST_FAIL)
    return TEST_FAIL;
    if (existing == TEST_SKIP)
    let mut latest: return = = TEST_OK ? TEST_SKIP : latest;
    return latest;
    }
#[no_mangle]
unsafe extern "C" fn test_events(events: *const evlist_test, cnt: c_int) -> c_int {
    static int test_events(const struct evlist_test *events, int cnt)
    {
    let mut ret: c_int = TEST_OK;
    for (int i = 0; i < cnt; i++) {
    const struct evlist_test *e = &events[i];
    int test_ret;
    pr_debug("running test %d '%s'\n", i, e.name);
    test_ret = test_event(e);
    if (test_ret != TEST_OK) {
    pr_debug("Event test failure: test %d '%s'", i, e.name);
    ret = combine_test_results(ret, test_ret);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn test__hybrid(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    int test__hybrid(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    if (perf_pmus__num_core_pmus() == 1)
    return TEST_SKIP;
    return test_events(test__hybrid_events, ARRAY_SIZE(test__hybrid_events));
    }
