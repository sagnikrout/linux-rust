//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_get_stack_rawtp.c
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

// Permit pretty deep stack traces
pub const MAX_STACK_RAWTP: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_trace_t {
    pub pid: c_int,
    pub kern_stack_size: c_int,
    pub user_stack_size: c_int,
    pub user_stack_buildid_size: c_int,
    pub kern_stack: [__u64; MAX_STACK_RAWTP],
    pub user_stack: [__u64; MAX_STACK_RAWTP],
    pub user_stack_buildid: [bpf_stack_build_id; MAX_STACK_RAWTP],
}

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(max_entries, 2);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(__u32));
    } perfmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct stack_trace_t);
    } stackdata_map SEC(".maps");
// Allocate per-cpu space twice the needed. For the code below
// usize = bpf_get_stack(ctx, raw_data, max_len, BPF_F_USER_STACK);
// if (usize < 0)
// return 0;
// ksize = bpf_get_stack(ctx, raw_data + usize, max_len - usize, 0);
//
// If we have value_size = MAX_STACK_RAWTP * sizeof(__u64),
// verifier will complain that access "raw_data + usize"
// with size "max_len - usize" may be out of bound.
// The maximum "raw_data + usize" is "raw_data + max_len"
// and the maximum "max_len - usize" is "max_len", verifier
// concludes that the maximum buffer access range is
// "raw_data[0...max_len * 2 - 1]" and hence reject the program.
//
// Doubling the to-be-used max buffer size can fix this verifier
// issue and avoid complicated C programming massaging.
// This is an acceptable workaround since there is one entry here.
//
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64[2 * MAX_STACK_RAWTP]);
    } rawdata_map SEC(".maps");
    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut c_void) -> c_int {
    int bpf_prog1(void *ctx)
    {
    int max_len, max_buildid_len, total_size;
    struct stack_trace_t *data;
    long usize, ksize;
    void *raw_data;
    let mut key: __u32 = 0;
    data = bpf_map_lookup_elem(&stackdata_map, &key);
    if (!data)
    return 0;
    max_len = MAX_STACK_RAWTP * sizeof(__u64);
    max_buildid_len = MAX_STACK_RAWTP * sizeof(struct bpf_stack_build_id);
    data.pid = bpf_get_current_pid_tgid();
    data.kern_stack_size = bpf_get_stack(ctx, data.kern_stack,
    max_len, 0);
    data.user_stack_size = bpf_get_stack(ctx, data.user_stack, max_len,
    BPF_F_USER_STACK);
    data.user_stack_buildid_size = bpf_get_stack(
    ctx, data.user_stack_buildid, max_buildid_len,
    BPF_F_USER_STACK | BPF_F_USER_BUILD_ID);
    bpf_perf_event_output(ctx, &perfmap, 0, data, sizeof(*data));
// write both kernel and user stacks to the same buffer
    raw_data = bpf_map_lookup_elem(&rawdata_map, &key);
    if (!raw_data)
    return 0;
    usize = bpf_get_stack(ctx, raw_data, max_len, BPF_F_USER_STACK);
    if (usize < 0)
    return 0;
    ksize = bpf_get_stack(ctx, raw_data + usize, max_len - usize, 0);
    if (ksize < 0)
    return 0;
    total_size = usize + ksize;
    if (total_size > 0 && total_size <= max_len)
    bpf_perf_event_output(ctx, &perfmap, 0, raw_data, total_size);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
