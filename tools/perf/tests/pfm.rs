//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/pfm.c
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
// Test support for libpfm4 event encodings.
//
// Copyright 2020 Google LLC.
//

#[no_mangle]
unsafe extern "C" fn count_pfm_events(evlist: *mut perf_evlist) -> c_int {
    static int count_pfm_events(struct perf_evlist *evlist)
    {
    struct perf_evsel *evsel;
    let mut count: c_int = 0;
    perf_evlist__for_each_entry(evlist, evsel) {
    count++;
    }
    return count;
    }
    static int test__pfm_events(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct evlist *evlist;
    struct option opt;
    size_t i;
    const struct {
    const char *events;
    int nr_events;
    } table[] = {
    {
    .events = "",
    .nr_events = 0,
    },
    {
    .events = "instructions",
    .nr_events = 1,
    },
    {
    .events = "instructions,cycles",
    .nr_events = 2,
    },
    {
    .events = "stereolab",
    .nr_events = 0,
    },
    {
    .events = "instructions,instructions",
    .nr_events = 2,
    },
    {
    .events = "stereolab,instructions",
    .nr_events = 0,
    },
    {
    .events = "instructions,stereolab",
    .nr_events = 1,
    },
    };
    for (i = 0; i < ARRAY_SIZE(table); i++) {
    evlist = evlist__new();
    if (evlist == core::ptr::null_mut())
    return -ENOMEM;
    opt.value = &evlist;
    parse_libpfm_events_option(&opt,
    table[i].events,
    0);
    TEST_ASSERT_EQUAL(table[i].events,
    count_pfm_events(evlist__core(evlist)),
    table[i].nr_events);
    TEST_ASSERT_EQUAL(table[i].events,
    evlist__nr_groups(evlist),
    0);
    evlist__put(evlist);
    }
    return 0;
    }
    static int test__pfm_group(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct evlist *evlist;
    struct option opt;
    size_t i;
    const struct {
    const char *events;
    int nr_events;
    int nr_groups;
    } table[] = {
    {
    .events = "{},",
    .nr_events = 0,
    .nr_groups = 0,
    },
    {
    .events = "{instructions}",
    .nr_events = 1,
    .nr_groups = 0,
    },
    {
    .events = "{instructions},{}",
    .nr_events = 1,
    .nr_groups = 0,
    },
    {
    .events = "{},{instructions}",
    .nr_events = 1,
    .nr_groups = 0,
    },
    {
    .events = "{instructions},{instructions}",
    .nr_events = 2,
    .nr_groups = 0,
    },
    {
    .events = "{instructions,cycles},{instructions,cycles}",
    .nr_events = 4,
    .nr_groups = 2,
    },
    {
    .events = "{stereolab}",
    .nr_events = 0,
    .nr_groups = 0,
    },
    {
    .events =
    "{instructions,cycles},{instructions,stereolab}",
    .nr_events = 3,
    .nr_groups = 1,
    },
    {
    .events = "instructions}",
    .nr_events = 1,
    .nr_groups = 0,
    },
    {
    .events = "{{instructions}}",
    .nr_events = 0,
    .nr_groups = 0,
    },
    };
    for (i = 0; i < ARRAY_SIZE(table); i++) {
    evlist = evlist__new();
    if (evlist == core::ptr::null_mut())
    return -ENOMEM;
    opt.value = &evlist;
    parse_libpfm_events_option(&opt,
    table[i].events,
    0);
    TEST_ASSERT_EQUAL(table[i].events,
    count_pfm_events(evlist__core(evlist)),
    table[i].nr_events);
    TEST_ASSERT_EQUAL(table[i].events,
    evlist__nr_groups(evlist),
    table[i].nr_groups);
    evlist__put(evlist);
    }
    return 0;
    }

    static int test__pfm_events(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    return TEST_SKIP;
    }
    static int test__pfm_group(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    return TEST_SKIP;
    }

    static struct test_case pfm_tests[] = {
    TEST_CASE_REASON("test of individual --pfm-events", pfm_events, "not compiled in"),
    TEST_CASE_REASON("test groups of --pfm-events", pfm_group, "not compiled in"),
    { .name = core::ptr::null_mut(), }
    };
    struct test_suite suite__pfm = {
    .desc = "Test libpfm4 support",
    .test_cases = pfm_tests,
    };
