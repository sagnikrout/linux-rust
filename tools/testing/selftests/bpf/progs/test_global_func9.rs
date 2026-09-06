//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func9.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub x: c_int,
    pub y: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct S);
    } map SEC(".maps");
    enum E {
    E_ITEM
    };
    let mut global_data_x: static int = 100;
    let mut global_data_y: static int volatile = 500;
#[no_mangle]
pub unsafe extern "C" fn foo(s: *const S) -> __noinline int {
    __noinline int foo(const struct S *s)
    {
    if (s)
    return bpf_get_prandom_u32() < s.x;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bar(x: *mut c_int) -> __noinline int {
    __noinline int bar(int *x)
    {
    if (x)
// x &= bpf_get_prandom_u32();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn baz(x: *mut volatile int) -> __noinline int {
    __noinline int baz(volatile int *x)
    {
    if (x)
// x &= bpf_get_prandom_u32();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qux(e: *mut enum E) -> __noinline int {
    __noinline int qux(enum E *e)
    {
    if (e)
    return *e;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn quux((*arr)[10]: *mut c_int) -> __noinline int {
    __noinline int quux(int (*arr)[10])
    {
    if (arr)
    return (*arr)[9];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn quuz(p: *mut c_int) -> __noinline int {
    __noinline int quuz(int **p)
    {
    if (p)
// p = NULL;
    return 0;
    }
    SEC("cgroup_skb/ingress")
    __success
#[no_mangle]
pub unsafe extern "C" fn global_func9(skb: *mut __sk_buff) -> c_int {
    int global_func9(struct __sk_buff *skb)
    {
    let mut result: c_int = 0;
    {
    let mut s: S = {.x = skb.len };
    result |= foo(&s);
    }
    {
    let mut key: __u32 = 1;
    const struct S *s = bpf_map_lookup_elem(&map, &key);
    result |= foo(s);
    }
    {
    let mut c: C = {.x = skb.len, .y = skb.family };
    result |= foo((const struct S *)&c);
    }
    {
    result |= foo(core::ptr::null_mut());
    }
    {
    bar(&result);
    bar(&global_data_x);
    }
    {
    result |= baz(&global_data_y);
    }
    {
    let mut e: enum E = E_ITEM;
    result |= qux(&e);
    }
    {
    int array[10] = {0};
    result |= quux(&array);
    }
    {
    int *p;
    result |= quuz(&p);
    }
    return result ? 1 : 0;
    }
