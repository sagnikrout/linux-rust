//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/tests/bpf/bpf_action_map.c
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

    char LICENSE[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, unsigned int);
    __type(value, unsigned long long);
    } rtla_test_map SEC(".maps");
    struct trace_event_raw_timerlat_sample;
    SEC("tp/timerlat_action")
#[no_mangle]
pub unsafe extern "C" fn action_handler(tp_args: *mut trace_event_raw_timerlat_sample) -> c_int {
    int action_handler(struct trace_event_raw_timerlat_sample *tp_args)
    {
    let mut key: c_uint = 0;
    let mut value: c_ulonglong = 42;
    bpf_map_update_elem(&rtla_test_map, &key, &value, BPF_ANY);
    return 0;
    }
