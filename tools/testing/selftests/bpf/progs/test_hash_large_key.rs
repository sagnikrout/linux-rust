//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_hash_large_key.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 2);
    __type(key, struct bigelement);
    __type(value, __u32);
    } hash_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct bigelement);
    } key_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bigelement {
    pub a: c_int,
    pub b: [c_char; 4096],
    pub c: c_longlong,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bpf_hash_large_key_test(ctx: *mut c_void) -> c_int {
    int bpf_hash_large_key_test(void *ctx)
    {
    let mut zero: c_int = 0, value = 42;
    struct bigelement *key;
    key = bpf_map_lookup_elem(&key_map, &zero);
    if (!key)
    return 0;
    key.c = 1;
    if (bpf_map_update_elem(&hash_map, key, &value, BPF_ANY))
    return 0;
    return 0;
    }
