//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/event_update.c
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

    static int process_event_unit(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_event_update *ev = (struct perf_record_event_update *)event;
    TEST_ASSERT_VAL("wrong id", ev.id == 123);
    TEST_ASSERT_VAL("wrong id", ev.type == PERF_EVENT_UPDATE__UNIT);
    TEST_ASSERT_VAL("wrong unit", !strcmp(ev.unit, "KRAVA"));
    return 0;
    }
    static int process_event_scale(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_event_update *ev = (struct perf_record_event_update *)event;
    TEST_ASSERT_VAL("wrong id", ev.id == 123);
    TEST_ASSERT_VAL("wrong id", ev.type == PERF_EVENT_UPDATE__SCALE);
    TEST_ASSERT_VAL("wrong scale", ev.scale.scale == 0.123);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_name {
    pub tool: perf_tool,
    pub name: *const c_char,
}

    static int process_event_name(const struct perf_tool *tool,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct event_name *tmp = container_of(tool, struct event_name, tool);
    struct perf_record_event_update *ev = (struct perf_record_event_update *)event;
    TEST_ASSERT_VAL("wrong id", ev.id == 123);
    TEST_ASSERT_VAL("wrong id", ev.type == PERF_EVENT_UPDATE__NAME);
    TEST_ASSERT_VAL("wrong name", !strcmp(ev.name, tmp.name));
    return 0;
    }
    static int process_event_cpus(const struct perf_tool *tool __maybe_unused,
    union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct perf_record_event_update *ev = (struct perf_record_event_update *)event;
    struct perf_cpu_map *map;
    map = cpu_map__new_data(&ev.cpus.cpus);
    TEST_ASSERT_VAL("wrong id", ev.id == 123);
    TEST_ASSERT_VAL("wrong type", ev.type == PERF_EVENT_UPDATE__CPUS);
    TEST_ASSERT_VAL("wrong cpus", perf_cpu_map__nr(map) == 3);
    TEST_ASSERT_VAL("wrong cpus", perf_cpu_map__cpu(map, 0).cpu == 1);
    TEST_ASSERT_VAL("wrong cpus", perf_cpu_map__cpu(map, 1).cpu == 2);
    TEST_ASSERT_VAL("wrong cpus", perf_cpu_map__cpu(map, 2).cpu == 3);
    perf_cpu_map__put(map);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test__event_update(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__event_update(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    struct evsel *evsel;
    struct event_name tmp;
    let mut target: target = {};
    struct evlist *evlist = evlist__new_default(&target, /*sample_callchains=*/false);
    TEST_ASSERT_VAL("failed to get evlist", evlist);
    evsel = evlist__first(evlist);
    TEST_ASSERT_VAL("failed to allocate ids",
    !perf_evsel__alloc_id(&evsel.core, 1, 1));
    perf_evlist__id_add(evlist__core(evlist), &evsel.core, 0, 0, 123);
    free((char *)evsel.unit);
    evsel.unit = strdup("KRAVA");
    TEST_ASSERT_VAL("failed to synthesize attr update unit",
    !perf_event__synthesize_event_update_unit(core::ptr::null_mut(), evsel, process_event_unit));
    evsel.scale = 0.123;
    TEST_ASSERT_VAL("failed to synthesize attr update scale",
    !perf_event__synthesize_event_update_scale(core::ptr::null_mut(), evsel, process_event_scale));
    perf_tool__init(&tmp.tool, /*ordered_events=*/false);
    tmp.name = evsel__name(evsel);
    TEST_ASSERT_VAL("failed to synthesize attr update name",
    !perf_event__synthesize_event_update_name(&tmp.tool, evsel, process_event_name));
    perf_cpu_map__put(evsel.core.pmu_cpus);
    evsel.core.pmu_cpus = perf_cpu_map__new("1,2,3");
    TEST_ASSERT_VAL("failed to synthesize attr update cpus",
    !perf_event__synthesize_event_update_cpus(&tmp.tool, evsel, process_event_cpus));
    evlist__put(evlist);
    return 0;
    }
    DEFINE_SUITE("Synthesize attr update", event_update);
