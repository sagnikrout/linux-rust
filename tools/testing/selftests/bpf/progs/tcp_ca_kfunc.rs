//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tcp_ca_kfunc.c
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
// Copyright (c) 2024 Facebook

    extern void bbr_init(struct sock *sk) __ksym;
    extern void bbr_main(struct sock *sk, u32 ack, int flag, const struct rate_sample *rs) __ksym;
    extern u32 bbr_sndbuf_expand(struct sock *sk) __ksym;
    extern u32 bbr_undo_cwnd(struct sock *sk) __ksym;
    extern void bbr_cwnd_event_tx_start(struct sock *sk) __ksym;
    extern u32 bbr_ssthresh(struct sock *sk) __ksym;
    extern u32 bbr_min_tso_segs(struct sock *sk) __ksym;
    extern void bbr_set_state(struct sock *sk, u8 new_state) __ksym;
    extern void dctcp_init(struct sock *sk) __ksym;
    extern void dctcp_update_alpha(struct sock *sk, u32 flags) __ksym;
    extern void dctcp_cwnd_event(struct sock *sk, enum tcp_ca_event ev) __ksym;
    extern void dctcp_cwnd_event_tx_start(struct sock *sk) __ksym;
    extern u32 dctcp_ssthresh(struct sock *sk) __ksym;
    extern u32 dctcp_cwnd_undo(struct sock *sk) __ksym;
    extern void dctcp_state(struct sock *sk, u8 new_state) __ksym;
    extern void cubictcp_init(struct sock *sk) __ksym;
    extern u32 cubictcp_recalc_ssthresh(struct sock *sk) __ksym;
    extern void cubictcp_cong_avoid(struct sock *sk, u32 ack, u32 acked) __ksym;
    extern void cubictcp_state(struct sock *sk, u8 new_state) __ksym;
    extern void cubictcp_cwnd_event_tx_start(struct sock *sk) __ksym;
    extern void cubictcp_acked(struct sock *sk, const struct ack_sample *sample) __ksym;
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: init, sk: *mut sock) {
    void BPF_PROG(init, struct sock *sk)
    {
    bbr_init(sk);
    dctcp_init(sk);
    cubictcp_init(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: in_ack_event, sk: *mut sock, flags: u32) {
    void BPF_PROG(in_ack_event, struct sock *sk, u32 flags)
    {
    dctcp_update_alpha(sk, flags);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cong_control, sk: *mut sock, ack: u32, flag: c_int, rs: *const rate_sample) {
    void BPF_PROG(cong_control, struct sock *sk, u32 ack, int flag, const struct rate_sample *rs)
    {
    bbr_main(sk, ack, flag, rs);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cong_avoid, sk: *mut sock, ack: u32, acked: u32) {
    void BPF_PROG(cong_avoid, struct sock *sk, u32 ack, u32 acked)
    {
    cubictcp_cong_avoid(sk, ack, acked);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sndbuf_expand, sk: *mut sock) -> u32 {
    u32 BPF_PROG(sndbuf_expand, struct sock *sk)
    {
    return bbr_sndbuf_expand(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: undo_cwnd, sk: *mut sock) -> u32 {
    u32 BPF_PROG(undo_cwnd, struct sock *sk)
    {
    bbr_undo_cwnd(sk);
    return dctcp_cwnd_undo(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cwnd_event, sk: *mut sock, event: enum tcp_ca_event) {
    void BPF_PROG(cwnd_event, struct sock *sk, enum tcp_ca_event event)
    {
    dctcp_cwnd_event(sk, event);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: cwnd_event_tx_start, sk: *mut sock) {
    void BPF_PROG(cwnd_event_tx_start, struct sock *sk)
    {
    bbr_cwnd_event_tx_start(sk);
    dctcp_cwnd_event_tx_start(sk);
    cubictcp_cwnd_event_tx_start(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ssthresh, sk: *mut sock) -> u32 {
    u32 BPF_PROG(ssthresh, struct sock *sk)
    {
    bbr_ssthresh(sk);
    dctcp_ssthresh(sk);
    return cubictcp_recalc_ssthresh(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: min_tso_segs, sk: *mut sock) -> u32 {
    u32 BPF_PROG(min_tso_segs, struct sock *sk)
    {
    return bbr_min_tso_segs(sk);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: set_state, sk: *mut sock, new_state: u8) {
    void BPF_PROG(set_state, struct sock *sk, u8 new_state)
    {
    bbr_set_state(sk, new_state);
    dctcp_state(sk, new_state);
    cubictcp_state(sk, new_state);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: pkts_acked, sk: *mut sock, sample: *const ack_sample) {
    void BPF_PROG(pkts_acked, struct sock *sk, const struct ack_sample *sample)
    {
    cubictcp_acked(sk, sample);
    }
    SEC(".struct_ops")
    struct tcp_congestion_ops tcp_ca_kfunc = {
    .init		= (void *)init,
    .in_ack_event	= (void *)in_ack_event,
    .cong_control	= (void *)cong_control,
    .cong_avoid	= (void *)cong_avoid,
    .sndbuf_expand	= (void *)sndbuf_expand,
    .undo_cwnd	= (void *)undo_cwnd,
    .cwnd_event	= (void *)cwnd_event,
    .cwnd_event_tx_start = (void *)cwnd_event_tx_start,
    .ssthresh	= (void *)ssthresh,
    .min_tso_segs	= (void *)min_tso_segs,
    .set_state	= (void *)set_state,
    .pkts_acked     = (void *)pkts_acked,
    .name		= "tcp_ca_kfunc",
    };
    char _license[] SEC("license") = "GPL";
