//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func_args.c
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
pub struct S {
    pub v: c_int,
}

    let mut global_variable: S = {};
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 7);
    __type(key, __u32);
    __type(value, int);
    } values SEC(".maps");
#[no_mangle]
unsafe extern "C" fn save_value(index: __u32, value: c_int) {
    static void save_value(__u32 index, int value)
    {
    bpf_map_update_elem(&values, &index, &value, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn foo(index: __u32, s: *mut S) -> __noinline int {
    __noinline int foo(__u32 index, struct S *s)
    {
    if (s) {
    save_value(index, s.v);
    return ++s.v;
    }
    save_value(index, 0);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn bar(index: __u32, s: *mut volatile struct S) -> __noinline int {
    __noinline int bar(__u32 index, volatile struct S *s)
    {
    if (s) {
    save_value(index, s.v);
    return ++s.v;
    }
    save_value(index, 0);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn baz(s: *mut S) -> __noinline int {
    __noinline int baz(struct S **s)
    {
    if (s)
// s = 0;
    return 0;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn test_cls(skb: *mut __sk_buff) -> c_int {
    int test_cls(struct __sk_buff *skb)
    {
    let mut index: __u32 = 0;
    {
    let mut v: c_int = foo(index++, 0);
    save_value(index++, v);
    }
    {
    let mut s: S = { .v = 100 };
    foo(index++, &s);
    save_value(index++, s.v);
    }
    {
    global_variable.v = 42;
    bar(index++, &global_variable);
    save_value(index++, global_variable.v);
    }
    {
    struct S v, *p = &v;
    baz(&p);
    save_value(index++, !p);
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
