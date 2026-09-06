//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_ca_update.c
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
    let mut ca1_cnt: c_int = 0;
    let mut ca2_cnt: c_int = 0;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ca_update_1_init, sk: *mut sock) {
    void BPF_PROG(ca_update_1_init, struct sock *sk)
    {
    ca1_cnt++;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ca_update_2_init, sk: *mut sock) {
    void BPF_PROG(ca_update_2_init, struct sock *sk)
    {
    ca2_cnt++;
    }
    SEC("struct_ops")
    void BPF_PROG(ca_update_cong_control, struct sock *sk,
    const struct rate_sample *rs)
    {
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ca_update_ssthresh, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(ca_update_ssthresh, struct sock *sk)
    {
    return tcp_sk(sk).snd_ssthresh;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ca_update_undo_cwnd, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(ca_update_undo_cwnd, struct sock *sk)
    {
    return tcp_sk(sk).snd_cwnd;
    }
    SEC(".struct_ops.link")
    struct tcp_congestion_ops ca_update_1 = {
    .init = (void *)ca_update_1_init,
    .cong_control = (void *)ca_update_cong_control,
    .ssthresh = (void *)ca_update_ssthresh,
    .undo_cwnd = (void *)ca_update_undo_cwnd,
    .name = "tcp_ca_update",
    };
    SEC(".struct_ops.link")
    struct tcp_congestion_ops ca_update_2 = {
    .init = (void *)ca_update_2_init,
    .cong_control = (void *)ca_update_cong_control,
    .ssthresh = (void *)ca_update_ssthresh,
    .undo_cwnd = (void *)ca_update_undo_cwnd,
    .name = "tcp_ca_update",
    };
    SEC(".struct_ops.link")
    struct tcp_congestion_ops ca_wrong = {
    .cong_control = (void *)ca_update_cong_control,
    .ssthresh = (void *)ca_update_ssthresh,
    .undo_cwnd = (void *)ca_update_undo_cwnd,
    .name = "tcp_ca_wrong",
    };
    SEC(".struct_ops")
    struct tcp_congestion_ops ca_no_link = {
    .cong_control = (void *)ca_update_cong_control,
    .ssthresh = (void *)ca_update_ssthresh,
    .undo_cwnd = (void *)ca_update_undo_cwnd,
    .name = "tcp_ca_no_link",
    };
