//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-evlist.c
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
// Builtin evlist command: Show the list of event selectors present
// in a perf.data file.
//

    static int process_header_feature(const struct perf_tool *tool __maybe_unused,
    struct perf_session *session __maybe_unused,
    union perf_event *event __maybe_unused)
    {
    session_done = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __cmd_evlist(file_name: *const c_char, details: *mut perf_attr_details) -> c_int {
    static int __cmd_evlist(const char *file_name, struct perf_attr_details *details)
    {
    struct perf_session *session;
    struct evsel *pos;
    struct perf_data data = {
    .path      = file_name,
    .mode      = PERF_DATA_MODE_READ,
    .force     = details.force,
    };
    struct perf_tool tool;
    let mut has_tracepoint: bool = false, has_group = false;
    perf_tool__init(&tool, /*ordered_events=*/false);
// only needed for pipe mode
    tool.attr = perf_event__process_attr;
    tool.feature = process_header_feature;
    session = perf_session__new(&data, &tool);
    if (IS_ERR(session))
    return PTR_ERR(session);
    if (data.is_pipe)
    perf_session__process_events(session);
    evlist__for_each_entry(session.evlist, pos) {
    evsel__fprintf(pos, details, stdout);
    if (pos.core.attr.type == PERF_TYPE_TRACEPOINT)
    has_tracepoint = true;
    if (!evsel__is_group_leader(pos))
    has_group = true;
    }
    if (has_tracepoint && !details.trace_fields)
    printf("# Tip: use 'perf evlist --trace-fields' to show fields for tracepoint events\n");
    if (has_group && !details.event_group)
    printf("# Tip: use 'perf evlist -g' to show group information\n");
    perf_session__delete(session);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_evlist(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_evlist(int argc, const char **argv)
    {
    let mut details: perf_attr_details = { .verbose = false, };
    const struct option options[] = {
    OPT_STRING('i', "input", &input_name, "file", "Input file name"),
    OPT_BOOLEAN('F', "freq", &details.freq, "Show the sample frequency"),
    OPT_BOOLEAN('v', "verbose", &details.verbose,
    "Show all event attr details"),
    OPT_BOOLEAN('g', "group", &details.event_group,
    "Show event group information"),
    OPT_BOOLEAN('f', "force", &details.force, "don't complain, do it"),
    OPT_BOOLEAN(0, "trace-fields", &details.trace_fields, "Show tracepoint fields"),
    OPT_END()
    };
    const char * const evlist_usage[] = {
    "perf evlist [<options>]",
    core::ptr::null_mut()
    };
    argc = parse_options(argc, argv, options, evlist_usage, 0);
    if (argc)
    usage_with_options(evlist_usage, options);
    if (details.event_group && (details.verbose || details.freq)) {
    usage_with_options_msg(evlist_usage, options,
    "--group option is not compatible with other options\n");
    }
    return __cmd_evlist(input_name, &details);
    }
