//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_ca_incompl_cong_ops.c
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
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: incompl_cong_ops_ssthresh, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(incompl_cong_ops_ssthresh, struct sock *sk)
    {
    return tcp_sk(sk).snd_ssthresh;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: incompl_cong_ops_undo_cwnd, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(incompl_cong_ops_undo_cwnd, struct sock *sk)
    {
    return tcp_sk(sk).snd_cwnd;
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops incompl_cong_ops = {
// Intentionally leaving out any of the required cong_avoid() and
// cong_control() here.
//
    .ssthresh = (void *)incompl_cong_ops_ssthresh,
    .undo_cwnd = (void *)incompl_cong_ops_undo_cwnd,
    .name = "bpf_incompl_ops",
    };
