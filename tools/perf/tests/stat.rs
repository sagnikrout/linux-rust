//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/stat.c
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

    static bool has_term(struct perf_record_stat_config *config,
    u64 tag, u64 val)
    {
    unsigned i;
    for (i = 0; i < config.nr; i++) {
    if ((config.data[i].tag == tag) &&
    (config.data[i].val == val))
    return true;
    }
    return false;
    }
    static int process_stat_config_event(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_stat_config *config = &event.stat_config;
    let mut test_stat_config: perf_stat_config = {};

    has_term(config, PERF_STAT_CONFIG_TERM__##term, val)
    TEST_ASSERT_VAL("wrong nr",        config.nr == PERF_STAT_CONFIG_TERM__MAX);
    TEST_ASSERT_VAL("wrong aggr_mode", HAS(AGGR_MODE, AGGR_CORE));
    TEST_ASSERT_VAL("wrong scale",     HAS(SCALE, 1));
    TEST_ASSERT_VAL("wrong interval",  HAS(INTERVAL, 1));

    perf_event__read_stat_config(&test_stat_config, config);
    TEST_ASSERT_VAL("wrong aggr_mode", test_stat_config.aggr_mode == AGGR_CORE);
    TEST_ASSERT_VAL("wrong scale",     test_stat_config.scale == 1);
    TEST_ASSERT_VAL("wrong interval",  test_stat_config.interval == 1);
    return 0;
    }
    static int test__synthesize_stat_config(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct perf_stat_config test_stat_config = {
    .aggr_mode	= AGGR_CORE,
    .scale		= 1,
    .interval	= 1,
    };
    TEST_ASSERT_VAL("failed to synthesize stat_config",
    !perf_event__synthesize_stat_config(core::ptr::null_mut(), &test_stat_config,
    process_stat_config_event,
    core::ptr::null_mut()));
    return 0;
    }
    static int process_stat_event(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_stat *st = &event.stat;
    TEST_ASSERT_VAL("wrong cpu",    st.cpu    == 1);
    TEST_ASSERT_VAL("wrong thread", st.thread == 2);
    TEST_ASSERT_VAL("wrong id",     st.id     == 3);
    TEST_ASSERT_VAL("wrong val",    st.val    == 100);
    TEST_ASSERT_VAL("wrong run",    st.ena    == 200);
    TEST_ASSERT_VAL("wrong ena",    st.run    == 300);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__synthesize_stat(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__synthesize_stat(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    struct perf_counts_values count;
    count.val = 100;
    count.ena = 200;
    count.run = 300;
    TEST_ASSERT_VAL("failed to synthesize stat_config",
    !perf_event__synthesize_stat(core::ptr::null_mut(), (struct perf_cpu){.cpu = 1}, 2, 3,
    &count, process_stat_event, core::ptr::null_mut()));
    return 0;
    }
    static int process_stat_round_event(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_stat_round *stat_round = &event.stat_round;
    TEST_ASSERT_VAL("wrong time", stat_round.time == 0xdeadbeef);
    TEST_ASSERT_VAL("wrong type", stat_round.type == PERF_STAT_ROUND_TYPE__INTERVAL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__synthesize_stat_round(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__synthesize_stat_round(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    TEST_ASSERT_VAL("failed to synthesize stat_config",
    !perf_event__synthesize_stat_round(core::ptr::null_mut(), 0xdeadbeef, PERF_STAT_ROUND_TYPE__INTERVAL,
    process_stat_round_event, core::ptr::null_mut()));
    return 0;
    }
    DEFINE_SUITE("Synthesize stat config", synthesize_stat_config);
    DEFINE_SUITE("Synthesize stat", synthesize_stat);
    DEFINE_SUITE("Synthesize stat round", synthesize_stat_round);
