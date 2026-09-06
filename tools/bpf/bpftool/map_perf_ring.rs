//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/map_perf_ring.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Netronome Systems, Inc.
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const MMAP_PAGE_CNT: c_int = 16;
    static volatile bool stop;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_sample {
    pub header: perf_event_header,
    pub time: __u64,
    pub size: __u32,
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_lost {
    pub header: perf_event_header,
    pub id: __u64,
    pub lost: __u64,
}

#[no_mangle]
unsafe extern "C" fn int_exit(signo: c_int) {
    static void int_exit(int signo)
    {
    fprintf(stderr, "Stopping...\n");
    stop = true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_pipe_ctx {
    pub all_cpus: bool,
    pub cpu: c_int,
    pub idx: c_int,
}

    static enum bpf_perf_event_ret
    print_bpf_output(void *private_data, int cpu, struct perf_event_header *event)
    {
    struct perf_event_sample *e = container_of(event,
    struct perf_event_sample,
    header);
    struct perf_event_lost *lost = container_of(event,
    struct perf_event_lost,
    header);
    struct event_pipe_ctx *ctx = private_data;
    let mut idx: c_int = ctx.all_cpus ? cpu : ctx.idx;
    if (json_output) {
    jsonw_start_object(json_wtr);
    jsonw_name(json_wtr, "type");
    jsonw_uint(json_wtr, e.header.type);
    jsonw_name(json_wtr, "cpu");
    jsonw_uint(json_wtr, cpu);
    jsonw_name(json_wtr, "index");
    jsonw_uint(json_wtr, idx);
    if (e.header.type == PERF_RECORD_SAMPLE) {
    jsonw_name(json_wtr, "timestamp");
    jsonw_uint(json_wtr, e.time);
    jsonw_name(json_wtr, "data");
    print_data_json(e.data, e.size);
    } else if (e.header.type == PERF_RECORD_LOST) {
    jsonw_name(json_wtr, "lost");
    jsonw_start_object(json_wtr);
    jsonw_name(json_wtr, "id");
    jsonw_uint(json_wtr, lost.id);
    jsonw_name(json_wtr, "count");
    jsonw_uint(json_wtr, lost.lost);
    jsonw_end_object(json_wtr);
    }
    jsonw_end_object(json_wtr);
    } else {
    if (e.header.type == PERF_RECORD_SAMPLE) {
    printf("== @%llu.%09llu CPU: %d index: %d =====\n",
    e.time / 1000000000ULL, e.time % 1000000000ULL,
    cpu, idx);
    fprint_hex(stdout, e.data, e.size, " ");
    printf("\n");
    } else if (e.header.type == PERF_RECORD_LOST) {
    printf("lost %llu events\n", lost.lost);
    } else {
    printf("unknown event type=%u size=%u\n",
    e.header.type, e.header.size);
    }
    }
    return LIBBPF_PERF_EVENT_CONT;
    }
#[no_mangle]
pub unsafe extern "C" fn do_event_pipe(argc: c_int, argv: *mut c_char) -> c_int {
    int do_event_pipe(int argc, char **argv)
    {
    struct perf_event_attr perf_attr = {
    .sample_type = PERF_SAMPLE_RAW | PERF_SAMPLE_TIME,
    .type = PERF_TYPE_SOFTWARE,
    .config = PERF_COUNT_SW_BPF_OUTPUT,
    .sample_period = 1,
    .wakeup_events = 1,
    };
    let mut map_info: bpf_map_info = {};
    LIBBPF_OPTS(perf_buffer_raw_opts, opts);
    struct event_pipe_ctx ctx = {
    .all_cpus = true,
    .cpu = -1,
    .idx = -1,
    };
    struct perf_buffer *pb;
    __u32 map_info_len;
    int err, map_fd;
    map_info_len = sizeof(map_info);
    map_fd = map_parse_fd_and_info(&argc, &argv, &map_info, &map_info_len,
    0);
    if (map_fd < 0)
    return -1;
    if (map_info.type != BPF_MAP_TYPE_PERF_EVENT_ARRAY) {
    p_err("map is not a perf event array");
    goto err_close_map;
    }
    while (argc) {
    if (argc < 2) {
    BAD_ARG();
    goto err_close_map;
    }
    if (is_prefix(*argv, "cpu")) {
    char *endptr;
    NEXT_ARG();
    ctx.cpu = strtoul(*argv, &endptr, 0);
    if (*endptr) {
    p_err("can't parse %s as CPU ID", *argv);
    goto err_close_map;
    }
    NEXT_ARG();
    } else if (is_prefix(*argv, "index")) {
    char *endptr;
    NEXT_ARG();
    ctx.idx = strtoul(*argv, &endptr, 0);
    if (*endptr) {
    p_err("can't parse %s as index", *argv);
    goto err_close_map;
    }
    NEXT_ARG();
    } else {
    BAD_ARG();
    goto err_close_map;
    }
    ctx.all_cpus = false;
    }
    if (!ctx.all_cpus) {
    if (ctx.idx == -1 || ctx.cpu == -1) {
    p_err("cpu and index must be specified together");
    goto err_close_map;
    }
    } else {
    ctx.cpu = 0;
    ctx.idx = 0;
    }
    opts.cpu_cnt = ctx.all_cpus ? 0 : 1;
    opts.cpus = &ctx.cpu;
    opts.map_keys = &ctx.idx;
    pb = perf_buffer__new_raw(map_fd, MMAP_PAGE_CNT, &perf_attr,
    print_bpf_output, &ctx, &opts);
    if (!pb) {
    p_err("failed to create perf buffer: %s (%d)",
    strerror(errno), errno);
    goto err_close_map;
    }
    signal(SIGINT, int_exit);
    signal(SIGHUP, int_exit);
    signal(SIGTERM, int_exit);
    if (json_output)
    jsonw_start_array(json_wtr);
    while (!stop) {
    err = perf_buffer__poll(pb, 200);
    if (err < 0 && err != -EINTR) {
    p_err("perf buffer polling failed: %s (%d)",
    strerror(errno), errno);
    goto err_close_pb;
    }
    }
    if (json_output)
    jsonw_end_array(json_wtr);
    perf_buffer__free(pb);
    close(map_fd);
    return 0;
    err_close_pb:
    perf_buffer__free(pb);
    err_close_map:
    close(map_fd);
    return -1;
    }
