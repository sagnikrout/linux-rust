//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/for_each_map_elem_write_key.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } array_map SEC(".maps");
    static __u64
    check_array_elem(struct bpf_map *map, __u32 *key, __u64 *val,
    void *data)
    {
    bpf_get_current_comm(key, sizeof(*key));
    return 0;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_map_key_write(ctx: *const c_void) -> c_int {
    int test_map_key_write(const void *ctx)
    {
    bpf_for_each_map_elem(&array_map, check_array_elem, core::ptr::null_mut(), 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
