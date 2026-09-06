//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/exceptions_ext.c
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

    SEC("?fentry")
#[no_mangle]
pub unsafe extern "C" fn pfentry(ctx: *mut c_void) -> c_int {
    int pfentry(void *ctx)
    {
    return 0;
    }
    SEC("?fentry")
#[no_mangle]
pub unsafe extern "C" fn throwing_fentry(ctx: *mut c_void) -> c_int {
    int throwing_fentry(void *ctx)
    {
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb(cookie: u64) -> __noinline int {
    __noinline int exception_cb(u64 cookie)
    {
    return cookie + 64;
    }
    SEC("?freplace")
#[no_mangle]
pub unsafe extern "C" fn extension(ctx: *mut __sk_buff) -> c_int {
    int extension(struct __sk_buff *ctx)
    {
    return 0;
    }
    SEC("?freplace")
    __exception_cb(exception_cb)
#[no_mangle]
pub unsafe extern "C" fn throwing_exception_cb_extension(cookie: u64) -> c_int {
    int throwing_exception_cb_extension(u64 cookie)
    {
    bpf_throw(32);
    return 0;
    }
    SEC("?freplace")
    __exception_cb(exception_cb)
#[no_mangle]
pub unsafe extern "C" fn throwing_extension(ctx: *mut __sk_buff) -> c_int {
    int throwing_extension(struct __sk_buff *ctx)
    {
    bpf_throw(64);
    return 0;
    }
    SEC("?fexit")
#[no_mangle]
pub unsafe extern "C" fn pfexit(ctx: *mut c_void) -> c_int {
    int pfexit(void *ctx)
    {
    return 0;
    }
    SEC("?fexit")
#[no_mangle]
pub unsafe extern "C" fn throwing_fexit(ctx: *mut c_void) -> c_int {
    int throwing_fexit(void *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?fmod_ret")
#[no_mangle]
pub unsafe extern "C" fn pfmod_ret(ctx: *mut c_void) -> c_int {
    int pfmod_ret(void *ctx)
    {
    return 0;
    }
    SEC("?fmod_ret")
#[no_mangle]
pub unsafe extern "C" fn throwing_fmod_ret(ctx: *mut c_void) -> c_int {
    int throwing_fmod_ret(void *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
