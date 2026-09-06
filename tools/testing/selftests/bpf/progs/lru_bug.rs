//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lru_bug.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub ptr: *mut task___kptr_untrusted,
}

    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct map_value);
    } lru_map SEC(".maps");
    let mut pid: c_int = 0;
    let mut result: c_int = 1;
    SEC("fentry/bpf_ktime_get_ns")
#[no_mangle]
pub unsafe extern "C" fn printk(ctx: *mut c_void) -> c_int {
    int printk(void *ctx)
    {
    let mut v: map_value = {};
    if (pid == bpf_get_current_task_btf().pid)
    bpf_map_update_elem(&lru_map, &(int){0}, &v, 0);
    return 0;
    }
    SEC("fentry/do_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nanosleep(ctx: *mut c_void) -> c_int {
    int nanosleep(void *ctx)
    {
    let mut val: map_value = {}, *v;
    struct task_struct *current;
    bpf_map_update_elem(&lru_map, &(int){0}, &val, 0);
    v = bpf_map_lookup_elem(&lru_map, &(int){0});
    if (!v)
    return 0;
    bpf_map_delete_elem(&lru_map, &(int){0});
    current = bpf_get_current_task_btf();
    v.ptr = current;
    pid = current.pid;
    bpf_ktime_get_ns();
    result = !v.ptr;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
