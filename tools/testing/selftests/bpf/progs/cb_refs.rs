//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cb_refs.c
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
    pub ptr: *mut prog_test_ref_kfunc __kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 16);
    } array_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn cb1(map: *mut c_void, key: *mut c_void, value: *mut c_void, ctx: *mut c_void) -> __noinline int {
    static __noinline int cb1(void *map, void *key, void *value, void *ctx)
    {
    void *p = *(void **)ctx;
    bpf_kfunc_call_test_release(p);
// Without the fix this would cause underflow
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn underflow_prog(ctx: *mut c_void) -> c_int {
    int underflow_prog(void *ctx)
    {
    struct prog_test_ref_kfunc *p;
    let mut sl: c_ulong = 0;
    p = bpf_kfunc_call_test_acquire(&sl);
    if (!p)
    return 0;
    bpf_for_each_map_elem(&array_map, cb1, &p, 0);
    bpf_kfunc_call_test_release(p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cb2(map: *mut c_void, key: *mut c_void, value: *mut c_void, ctx: *mut c_void) -> __always_inline int {
    static __always_inline int cb2(void *map, void *key, void *value, void *ctx)
    {
    let mut sl: c_ulong = 0;
// (void **)ctx = bpf_kfunc_call_test_acquire(&sl);
// Without the fix this would leak memory
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn leak_prog(ctx: *mut c_void) -> c_int {
    int leak_prog(void *ctx)
    {
    struct prog_test_ref_kfunc *p;
    struct map_value *v;
    v = bpf_map_lookup_elem(&array_map, &(int){0});
    if (!v)
    return 0;
    p = core::ptr::null_mut();
    bpf_for_each_map_elem(&array_map, cb2, &p, 0);
    p = bpf_kptr_xchg(&v.ptr, p);
    if (p)
    bpf_kfunc_call_test_release(p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cb(map: *mut c_void, key: *mut c_void, value: *mut c_void, ctx: *mut c_void) -> __always_inline int {
    static __always_inline int cb(void *map, void *key, void *value, void *ctx)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cb3(map: *mut c_void, key: *mut c_void, value: *mut c_void, ctx: *mut c_void) -> __always_inline int {
    static __always_inline int cb3(void *map, void *key, void *value, void *ctx)
    {
    let mut sl: c_ulong = 0;
    void *p;
    bpf_kfunc_call_test_acquire(&sl);
    bpf_for_each_map_elem(&array_map, cb, &p, 0);
// It should only complain here, not in cb. This is why we need
// callback_ref to be set to frameno.
//
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn nested_cb(ctx: *mut c_void) -> c_int {
    int nested_cb(void *ctx)
    {
    struct prog_test_ref_kfunc *p;
    let mut sl: c_ulong = 0;
    let mut sp: c_int = 0;
    p = bpf_kfunc_call_test_acquire(&sl);
    if (!p)
    return 0;
    bpf_for_each_map_elem(&array_map, cb3, &sp, 0);
    bpf_kfunc_call_test_release(p);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn non_cb_transfer_ref(ctx: *mut c_void) -> c_int {
    int non_cb_transfer_ref(void *ctx)
    {
    struct prog_test_ref_kfunc *p;
    let mut sl: c_ulong = 0;
    p = bpf_kfunc_call_test_acquire(&sl);
    if (!p)
    return 0;
    cb1(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), &p);
    bpf_kfunc_call_test_acquire(&sl);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
