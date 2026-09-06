//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sock_destroy_prog_fail.c
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
    int bpf_sock_destroy(struct sock_common *sk) __ksym;
    SEC("tp_btf/tcp_destroy_sock")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "calling kernel function bpf_sock_destroy is not) -> __failure {
    __failure __msg("calling kernel function bpf_sock_destroy is not allowed")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_tcp_destroy_sock, sk: *mut sock) -> c_int {
    int BPF_PROG(trace_tcp_destroy_sock, struct sock *sk)
    {
// should not load
    bpf_sock_destroy((struct sock_common *)sk);
    return 0;
    }
