//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func12.c
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

#[no_mangle]
pub unsafe extern "C" fn foo(s: *const S) -> __noinline int {
    __noinline int foo(const struct S *s)
    {
    return bpf_get_prandom_u32() < s.x;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn __msg('mem_or_null'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'mem_or_null'")
#[no_mangle]
pub unsafe extern "C" fn global_func12(skb: *mut __sk_buff) -> c_int {
    int global_func12(struct __sk_buff *skb)
    {
    let mut s: S = {.x = skb.len };
    foo(&s);
    return 1;
    }
