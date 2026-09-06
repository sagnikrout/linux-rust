//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stacktrace_map_skip.c
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

pub const TEST_STACK_DEPTH: c_int = 2;
pub const TEST_MAX_ENTRIES: c_int = 16384;
    typedef __u64 stack_trace_t[TEST_STACK_DEPTH];
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(max_entries, TEST_MAX_ENTRIES);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stackmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, TEST_MAX_ENTRIES);
    __type(key, __u32);
    __type(value, __u32);
    } stackid_hmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, TEST_MAX_ENTRIES);
    __type(key, __u32);
    __type(value, stack_trace_t);
    } stack_amap SEC(".maps");
    let mut pid: c_int = 0;
    let mut control: c_int = 0;
    let mut failed: c_int = 0;
    SEC("tracepoint/sched/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut trace_event_raw_sched_switch) -> c_int {
    int oncpu(struct trace_event_raw_sched_switch *ctx)
    {
    let mut max_len: __u32 = TEST_STACK_DEPTH * sizeof(__u64);
    let mut key: __u32 = 0, val = 0;
    __u64 *stack_p;
    if (pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    if (control)
    return 0;
// it should allow skipping whole buffer size entries
    key = bpf_get_stackid(ctx, &stackmap, TEST_STACK_DEPTH);
    if ((int)key >= 0) {
// The size of stackmap and stack_amap should be the same
    bpf_map_update_elem(&stackid_hmap, &key, &val, 0);
    stack_p = bpf_map_lookup_elem(&stack_amap, &key);
    if (stack_p) {
    bpf_get_stack(ctx, stack_p, max_len, TEST_STACK_DEPTH);
// it wrongly skipped all the entries and filled zero
    if (stack_p[0] == 0)
    failed = 1;
    }
    } else {
// old kernel doesn't support skipping that many entries
    failed = 2;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
