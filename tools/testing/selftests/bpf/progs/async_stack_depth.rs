//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/async_stack_depth.c
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
pub struct hmap_elem {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap SEC(".maps");
    __attribute__((noinline))
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    volatile char buf[256] = {};
    return buf[69];
    }
    __attribute__((noinline))
#[no_mangle]
unsafe extern "C" fn bad_timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int bad_timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    volatile char buf[300] = {};
    return buf[255] + timer_cb(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(is": "combined stack size of 2 calls) -> __failure {
    __failure __msg("combined stack size of 2 calls is")
#[no_mangle]
pub unsafe extern "C" fn pseudo_call_check(ctx: *mut __sk_buff) -> c_int {
    int pseudo_call_check(struct __sk_buff *ctx)
    {
    struct hmap_elem *elem;
    volatile char buf[256] = {};
    elem = bpf_map_lookup_elem(&hmap, &(int){0});
    if (!elem)
    return 0;
    timer_cb(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    return bpf_timer_set_callback(&elem.timer, timer_cb) + buf[0];
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(is": "combined stack size of 2 calls) -> __failure {
    __failure __msg("combined stack size of 2 calls is")
#[no_mangle]
pub unsafe extern "C" fn async_call_root_check(ctx: *mut __sk_buff) -> c_int {
    int async_call_root_check(struct __sk_buff *ctx)
    {
    struct hmap_elem *elem;
    volatile char buf[256] = {};
    elem = bpf_map_lookup_elem(&hmap, &(int){0});
    if (!elem)
    return 0;
    return bpf_timer_set_callback(&elem.timer, bad_timer_cb) + buf[0];
    }
    char _license[] SEC("license") = "GPL";
