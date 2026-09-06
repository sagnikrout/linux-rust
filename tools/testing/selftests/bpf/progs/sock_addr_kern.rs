//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_addr_kern.c
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
// Copyright (c) 2024 Google LLC

    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn init_sock(args: *mut init_sock_args) -> c_int {
    int init_sock(struct init_sock_args *args)
    {
    bpf_kfunc_init_sock(args);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn close_sock(ctx: *mut c_void) -> c_int {
    int close_sock(void *ctx)
    {
    bpf_kfunc_close_sock();
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_connect(args: *mut addr_args) -> c_int {
    int kernel_connect(struct addr_args *args)
    {
    return bpf_kfunc_call_kernel_connect(args);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_bind(args: *mut addr_args) -> c_int {
    int kernel_bind(struct addr_args *args)
    {
    return bpf_kfunc_call_kernel_bind(args);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_listen(args: *mut addr_args) -> c_int {
    int kernel_listen(struct addr_args *args)
    {
    return bpf_kfunc_call_kernel_listen();
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_sendmsg(args: *mut sendmsg_args) -> c_int {
    int kernel_sendmsg(struct sendmsg_args *args)
    {
    return bpf_kfunc_call_kernel_sendmsg(args);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn sock_sendmsg(args: *mut sendmsg_args) -> c_int {
    int sock_sendmsg(struct sendmsg_args *args)
    {
    return bpf_kfunc_call_sock_sendmsg(args);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_getsockname(args: *mut addr_args) -> c_int {
    int kernel_getsockname(struct addr_args *args)
    {
    return bpf_kfunc_call_kernel_getsockname(args);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn kernel_getpeername(args: *mut addr_args) -> c_int {
    int kernel_getpeername(struct addr_args *args)
    {
    return bpf_kfunc_call_kernel_getpeername(args);
    }
    char _license[] SEC("license") = "GPL";
