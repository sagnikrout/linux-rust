//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/parse-no-sample-id-all.c
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


#[no_mangle]
unsafe extern "C" fn process_event(pevlist: *mut evlist, event: *mut union perf_event) -> c_int {
    static int process_event(struct evlist **pevlist, union perf_event *event)
    {
    struct perf_sample sample;
    int ret;
    if (event.header.type == PERF_RECORD_HEADER_ATTR) {
    if (perf_event__process_attr(core::ptr::null_mut(), event, pevlist)) {
    pr_debug("perf_event__process_attr failed\n");
    return -1;
    }
    return 0;
    }
    if (event.header.type >= PERF_RECORD_USER_TYPE_START)
    return -1;
    if (!*pevlist)
    return -1;
    perf_sample__init(&sample, /*all=*/false);
    ret = evlist__parse_sample(*pevlist, event, &sample);
    perf_sample__exit(&sample);
    if (ret) {
    pr_debug("evlist__parse_sample failed\n");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn process_events(events: *mut union perf_event, count: usize) -> c_int {
    static int process_events(union perf_event **events, size_t count)
    {
    struct evlist *evlist = core::ptr::null_mut();
    let mut err: c_int = 0;
    size_t i;
    for (i = 0; i < count && !err; i++)
    err = process_event(&evlist, events[i]);
    evlist__put(evlist);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_attr_event {
    pub header: perf_event_header,
    pub attr: perf_event_attr,
    pub id: u64,
}

//
// test__parse_no_sample_id_all - test parsing with no sample_id_all bit set.
//
// This function tests parsing data produced on kernel's that do not support the
// sample_id_all bit.  Without the sample_id_all bit, non-sample events (such as
// mmap events) do not have an id sample appended, and consequently logic
// designed to determine the id will not work.  That case happens when there is
// more than one selected event, so this test processes three events: 2
// attributes representing the selected events and one mmap event.
//
// Return: %0 on success, %-1 if the test fails.
//
    static int test__parse_no_sample_id_all(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    int err;
    struct test_attr_event event1 = {
    .header = {
    .type = PERF_RECORD_HEADER_ATTR,
    .size = sizeof(struct test_attr_event),
    },
    .attr = {
    .size = sizeof(struct perf_event_attr),
    },
    .id = 1,
    };
    struct test_attr_event event2 = {
    .header = {
    .type = PERF_RECORD_HEADER_ATTR,
    .size = sizeof(struct test_attr_event),
    },
    .attr = {
    .size = sizeof(struct perf_event_attr),
    },
    .id = 2,
    };
    struct perf_record_mmap event3 = {
    .header = {
    .type = PERF_RECORD_MMAP,
    .size = sizeof(struct perf_record_mmap),
    },
    };
    union perf_event *events[] = {
    (union perf_event *)&event1,
    (union perf_event *)&event2,
    (union perf_event *)&event3,
    };
    err = process_events(events, ARRAY_SIZE(events));
    if (err)
    return -1;
    return 0;
    }
    DEFINE_SUITE("Parse with no sample_id_all bit set", parse_no_sample_id_all);
