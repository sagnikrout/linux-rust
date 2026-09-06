//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_ca_write_sk_pacing.c
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

#[no_mangle]
unsafe extern "C" fn tcp_left_out(tp: *const tcp_sock) -> c_uint {
    static unsigned int tcp_left_out(const struct tcp_sock *tp)
    {
    return tp.sacked_out + tp.lost_out;
    }
#[no_mangle]
unsafe extern "C" fn tcp_packets_in_flight(tp: *const tcp_sock) -> c_uint {
    static unsigned int tcp_packets_in_flight(const struct tcp_sock *tp)
    {
    return tp.packets_out - tcp_left_out(tp) + tp.retrans_out;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: write_sk_pacing_init, sk: *mut sock) {
    void BPF_PROG(write_sk_pacing_init, struct sock *sk)
    {

    __sync_bool_compare_and_swap(&sk.sk_pacing_status, SK_PACING_NONE,
    SK_PACING_NEEDED);

    sk.sk_pacing_status = SK_PACING_NEEDED;

    }
    SEC("struct_ops")
    void BPF_PROG(write_sk_pacing_cong_control, struct sock *sk,
    const struct rate_sample *rs)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    unsigned long rate =
    ((tp.snd_cwnd * tp.mss_cache * USEC_PER_SEC) << 3) /
    (tp.srtt_us ?: 1U << 3);
    sk.sk_pacing_rate = min(rate, sk.sk_max_pacing_rate);
    tp.app_limited = (tp.delivered + tcp_packets_in_flight(tp)) ?: 1;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: write_sk_pacing_ssthresh, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(write_sk_pacing_ssthresh, struct sock *sk)
    {
    return tcp_sk(sk).snd_ssthresh;
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: write_sk_pacing_undo_cwnd, sk: *mut sock) -> __u32 {
    __u32 BPF_PROG(write_sk_pacing_undo_cwnd, struct sock *sk)
    {
    return tcp_sk(sk).snd_cwnd;
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops write_sk_pacing = {
    .init = (void *)write_sk_pacing_init,
    .cong_control = (void *)write_sk_pacing_cong_control,
    .ssthresh = (void *)write_sk_pacing_ssthresh,
    .undo_cwnd = (void *)write_sk_pacing_undo_cwnd,
    .name = "bpf_w_sk_pacing",
    };
