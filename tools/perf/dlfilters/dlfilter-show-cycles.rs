//! Automatically rewritten from C to Rust
//! Source: tools/perf/dlfilters/dlfilter-show-cycles.c
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
// dlfilter-show-cycles.c: Print the number of cycles at the start of each line
// Copyright (c) 2021, Intel Corporation.
//

pub const MAX_CPU: c_int = 4096;
    enum {
    INSTR_CYC,
    BRNCH_CYC,
    OTHER_CYC,
    MAX_ENTRY
    };
    static __u64 cycles[MAX_CPU][MAX_ENTRY];
    static __u64 cycles_rpt[MAX_CPU][MAX_ENTRY];
pub const BITS: c_int = 16;

    static struct entry {
    __u32 used;
    __s32 tid;
    __u64 cycles[MAX_ENTRY];
    __u64 cycles_rpt[MAX_ENTRY];
    } table[TABLESZ];
    static int tid_cnt;
#[no_mangle]
unsafe extern "C" fn event_entry(event: *const c_char) -> c_int {
    static int event_entry(const char *event)
    {
    if (!event)
    return OTHER_CYC;
    if (!strncmp(event, "instructions", 12))
    return INSTR_CYC;
    if (!strncmp(event, "branches", 8))
    return BRNCH_CYC;
    return OTHER_CYC;
    }
    static struct entry *find_entry(__s32 tid)
    {
    let mut pos: __u32 = tid & MASK;
    struct entry *e;
    e = &table[pos];
    while (e.used) {
    if (e.tid == tid)
    return e;
    if (++pos == TABLESZ)
    pos = 0;
    e = &table[pos];
    }
    if (tid_cnt >= TABLEMAX) {
    fprintf(stderr, "Too many threads\n");
    return core::ptr::null_mut();
    }
    tid_cnt += 1;
    e.used = 1;
    e.tid = tid;
    return e;
    }
#[no_mangle]
unsafe extern "C" fn add_entry(tid: __s32, pos: c_int, cnt: __u64) {
    static void add_entry(__s32 tid, int pos, __u64 cnt)
    {
    struct entry *e = find_entry(tid);
    if (e)
    e.cycles[pos] += cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn filter_event_early(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int {
    int filter_event_early(void *data, const struct perf_dlfilter_sample *sample, void *ctx)
    {
    let mut cpu: __s32 = sample.cpu;
    let mut tid: __s32 = sample.tid;
    int pos;
    if (!sample.cyc_cnt)
    return 0;
    pos = event_entry(sample.event);
    if (cpu >= 0 && cpu < MAX_CPU)
    cycles[cpu][pos] += sample.cyc_cnt;
#[no_mangle]
pub unsafe extern "C" fn if(-1: tid !=) -> else {
    else if (tid != -1)
    add_entry(tid, pos, sample.cyc_cnt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_vals(cycles: __u64, delta: __u64) {
    static void print_vals(__u64 cycles, __u64 delta)
    {
    if (delta)
    printf("%10llu %10llu ", (unsigned long long)cycles, (unsigned long long)delta);
    else
    printf("%10llu %10s ", (unsigned long long)cycles, "");
    }
#[no_mangle]
pub unsafe extern "C" fn filter_event(data: *mut c_void, sample: *const perf_dlfilter_sample, ctx: *mut c_void) -> c_int {
    int filter_event(void *data, const struct perf_dlfilter_sample *sample, void *ctx)
    {
    let mut cpu: __s32 = sample.cpu;
    let mut tid: __s32 = sample.tid;
    int pos;
    pos = event_entry(sample.event);
    if (cpu >= 0 && cpu < MAX_CPU) {
    print_vals(cycles[cpu][pos], cycles[cpu][pos] - cycles_rpt[cpu][pos]);
    cycles_rpt[cpu][pos] = cycles[cpu][pos];
    return 0;
    }
    if (tid != -1) {
    struct entry *e = find_entry(tid);
    if (e) {
    print_vals(e.cycles[pos], e.cycles[pos] - e.cycles_rpt[pos]);
    e.cycles_rpt[pos] = e.cycles[pos];
    return 0;
    }
    }
    printf("%22s", "");
    return 0;
    }
    const char *filter_description(const char **long_description)
    {
    static char *long_desc = "Cycle counts are accumulated per CPU (or "
    "per thread if CPU is not recorded) from IPC information, and "
    "printed together with the change since the last print, at the "
    "start of each line. Separate counts are kept for branches, "
    "instructions or other events.";
// long_description = long_desc;
    return "Print the number of cycles at the start of each line";
    }
